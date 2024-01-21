use crate::purl_data::{self, PurlComponent, PurlType, PurlTypeStatus};

lazy_static! {
    pub static ref TYPE_REGEX: regex::Regex =
        regex::Regex::new(r"^[a-zA-Z\.\+\-][a-zA-Z0-9\.\+\-]*$").unwrap();

    // "may only contain alphanumeric characters or single hyphens, and cannot begin or end with a hyphen"
    pub static ref GITHUB_USERNAME_REGEX: regex::Regex =
        regex::Regex::new(r"^[a-zA-Z0-9]+(-?[a-zA-Z0-9]+)*$").unwrap();

    pub static ref GITHUB_REPO_NAME_REGEX: regex::Regex =
        regex::Regex::new(r"^[a-zA-Z\._\-][a-zA-Z0-9\._\-]*$").unwrap();
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

pub fn eval_purl_name(
    purl_name: String,
    purl_namespace: purl_data::PurlNamespace,
    typex: purl_data::PurlType,
) -> EvalResult {
    // sanity check: ensure valid percent-encoding
    if let Err(e) = urlencoding::decode(&purl_name) {
        return EvalResult::Invalid(format!(
            "could not decode, so it must not be a valid percent-encoded string ({e})"
        ));
    }

    let canonical_namespace = purl_namespace.as_canonical();
    let mut findings = vec![];

    match typex {
        PurlType::Github => {
            findings.push(if GITHUB_REPO_NAME_REGEX.is_match(&purl_name) {
                EvalResult::ProbablyOk("name is valid for a GitHub repo".to_string())
            } else {
                EvalResult::AtLeastValid("name is not valid as a GitHub repo name".to_string())
            });
        }
        _ => findings.push(EvalResult::ProbablyOk(
            "do not have any type-specific name checks to perform".to_string(),
        )),
    }

    EvalResult::aggregate(&findings)
}

#[cfg(test)]
mod tests {
    use crate::purl_data::{PurlComponent, PurlNamespace, PurlType};

    use super::eval_purl_namespace;

    use paste::paste;

    // credit for this approach to table-driven tests: burntsushi
    macro_rules! test_eval_ns {
        ($name:ident, $t:expr, $ns:expr, $expect:expr) => {
            paste! {
            #[test]
            fn [<test_eval_ns_ $name>]() {
                let typex = $t;
                let namespace = $ns;
                let expected_summary = $expect;
                let result = eval_purl_namespace(PurlNamespace::new_naive(namespace), PurlType::new(typex));
                result.summary().ne(expected_summary).then(|| {
                    panic!("for type '{typex}' and ns '{namespace}' expected '{expected_summary}' but got '{actual}'", actual=result.summary())
                });
            }
            }
        }
    }

    test_eval_ns!(gh_normal, "github", "ja-he", "ok");
    test_eval_ns!(gh_empty, "github", "", "invalid"); // empty for github
    test_eval_ns!(gh_trailslash, "github", "ja-he/", "valid");
    test_eval_ns!(gh_trailslash_many, "github", "ja-he////", "valid");
    test_eval_ns!(gh_leadslash_many, "github", "////ja-he", "valid");
    test_eval_ns!(gh_lead_and_trailslash, "github", "////ja-he//", "valid");
    test_eval_ns!(gh_two_parts, "github", "ja/he", "invalid"); // more than 1 for github
    test_eval_ns!(github_underscore, "github", "ja_he", "invalid"); // github does not allow underscores
    test_eval_ns!(github_trailing_hyphen, "github", "jahe-", "invalid");
    test_eval_ns!(github_leading_hyphen, "github", "-jahe", "invalid");
}
