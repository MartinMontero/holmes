//! Lock 2.5c — the tool-approval protocol (loop §6 Phase 2.5(iii):
//! "structured, previewable approval requests … for every gated action,
//! blocking until answered; rendering is Alfred's surface (obligation
//! recorded), the protocol and blocking behavior are Holmes's, testable
//! headlessly").
//!
//! Deny-by-default, per case: no tool fires without a [`ToolGrant`], and
//! the only mint of a grant is an operator **Approved** decision on a
//! previously previewed request. "Blocking until answered" is a property
//! of this state machine, not of a thread: before a decision is
//! recorded, [`ApprovalProtocol::authorize`] refuses every tool, so the
//! caller has nothing to fire with. An unanswered request blocks
//! forever; there is no timeout-to-approve path, only timeout-to-deny at
//! the caller's discretion (denial is always safe).
//!
//! The approval log is **born-redacted** (constitution #9): tool names,
//! decisions, counts, and timestamps — never arguments, previews of
//! content, or extraction text. `ToolGrant` is sealed (private field):
//! nothing converts text — hostile or otherwise — into a grant, which is
//! half of the lock-2.5a "extractions carry no authority" claim.

use std::fmt;

/// Bounds on operator-facing tool metadata: a name that cannot smuggle
/// (lowercase alphanumerics plus `._-`, capped) and a purpose line the
/// preview shows.
pub const MAX_TOOL_NAME_CHARS: usize = 64;
pub const MAX_TOOL_PURPOSE_CHARS: usize = 256;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApprovalError {
    /// Deny-by-default: no grant exists for this tool in this case.
    NotGranted {
        tool: String,
    },
    UnknownRequest(RequestId),
    /// A settled request cannot be re-decided (no approve-after-deny).
    AlreadyDecided(RequestId),
    EmptyToolSet,
    /// Name outside the bounded vocabulary (charset/length).
    InvalidToolName {
        name_chars: usize,
    },
    InvalidToolPurpose {
        purpose_chars: usize,
    },
}

impl fmt::Display for ApprovalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApprovalError::NotGranted { tool } => write!(
                f,
                "refused: no operator grant for tool '{tool}' in this case (deny-by-default; \
                 request approval first)"
            ),
            ApprovalError::UnknownRequest(id) => write!(f, "refused: unknown request {}", id.0),
            ApprovalError::AlreadyDecided(id) => {
                write!(f, "refused: request {} already decided", id.0)
            }
            ApprovalError::EmptyToolSet => {
                write!(f, "refused: an approval request needs at least one tool")
            }
            ApprovalError::InvalidToolName { name_chars } => write!(
                f,
                "refused: tool name invalid ({name_chars} chars; bounded lowercase \
                 alphanumerics plus ._- up to {MAX_TOOL_NAME_CHARS})"
            ),
            ApprovalError::InvalidToolPurpose { purpose_chars } => write!(
                f,
                "refused: tool purpose invalid ({purpose_chars} chars; non-empty up to \
                 {MAX_TOOL_PURPOSE_CHARS})"
            ),
        }
    }
}

impl std::error::Error for ApprovalError {}

/// One tool as shown to the operator: name + purpose, both bounded.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolDescriptor {
    name: String,
    purpose: String,
}

