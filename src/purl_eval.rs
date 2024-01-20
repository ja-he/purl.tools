use crate::purl_data::{self, PurlComponent, PurlType, PurlTypeStatus};

lazy_static! {
    pub static ref TYPE_REGEX: regex::Regex =
        regex::Regex::new(r"^[a-zA-Z\.\+\-][a-zA-Z0-9\.\+\-]*$").unwrap();

    // "may only contain alphanumeric characters or single hyphens, and cannot begin or end with a hyphen"
    pub static ref GITHUB_USERNAME_REGEX: regex::Regex =
        regex::Regex::new(r"^[a-zA-Z0-9]+(-?[a-zA-Z0-9]+)*$").unwrap();
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
    pub fn aggregate(results: &Vec<EvalResult>) -> EvalResult {
        results
            .iter()
            .cloned()
            .reduce(|accumulator, elem| accumulator.combine(&elem))
            .unwrap_or(EvalResult::ProbablyOk(
                "no specific issues to point out".to_string(),
            ))
            .clone()
    }
    pub fn more_severe_than(&self, other: &EvalResult) -> bool {
        match (self, other) {
            (EvalResult::Verified(_), _) => false,
            (EvalResult::ProbablyOk(_), EvalResult::Verified(_)) => true,
            (EvalResult::ProbablyOk(_), _) => false,
            (EvalResult::AtLeastValid(_), EvalResult::Verified(_)) => true,
            (EvalResult::AtLeastValid(_), EvalResult::ProbablyOk(_)) => true,
            (EvalResult::AtLeastValid(_), _) => false,
            (EvalResult::Invalid(_), EvalResult::Invalid(_)) => false,
            (EvalResult::Invalid(_), _) => true,
        }
    }
    pub fn combine(&self, other: &EvalResult) -> EvalResult {
        match (self, other) {
            (EvalResult::Verified(s), EvalResult::Verified(o)) => {
                EvalResult::Verified(s.clone() + o)
            }
            (EvalResult::ProbablyOk(s), EvalResult::ProbablyOk(o)) => {
                EvalResult::ProbablyOk(s.clone() + o)
            }
            (EvalResult::AtLeastValid(s), EvalResult::AtLeastValid(o)) => {
                EvalResult::AtLeastValid(s.clone() + o)
            }
            (EvalResult::Invalid(s), EvalResult::Invalid(o)) => EvalResult::Invalid(s.clone() + o),
            _ => {
                if self.more_severe_than(other) {
                    self.clone()
                } else {
                    other.clone()
                }
            }
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
    let mut findings = vec![];

    let canonical = purl_namespace.as_canonical();
    if canonical.iter().any(String::is_empty) {
        findings.push(EvalResult::Invalid(
            "contains empty (inner) segments".to_string(),
        ));
    }
    if purl_namespace.len() != canonical.len() {
        findings.push(EvalResult::AtLeastValid("had to canonicalize".to_string()));
    }

    // TODO: regex check

    match typex {
        PurlType::Github => {
            if canonical.iter().any(|s| !GITHUB_USERNAME_REGEX.is_match(s)) {
                findings.push( EvalResult::Invalid("GitHub Namespace does not satisfy restrictions 'may only contain alphanumeric characters or single hyphens, and cannot begin or end with a hyphen'".to_string()));
            }

            if canonical.len() != 1 {
                findings.push(EvalResult::Invalid(
                    "namespace for GitHub should have one element only".to_string(),
                ));
            }

            findings.push(EvalResult::ProbablyOk("namespace looks good for GitHub type, but I did not verify for existence with GitHub".to_string()));
        }
        PurlType::Cargo => {
            if !canonical.is_empty() {
                findings.push(EvalResult::Invalid(
                    "namespace for Cargo (crates.io) should be empty".to_string(),
                ));
            }
        }
        _ => {
            findings.push(EvalResult::ProbablyOk(
                "namespace seems good, but I did not have type-specific checks to run for it"
                    .to_string(),
            ));
        }
    }

    EvalResult::aggregate(&findings)
}
    }

}
