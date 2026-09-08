# migrate-vocabulary.ps1 ??? D-15 migration (Martin's 2026-09-01 ruling):
# [DIRECTIONAL] -> [REPORTED]; [NEEDS-CAVEAT] -> source-earned state,
# caveat preserved verbatim. ASCII-only script (PS5.1 reads BOM-less
# files as ANSI); em dash built as $em. Fails loudly if any custom
# "before" string is not found exactly once.
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$em = [string][char]0x2014
$utf8 = New-Object System.Text.UTF8Encoding($false)

function Edit-File {
    param([string]$Path, [array]$Customs, [bool]$Generics = $true)
    $t = [System.IO.File]::ReadAllText($Path, $utf8)
    foreach ($pair in $Customs) {
        $before = $pair[0]; $after = $pair[1]
        $count = ([regex]::Matches($t, [regex]::Escape($before))).Count
        if ($count -ne 1) {
            $afterCount = ([regex]::Matches($t, [regex]::Escape($after))).Count
            if ($count -eq 0 -and $afterCount -ge 1) { Write-Output "  already applied, skipping: $($before.Substring(0, [Math]::Min(40, $before.Length)))..."; continue }
            throw "CUSTOM MATCH FAILURE in ${Path}: expected 1, found $count -- $before"
        }
        $t = $t.Replace($before, $after)
    }
    if ($Generics) {
        $t = $t.Replace('[DIRECTIONAL ', '[REPORTED ')
        $t = $t.Replace('[NEEDS-CAVEAT ', '[UNVERIFIED ')
        $t = $t.Replace('[DIRECTIONAL]', '[REPORTED]')
        $t = $t.Replace('[NEEDS-CAVEAT]', '[UNVERIFIED]')
    }
    [System.IO.File]::WriteAllText($Path, $t, $utf8)
    Write-Output "migrated: $Path ($($Customs.Count) custom + generics=$Generics)"
}

$root = 'C:\Users\User\dev\holmes-clone'
Set-Location $root

# --- docs/holmes-spec-v2.md ---
Edit-File 'docs\holmes-spec-v2.md' @(
    @("> - **[DIRECTIONAL]** $em supported only by secondary/third-party sources, or a vendor figure not independently confirmed. Treat as a planning estimate; re-verify before relying.`r`n> - **[NEEDS-CAVEAT]** $em the *concept* holds but an exact quote/number/detail could not be confirmed from a primary source within budget. Confirm against the cited primary source before quoting or hard-coding.",
      "> - **The confidence vocabulary is the five system states (D-15, ruled 2026-09-01):** ``EXECUTED`` -- the check ran, output in hand; ``VERIFIED-LIVE`` -- observed live against the primary source this session; ``CANON`` -- a checked-in repo statement; ``REPORTED`` -- someone else's evidence (secondary/third-party source, or a vendor figure not independently confirmed -- treat as a planning estimate, re-verify before relying); ``UNVERIFIED`` -- not checked or not confirmable within budget (the concept may hold; confirm against the cited primary source before quoting or hard-coding). Unmarked claims stand as verified against primary sources as of the verification date. Never silently harden a REPORTED/UNVERIFIED claim into a fact."),
    @('all Blacksky `[NEEDS-CAVEAT]` items resolved', 'all Blacksky caveated items resolved'),
    @('downgraded to [NEEDS-CAVEAT]', 'downgraded to [REPORTED]'),
    @('is now **[NEEDS-CAVEAT]**: secondary sources conflict', 'is now **[REPORTED]**: secondary sources conflict'),
    @("[NEEDS-CAVEAT $em permanence", "[REPORTED $em permanence"),
    @('permanence is [NEEDS-CAVEAT]', 'permanence is [REPORTED]')
)

# --- docs/triad-canon.md ---
Edit-File 'docs\triad-canon.md' @(, @(
    @("Carry confidence markers $em **[DIRECTIONAL]** for estimates/secondary sourcing, **[NEEDS-CAVEAT]** for unconfirmed details $em",
      "Carry confidence labels in the five system states (D-15, 2026-09-01: EXECUTED / VERIFIED-LIVE / CANON / REPORTED / UNVERIFIED) $em **[REPORTED]** for estimates/secondary sourcing, **[UNVERIFIED]** for unconfirmed details $em")
))
# --- docs/security.md --- (generics only)

