use crate::purl_data::{self, PurlComponent, PurlType, PurlTypeStatus};

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

pub fn eval_purl_type(purl_type: PurlType) -> EvalResult {
    match purl_type.status() {
        PurlTypeStatus::WellKnown => EvalResult::Verified("well-known identifier".to_string()),
        PurlTypeStatus::Proposed => {
            EvalResult::ProbablyOk("officially proposed identifier".to_string())
        }
        PurlTypeStatus::Other => {
            let purl_type_str = purl_type.to_string();
            if purl_type_str.is_empty() {
                EvalResult::Invalid("type must not be empty".to_string())
            } else if TYPE_REGEX.is_match(&purl_type_str) {
                EvalResult::AtLeastValid("valid identifier".to_string())
            } else {
                EvalResult::Invalid("does not match regex".to_string())
            }
        }
    }
}

pub fn eval_purl_namespace(
    purl_namespace: purl_data::PurlNamespace,
    typex: purl_data::PurlType,
) -> EvalResult {
    let canonical = purl_namespace.as_canonical();
    if canonical.iter().any(String::is_empty) {
        return EvalResult::Invalid("contains empty (inner) segments".to_string());
    }

    // TODO: regex check

    if typex == PurlType::Github && purl_namespace.len() != 1 {
        if canonical.len() == 1 {
            return EvalResult::AtLeastValid("had to canonicalize".to_string());
        } else {
            return EvalResult::Invalid(
                "namespace for GitHub should have one element only".to_string(),
            );
        }
    }

    if typex == PurlType::Cargo && !purl_namespace.is_empty() {
        if canonical.is_empty() {
            return EvalResult::AtLeastValid("had to canonicalize".to_string());
        } else {
            return EvalResult::Invalid(
                "namespace for Cargo (crates.io) should be empty".to_string(),
            );
        }
    }

    if purl_namespace.len() != canonical.len() {
        return EvalResult::AtLeastValid("had to canonicalize".to_string());
    }

    EvalResult::ProbablyOk(
        "namespace seems good, but I did not have type-specific checks to run for it".to_string(),
    )
}
