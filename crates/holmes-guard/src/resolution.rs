//! L1b — provider/model-id resolution guard (AC-DL-1 §2).
//!
//! Deny-by-default: permitted ids pass; excluded ids *and unknown ids* are
//! rejected at resolution time, before any client is instantiated. Absence
//! from the permitted set is rejection, not a warning.

use crate::policy;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Denial {
    ExcludedProvider(String),
    ExcludedModelFamily(String),
    UnknownProvider(String),
    UnknownModel { provider: String, model: String },
}

impl fmt::Display for Denial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Denial::ExcludedProvider(p) => {
                write!(f, "denied: provider '{p}' is excluded by the compiled denylist")
            }
            Denial::ExcludedModelFamily(m) => {
                write!(f, "denied: model id '{m}' belongs to an excluded model family")
            }
            Denial::UnknownProvider(p) => {
                write!(f, "denied: provider '{p}' is not in the permitted set (deny-by-default)")
            }
            Denial::UnknownModel { provider, model } => write!(
                f,
                "denied: model id '{model}' is not a permitted family for provider '{provider}' (deny-by-default)"
            ),
        }
    }
}

impl std::error::Error for Denial {}

/// A provider/model pair that passed resolution, normalized.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedModel {
    pub provider: String,
    pub model: String,
}

/// Resolve a requested provider/model pair against the compiled policy.
/// Every failure path is a denial; there is no warning state.
pub fn resolve(provider: &str, model: &str) -> Result<ResolvedModel, Denial> {
    let provider = provider.trim().to_ascii_lowercase();
    let model = model.trim().to_ascii_lowercase();

    if policy::provider_excluded(&provider) {
        return Err(Denial::ExcludedProvider(provider));
    }
    if policy::model_family_excluded(&model) {
        return Err(Denial::ExcludedModelFamily(model));
    }
    if provider.is_empty() || !policy::provider_permitted(&provider) {
        return Err(Denial::UnknownProvider(provider));
    }
    if model.is_empty() || !policy::model_permitted_for_provider(&provider, &model) {
        return Err(Denial::UnknownModel { provider, model });
    }
    Ok(ResolvedModel { provider, model })
}