impl ToolDescriptor {
    pub fn new(name: impl Into<String>, purpose: impl Into<String>) -> Result<Self, ApprovalError> {
        let name = name.into();
        let name_chars = name.chars().count();
        if name.is_empty()
            || name_chars > MAX_TOOL_NAME_CHARS
            || !name.chars().all(|c| {
                c.is_ascii_lowercase() || c.is_ascii_digit() || matches!(c, '.' | '_' | '-')
            })
        {
            return Err(ApprovalError::InvalidToolName { name_chars });
        }
        let purpose = purpose.into();
        let purpose_chars = purpose.chars().count();
        if purpose.trim().is_empty() || purpose_chars > MAX_TOOL_PURPOSE_CHARS {
            return Err(ApprovalError::InvalidToolPurpose { purpose_chars });
        }
        Ok(Self { name, purpose })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn purpose(&self) -> &str {
        &self.purpose
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RequestId(pub usize);

/// The operator's answer. There is no "partial" — a request is approved
/// or denied as the previewed set; narrowing means a new request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApprovalDecision {
    Approved,
    Denied,
}

/// A structured, previewable approval request (the ACP payload shape;
/// rendering is Alfred's obligation, recorded in `docs/security.md`).
#[derive(Debug, Clone)]
pub struct ApprovalRequest {
    pub id: RequestId,
    pub case_id: String,
    pub tools: Vec<ToolDescriptor>,
    pub requested_at: String,
    decision: Option<ApprovalDecision>,
}

impl ApprovalRequest {
    /// The deterministic preview the operator sees before deciding:
    /// every tool, name and purpose, nothing hidden, nothing else.
    pub fn preview(&self) -> String {
        let mut out = format!(
            "case {} requests {} tool(s):\n",
            self.case_id,
            self.tools.len()
        );
        for t in &self.tools {
            out.push_str(&format!("  - {} : {}\n", t.name, t.purpose));
        }
        out.push_str("approve to grant exactly this set; deny to grant nothing.");
        out
    }

    pub fn decision(&self) -> Option<ApprovalDecision> {
        self.decision
    }
}

/// A granted capability: case-scoped, tool-named, sealed. The private
/// field means the only mint is [`ApprovalProtocol::record_decision`]
/// on an Approved request — no path from any text to a grant exists.
#[derive(Debug, Clone)]
pub struct ToolGrant {
    case_id: String,
    tool: String,
    _minted_by_operator_decision: (),
}

impl ToolGrant {
    pub fn case_id(&self) -> &str {
        &self.case_id
    }

    pub fn tool(&self) -> &str {
        &self.tool
    }
}

/// One born-redacted log entry: names, decision, timestamp — no
/// arguments, no content, no previews of extractions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedactedApprovalEntry {
    pub case_id: String,
    pub tool: String,
    pub decision: ApprovalDecision,
    pub at: String,
}

/// The per-case approval state machine.
pub struct ApprovalProtocol {
    case_id: String,
    requests: Vec<ApprovalRequest>,
    grants: Vec<ToolGrant>,
    log: Vec<RedactedApprovalEntry>,
}

impl ApprovalProtocol {
    pub fn new(case_id: impl Into<String>) -> Self {
        Self {
            case_id: case_id.into(),
            requests: Vec::new(),
            grants: Vec::new(),
            log: Vec::new(),
        }
    }

    /// Stage a request for the operator. Nothing is granted here; the
    /// request exists so it can be previewed and answered.
    pub fn request(
        &mut self,
        tools: Vec<ToolDescriptor>,
        requested_at: impl Into<String>,
    ) -> Result<RequestId, ApprovalError> {
        if tools.is_empty() {
            return Err(ApprovalError::EmptyToolSet);
        }
        let id = RequestId(self.requests.len());
        self.requests.push(ApprovalRequest {
            id,
            case_id: self.case_id.clone(),
            tools,
            requested_at: requested_at.into(),
            decision: None,
        });
        Ok(id)
    }

    pub fn requests(&self) -> &[ApprovalRequest] {
        &self.requests
    }

    /// Record the operator's decision. Approved mints one grant per tool
    /// in the previewed set; Denied mints nothing. Either way the
    /// decision is logged born-redacted, one entry per tool.
    pub fn record_decision(
        &mut self,
        id: RequestId,
        decision: ApprovalDecision,
        decided_at: impl Into<String>,
    ) -> Result<usize, ApprovalError> {
        let at = decided_at.into();
        let request = self
            .requests
            .get_mut(id.0)
            .ok_or(ApprovalError::UnknownRequest(id))?;
        if request.decision.is_some() {
            return Err(ApprovalError::AlreadyDecided(id));
        }
        request.decision = Some(decision);
        let mut minted = 0;
        for tool in &request.tools {
            self.log.push(RedactedApprovalEntry {
                case_id: self.case_id.clone(),
                tool: tool.name.clone(),
                decision,
                at: at.clone(),
            });
            if decision == ApprovalDecision::Approved {
                self.grants.push(ToolGrant {
                    case_id: self.case_id.clone(),
                    tool: tool.name.clone(),
                    _minted_by_operator_decision: (),
                });
                minted += 1;
            }
        }
        Ok(minted)
    }