# --- docs/architecture.md ---
Edit-File 'docs\architecture.md' @(
    @("Markers (``[DIRECTIONAL]``, ``[NEEDS-CAVEAT]``) are carried verbatim $em never harden them.",
      "Claim labels are the five system states (D-15): EXECUTED / VERIFIED-LIVE / CANON / REPORTED / UNVERIFIED; caveated labels are carried verbatim $em never harden them."),
    @('permanence is `[NEEDS-CAVEAT]`', 'permanence is `[REPORTED]`')
)

# --- docs/holmes-project-orientation.md ---
Edit-File 'docs\holmes-project-orientation.md' @(
    @("Preserve ``[DIRECTIONAL]`` / ``[NEEDS-CAVEAT]`` markers $em don't silently harden caveated claims into facts.",
      "Preserve the five-state confidence labels (D-15: EXECUTED / VERIFIED-LIVE / CANON / REPORTED / UNVERIFIED) $em don't silently harden caveated claims into facts."),
    @("Blacksky ``[NEEDS-CAVEAT]`` items $em RESOLVED", "Blacksky caveated items $em RESOLVED")
)

# --- docs/holmes-vs-wcjbt.md ---
Edit-File 'docs\holmes-vs-wcjbt.md' @(
    @('**[DIRECTIONAL]** marks design intent or figures not externally verifiable; **[NEEDS-CAVEAT]** marks a claim with a known caveat or unresolved conflict.',
      '**[REPORTED]** marks design intent or figures resting on someone else''s evidence; **[UNVERIFIED]** marks a claim with a known caveat or unresolved conflict. (Five system states per D-15, 2026-09-01.)'),
    @('specifics. [NEEDS-CAVEAT]**', 'specifics. [REPORTED]**')
)

# --- docs/acceptance/holmes-denylist-acceptance-criteria.md --- (generics only)

# --- docs/holmes-launch-runbook-v1.md ---
Edit-File 'docs\holmes-launch-runbook-v1.md' @(, @(
    @('**[DIRECTIONAL]** = secondary/asserted; **[NEEDS-CAVEAT]** = concept holds, detail unconfirmed.',
      '**[REPORTED]** = secondary/asserted; **[UNVERIFIED]** = concept holds, detail unconfirmed. (Five system states per D-15.)')
))
# --- docs/prompts/holmes-master-build-loop-v2.md ---
Edit-File 'docs\prompts\holmes-master-build-loop-v2.md' @(, @(
    @('Spec-derived docs preserve the canon markers verbatim:', 'Spec-derived docs carry the five state labels verbatim (D-15):')
))
# --- docs/holmes-claude-project-instructions.md ---
Edit-File 'docs\holmes-claude-project-instructions.md' @(, @(
    @('\[DIRECTIONAL\] \= secondary/estimate; \[NEEDS-CAVEAT\] \= concept holds, exact detail unconfirmed.',
      '\[REPORTED\] \= secondary/estimate; \[UNVERIFIED\] \= concept holds, exact detail unconfirmed (five system states, D-15).')
))
# --- CLAUDE.md ---
Edit-File 'CLAUDE.md' @(
    @("Preserve ``[DIRECTIONAL]`` and ``[NEEDS-CAVEAT]`` markers $em never silently harden caveated claims into facts.",
      "Claim labels are the five system states (D-15, 2026-09-01): EXECUTED / VERIFIED-LIVE / CANON / REPORTED / UNVERIFIED $em never silently harden caveated claims into facts."),
    @('permanence `[NEEDS-CAVEAT]`', 'permanence `[REPORTED]`')
)

# --- generics-only files ---
Edit-File 'docs\security.md' @()
Edit-File 'docs\acceptance\holmes-denylist-acceptance-criteria.md' @()

Write-Output '--- residual census (must be empty) ---'
$hits = Select-String -Path 'docs\holmes-spec-v2.md','docs\triad-canon.md','docs\security.md','docs\architecture.md','docs\holmes-project-orientation.md','docs\holmes-vs-wcjbt.md','docs\acceptance\holmes-denylist-acceptance-criteria.md','docs\holmes-launch-runbook-v1.md','docs\prompts\holmes-master-build-loop-v2.md','docs\holmes-claude-project-instructions.md','CLAUDE.md' -Pattern '\[DIRECTIONAL','\[NEEDS-CAVEAT'
if ($hits) { $hits | ForEach-Object { Write-Output ("RESIDUAL: {0}:{1}" -f $_.Path, $_.LineNumber) }; throw 'RESIDUAL MARKERS' }
Write-Output 'residual census: CLEAN'
