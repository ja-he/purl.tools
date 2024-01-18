use crate::purl_data::{PurlType, PurlTypeStatus};

lazy_static! {
    pub static ref TYPE_REGEX: regex::Regex =
        regex::Regex::new(r"^[a-zA-Z\.\+\-][a-zA-Z0-9\.\+\-]*$").unwrap();
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvalResult {
    Verified(String),
    ProbablyOk(String),
    AtLeastValid(String),
    Invalid(String),
}

impl EvalResult {
    pub fn summary(&self) -> String {
        match self {
            EvalResult::Verified(_) => "verified".to_string(),
            EvalResult::ProbablyOk(_) => "ok".to_string(),
            EvalResult::AtLeastValid(_) => "valid".to_string(),
            EvalResult::Invalid(_) => "invalid".to_string(),
        }
    }
    pub fn explanation(&self) -> String {
        match self {
            EvalResult::Verified(s)
            | EvalResult::ProbablyOk(s)
            | EvalResult::AtLeastValid(s)
            | EvalResult::Invalid(s) => s.clone(),
        }
    }
}

pub fn eval_purl_type(s: &str) -> EvalResult {
    match PurlType::new(s).status() {
        PurlTypeStatus::WellKnown => EvalResult::Verified("well-known identifier".to_string()),
        PurlTypeStatus::Proposed => {
            EvalResult::ProbablyOk("officially proposed identifier".to_string())
        }
        PurlTypeStatus::Other => {
            if s.is_empty() {
                EvalResult::Invalid("type must not be empty".to_string())
            } else if TYPE_REGEX.is_match(s) {
                EvalResult::AtLeastValid("valid identifier".to_string())
            } else {
                EvalResult::Invalid("does not match regex".to_string())
            }
        }
    }
}