    /// The gate every tool invocation passes: deny-by-default. Returns
    /// the grant (proof of operator approval) or the refusal.
    pub fn authorize(&self, tool: &str) -> Result<&ToolGrant, ApprovalError> {
        self.grants
            .iter()
            .find(|g| g.tool == tool)
            .ok_or_else(|| ApprovalError::NotGranted {
                tool: tool.to_owned(),
            })
    }

    /// The born-redacted approval log (append-only).
    pub fn log(&self) -> &[RedactedApprovalEntry] {
        &self.log
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tools() -> Vec<ToolDescriptor> {
        vec![
            ToolDescriptor::new("registry.search", "query the corporate registry").unwrap(),
            ToolDescriptor::new("court-records.fetch", "fetch docket entries").unwrap(),
        ]
    }

    #[test]
    fn deny_by_default_blocks_until_answered() {
        let mut p = ApprovalProtocol::new("case-1");
        let id = p.request(tools(), "t0").unwrap();
        // Unanswered: everything refused — this IS the blocking behavior.
        assert!(matches!(
            p.authorize("registry.search").unwrap_err(),
            ApprovalError::NotGranted { .. }
        ));
        let minted = p
            .record_decision(id, ApprovalDecision::Approved, "t1")
            .unwrap();
        assert_eq!(minted, 2);
        let grant = p.authorize("registry.search").unwrap();
        assert_eq!(grant.case_id(), "case-1");
        // Never-requested tools stay refused even after an approval.
        assert!(p.authorize("shell.exec").is_err());
    }

    #[test]
    fn denied_requests_mint_nothing_and_cannot_be_flipped() {
        let mut p = ApprovalProtocol::new("case-2");
        let id = p.request(tools(), "t0").unwrap();
        assert_eq!(
            p.record_decision(id, ApprovalDecision::Denied, "t1")
                .unwrap(),
            0
        );
        assert!(p.authorize("registry.search").is_err());
        assert_eq!(
            p.record_decision(id, ApprovalDecision::Approved, "t2")
                .unwrap_err(),
            ApprovalError::AlreadyDecided(id)
        );
    }

    #[test]
    fn preview_shows_the_whole_set_and_log_is_born_redacted() {
        let mut p = ApprovalProtocol::new("case-3");
        let id = p.request(tools(), "2026-07-20T00:00:00Z").unwrap();
        let preview = p.requests()[id.0].preview();
        assert!(preview.contains("registry.search"));
        assert!(preview.contains("court-records.fetch"));
        assert!(preview.contains("2 tool(s)"));
        p.record_decision(id, ApprovalDecision::Approved, "2026-07-20T00:01:00Z")
            .unwrap();
        assert_eq!(p.log().len(), 2);
        for entry in p.log() {
            assert_eq!(entry.decision, ApprovalDecision::Approved);
            // Redaction: the entry type has exactly case/tool/decision/at.
            let rendered = format!("{entry:?}");
            assert!(!rendered.contains("purpose"), "log carries tool metadata");
        }
    }

    #[test]
    fn tool_names_are_bounded_and_cannot_smuggle() {
        assert!(ToolDescriptor::new("ok.tool-1", "p").is_ok());
        assert!(ToolDescriptor::new("", "p").is_err());
        assert!(ToolDescriptor::new("UPPER", "p").is_err());
        assert!(ToolDescriptor::new("sp ace", "p").is_err());
        assert!(ToolDescriptor::new("zero\u{200B}width", "p").is_err());
        assert!(ToolDescriptor::new("a".repeat(MAX_TOOL_NAME_CHARS + 1), "p").is_err());
        assert!(ToolDescriptor::new("t", "").is_err());
    }
}
