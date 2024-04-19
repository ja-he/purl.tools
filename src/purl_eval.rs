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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum EvalResultLevel {
    Verified,
    ProbablyOk,
    AtLeastValid,
    Invalid,
}

impl std::fmt::Display for EvalResultLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvalResultLevel::Verified => write!(f, "verified"),
            EvalResultLevel::ProbablyOk => write!(f, "ok"),
            EvalResultLevel::AtLeastValid => write!(f, "valid"),
            EvalResultLevel::Invalid => write!(f, "invalid"),
        }
    }
}

impl EvalResultLevel {
    pub fn more_severe_than(&self, other: &EvalResultLevel) -> bool {
        self > other
    }
    pub fn at_least_as_good_as(&self, other: &EvalResultLevel) -> bool {
        self <= other
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvalResult {
    pub level: EvalResultLevel,
    pub explanation: String,
}

impl EvalResult {
    pub fn aggregate(results: &[EvalResult]) -> EvalResult {
        results
            .iter()
            .cloned()
            .reduce(|accumulator, elem| accumulator.combine(&elem))
            .unwrap_or(EvalResult {
                level: EvalResultLevel::ProbablyOk,
                explanation: "no specific issues to point out".to_string(),
            })
            .clone()
    }
    pub fn combine(&self, other: &EvalResult) -> EvalResult {
        match (&self.level, &other.level) {
            (EvalResultLevel::Verified, EvalResultLevel::Verified) => EvalResult {
                level: EvalResultLevel::Verified,
                explanation: self.explanation.clone() + &other.explanation,
            },
            (EvalResultLevel::ProbablyOk, EvalResultLevel::ProbablyOk) => EvalResult {
                level: EvalResultLevel::ProbablyOk,
                explanation: self.explanation.clone() + &other.explanation,
            },
            (EvalResultLevel::AtLeastValid, EvalResultLevel::AtLeastValid) => EvalResult {
                level: EvalResultLevel::AtLeastValid,
                explanation: self.explanation.clone() + &other.explanation,
            },
            (EvalResultLevel::Invalid, EvalResultLevel::Invalid) => EvalResult {
                level: EvalResultLevel::Invalid,
                explanation: self.explanation.clone() + &other.explanation,
            },
            _ => {
                if self.level.more_severe_than(&other.level) {
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
        PurlTypeStatus::WellKnown => EvalResult {
            level: EvalResultLevel::Verified,
            explanation: "well-known identifier".to_string(),
        },
        PurlTypeStatus::Proposed => EvalResult {
            level: EvalResultLevel::ProbablyOk,
            explanation: "officially proposed identifier".to_string(),
        },
        PurlTypeStatus::Other => {
            let purl_type_str = purl_type.to_string();
            if purl_type_str.is_empty() {
                EvalResult {
                    level: EvalResultLevel::Invalid,
                    explanation: "type must not be empty".to_string(),
                }
            } else if TYPE_REGEX.is_match(&purl_type_str) {
                EvalResult {
                    level: EvalResultLevel::AtLeastValid,
                    explanation: "valid identifier".to_string(),
                }
            } else {
                EvalResult {
                    level: EvalResultLevel::Invalid,
                    explanation: "does not match regex".to_string(),
                }
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
        findings.push(EvalResult {
            level: EvalResultLevel::Invalid,
            explanation: "contains empty (inner) segments".to_string(),
        });
    }
    if purl_namespace.len() != canonical.len() {
        findings.push(EvalResult {
            level: EvalResultLevel::AtLeastValid,
            explanation: "had to canonicalize".to_string(),
        });
    }

    // TODO: regex check

    match typex {
        PurlType::Github => {
            if canonical.iter().any(|s| !GITHUB_USERNAME_REGEX.is_match(s)) {
                findings.push( EvalResult{level:EvalResultLevel::AtLeastValid,explanation:"GitHub Namespace does not satisfy restrictions 'may only contain alphanumeric characters or single hyphens, and cannot begin or end with a hyphen'".to_string(),});
            }

            if canonical.len() != 1 {
                findings.push(EvalResult {
                    level: EvalResultLevel::AtLeastValid,
                    explanation: "namespace for GitHub should have one element only".to_string(),
                });
            }

            findings.push(EvalResult {
                level: EvalResultLevel::ProbablyOk,
                explanation: "namespace looks good for GitHub type, but I did not verify for existence with GitHub"
                    .to_string(),
            });
        }
        PurlType::Cargo => {
            if !canonical.is_empty() {
                findings.push(EvalResult {
                    level: EvalResultLevel::AtLeastValid,
                    explanation: "namespace for Cargo (crates.io) should be empty".to_string(),
                });
            } else {
                findings.push(EvalResult {
                    level: EvalResultLevel::Verified,
                    explanation: "empty namespace for Cargo (crates.io) is correct".to_string(),
                });
            }
        }
        PurlType::Npm => {
            if !canonical.is_empty() {
                findings.push(EvalResult {
                    level: EvalResultLevel::AtLeastValid,
                    explanation: "namespace for NPM (npmjs.org) should be empty".to_string(),
                });
            } else {
                findings.push(EvalResult {
                    level: EvalResultLevel::Verified,
                    explanation: "empty namespace for NPM (npmjs.org) is correct".to_string(),
                });
            }
        }
        PurlType::Alpm => {
            // for alpm, the namespace should be the vendor, such as "arch", "arch32", "manjaro", ...
            // (case-insensitive / lowercased)

            if canonical.is_empty() {
                findings.push(EvalResult {
                    level: EvalResultLevel::AtLeastValid,
                    explanation:
                        "namespace for alpm should not be empty (it should instead be the vendor)"
                            .to_string(),
                });
            } else if canonical.len() > 1 {
                findings.push(EvalResult {
                    level: EvalResultLevel::AtLeastValid,
                    explanation: "namespace for alpm should have one element (the vendor, e.g. \"arch\" or \"manjaro\")".to_string(),
                });
            } else {
                match canonical[0].as_str() {
                    "arch" => {}
                    other => {
                        findings.push(EvalResult {
                            level: EvalResultLevel::AtLeastValid,
                            explanation: format!("namespace '{other}' not known / specifically considered in this verification (this does not mean it is wrong, but it cannot be checked here, currently)"),
                        });
                    }
                }
            }
        }
        _ => {
            findings.push(EvalResult {
                level: EvalResultLevel::ProbablyOk,
                explanation:
                    "namespace seems good, but I did not have type-specific checks to run for it"
                        .to_string(),
            });
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
        return EvalResult {
            level: EvalResultLevel::Invalid,
            explanation: format!(
                "could not decode, so it must not be a valid percent-encoded string ({e})"
            ),
        };
    }
    if purl_name.is_empty() {
        return EvalResult {
            level: EvalResultLevel::Invalid,
            explanation: "a name is required (meaning also that it may not be empty)".to_string(),
        };
    }

    let canonical_namespace = purl_namespace.as_canonical();
    let mut findings = vec![];

    match typex {
        PurlType::Github => {
            findings.push(if GITHUB_REPO_NAME_REGEX.is_match(&purl_name) {
                EvalResult {
                    level: EvalResultLevel::ProbablyOk,
                    explanation: "name is valid for a GitHub repo".to_string(),
                }
            } else {
                EvalResult {
                    level: EvalResultLevel::AtLeastValid,
                    explanation: "name is not valid as a GitHub repo name".to_string(),
                }
            });
        }
        _ => findings.push(EvalResult {
            level: EvalResultLevel::ProbablyOk,
            explanation: "do not have any type-specific name checks to perform".to_string(),
        }),
    }

    EvalResult::aggregate(&findings)
}

// TODO
pub fn eval_purl_version(
    _typex: purl_data::PurlType,
    _purl_namespace: purl_data::PurlNamespace,
    _purl_name: String,
    version: Option<String>,
) -> EvalResult {
    match version {
        None => EvalResult {
            level: EvalResultLevel::ProbablyOk,
            explanation: "nothing to check on version".to_string(),
        },
        Some(s) => {
            match urlencoding::decode(&s) {
                Err(e) => EvalResult {
                    level: EvalResultLevel::Invalid,
                    explanation: format!(
                        "could not decode, so it must not be a valid percent-encoded string ({e})"
                    ),
                },
                Ok(decoded) => {
                    let decoded = decoded.into_owned();
                    let mut findings = vec![];
                    // TODO
                    EvalResult::aggregate(&findings)
                }
            }
        }
    }
}

pub fn eval_purl_qualifiers(qualifiers: Option<String>) -> EvalResult {
    match qualifiers {
        None => EvalResult {
            level: EvalResultLevel::ProbablyOk,
            explanation: "nothing to check on qualifiers".to_string(),
        },
        Some(s) => {
            let mut findings = vec![];
            EvalResult::aggregate(&findings)
        }
    }
}

pub fn eval_purl_subpath(subpath: Option<String>) -> EvalResult {
    match subpath {
        None => EvalResult {
            level: EvalResultLevel::ProbablyOk,
            explanation: "nothing to check on subpath".to_string(),
        },
        Some(s) => {
            let mut findings = vec![];
            EvalResult::aggregate(&findings)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::purl_data::{PurlComponent, PurlNamespace, PurlType};

    use super::{eval_purl_namespace, EvalResultLevel};

    use paste::paste;

    // credit for this approach to table-driven tests: burntsushi
    macro_rules! test_eval_ns {
        ($name:ident, $t:expr, $ns:expr, $expect:expr) => {
            paste! {
            #[test]
            fn [<test_eval_ns_ $name>]() {
                let typex = $t;
                let namespace = $ns;
                let expected_level = $expect;
                let result = eval_purl_namespace(PurlNamespace::new_naive(namespace), PurlType::new(typex));
                (result.level != expected_level).then(|| {
                    panic!("for type '{typex}' and ns '{namespace}' expected '{expected_level}' but got '{actual}'", actual=result.level)
                });
            }
            }
        }
    }

    test_eval_ns!(gh_normal, "github", "ja-he", EvalResultLevel::ProbablyOk);
    test_eval_ns!(gh_empty, "github", "", EvalResultLevel::AtLeastValid); // empty for github
    test_eval_ns!(
        gh_trailslash,
        "github",
        "ja-he/",
        EvalResultLevel::AtLeastValid
    );
    test_eval_ns!(
        gh_trailslash_many,
        "github",
        "ja-he////",
        EvalResultLevel::AtLeastValid
    );
    test_eval_ns!(
        gh_leadslash_many,
        "github",
        "////ja-he",
        EvalResultLevel::AtLeastValid
    );
    test_eval_ns!(
        gh_lead_and_trailslash,
        "github",
        "////ja-he//",
        EvalResultLevel::AtLeastValid
    );
    test_eval_ns!(
        gh_two_parts,
        "github",
        "ja/he",
        EvalResultLevel::AtLeastValid
    ); // more than 1 for github
    test_eval_ns!(
        github_underscore,
        "github",
        "ja_he",
        EvalResultLevel::AtLeastValid
    ); // github does not allow underscores
    test_eval_ns!(
        github_trailing_hyphen,
        "github",
        "jahe-",
        EvalResultLevel::AtLeastValid
    );
    test_eval_ns!(
        github_leading_hyphen,
        "github",
        "-jahe",
        EvalResultLevel::AtLeastValid
    );

    macro_rules! test_ord_geq {
        ($name:ident, $l:expr, $r:expr, $expect:expr) => {
            paste! {
            #[test]
            fn [<test_ord_geq_ $name>]() {
                let l = $l;
                let r = $r;
                let expected = $expect;
                let result = l >= r;
                if result != expected {
                    panic!("({l} >= {r}) expects {expected} got {result}")
                }
            }
            }
        };
    }

    test_ord_geq!(
        ve_ve,
        EvalResultLevel::Verified,
        EvalResultLevel::Verified,
        true
    );
    test_ord_geq!(
        ve_ok,
        EvalResultLevel::Verified,
        EvalResultLevel::ProbablyOk,
        false
    );
    test_ord_geq!(
        ve_va,
        EvalResultLevel::Verified,
        EvalResultLevel::AtLeastValid,
        false
    );
    test_ord_geq!(
        ve_in,
        EvalResultLevel::Verified,
        EvalResultLevel::Invalid,
        false
    );
    test_ord_geq!(
        ok_ve,
        EvalResultLevel::ProbablyOk,
        EvalResultLevel::Verified,
        true
    );
    test_ord_geq!(
        ok_ok,
        EvalResultLevel::ProbablyOk,
        EvalResultLevel::ProbablyOk,
        true
    );
    test_ord_geq!(
        ok_va,
        EvalResultLevel::ProbablyOk,
        EvalResultLevel::AtLeastValid,
        false
    );
    test_ord_geq!(
        ok_in,
        EvalResultLevel::ProbablyOk,
        EvalResultLevel::Invalid,
        false
    );
    test_ord_geq!(
        va_ve,
        EvalResultLevel::AtLeastValid,
        EvalResultLevel::Verified,
        true
    );
    test_ord_geq!(
        va_ok,
        EvalResultLevel::AtLeastValid,
        EvalResultLevel::ProbablyOk,
        true
    );
    test_ord_geq!(
        va_va,
        EvalResultLevel::AtLeastValid,
        EvalResultLevel::AtLeastValid,
        true
    );
    test_ord_geq!(
        va_in,
        EvalResultLevel::AtLeastValid,
        EvalResultLevel::Invalid,
        false
    );
    test_ord_geq!(
        in_ve,
        EvalResultLevel::Invalid,
        EvalResultLevel::Verified,
        true
    );
    test_ord_geq!(
        in_ok,
        EvalResultLevel::Invalid,
        EvalResultLevel::ProbablyOk,
        true
    );
    test_ord_geq!(
        in_va,
        EvalResultLevel::Invalid,
        EvalResultLevel::AtLeastValid,
        true
    );
    test_ord_geq!(
        in_in,
        EvalResultLevel::Invalid,
        EvalResultLevel::Invalid,
        true
    );

    macro_rules! test_mst {
        ($name:ident, $l:expr, $r:expr, $expect:expr) => {
            paste! {
            #[test]
            fn [<test_mst_ $name>]() {
                let l = $l;
                let r = $r;
                let expected = $expect;
                let result = l.more_severe_than(&r);
                if result != expected {
                    panic!("mst({l},{r}) expects {expected} got {result}")
                }
            }
            }
        };
    }

    test_mst!(
        ve_ve,
        EvalResultLevel::Verified,
        EvalResultLevel::Verified,
        false
    );
    test_mst!(
        ve_ok,
        EvalResultLevel::Verified,
        EvalResultLevel::ProbablyOk,
        false
    );
    test_mst!(
        ve_va,
        EvalResultLevel::Verified,
        EvalResultLevel::AtLeastValid,
        false
    );
    test_mst!(
        ve_in,
        EvalResultLevel::Verified,
        EvalResultLevel::Invalid,
        false
    );
    test_mst!(
        ok_ve,
        EvalResultLevel::ProbablyOk,
        EvalResultLevel::Verified,
        true
    );
    test_mst!(
        ok_ok,
        EvalResultLevel::ProbablyOk,
        EvalResultLevel::ProbablyOk,
        false
    );
    test_mst!(
        ok_va,
        EvalResultLevel::ProbablyOk,
        EvalResultLevel::AtLeastValid,
        false
    );
    test_mst!(
        ok_in,
        EvalResultLevel::ProbablyOk,
        EvalResultLevel::Invalid,
        false
    );
    test_mst!(
        va_ve,
        EvalResultLevel::AtLeastValid,
        EvalResultLevel::Verified,
        true
    );
    test_mst!(
        va_ok,
        EvalResultLevel::AtLeastValid,
        EvalResultLevel::ProbablyOk,
        true
    );
    test_mst!(
        va_va,
        EvalResultLevel::AtLeastValid,
        EvalResultLevel::AtLeastValid,
        false
    );
    test_mst!(
        va_in,
        EvalResultLevel::AtLeastValid,
        EvalResultLevel::Invalid,
        false
    );
    test_mst!(
        in_ve,
        EvalResultLevel::Invalid,
        EvalResultLevel::Verified,
        true
    );
    test_mst!(
        in_ok,
        EvalResultLevel::Invalid,
        EvalResultLevel::ProbablyOk,
        true
    );
    test_mst!(
        in_va,
        EvalResultLevel::Invalid,
        EvalResultLevel::AtLeastValid,
        true
    );
    test_mst!(
        in_in,
        EvalResultLevel::Invalid,
        EvalResultLevel::Invalid,
        false
    );

    macro_rules! test_alaga {
        ($name:ident, $l:expr, $r:expr, $expect:expr) => {
            paste! {
            #[test]
            fn [<test_alaga_ $name>]() {
                let l = $l;
                let r = $r;
                let expected = $expect;
                let result = l.at_least_as_good_as(&r);
                if result != expected {
                    panic!("alaga({l},{r}) expects {expected} got {result}")
                }
            }
            }
        };
    }

    test_alaga!(
        ve_ve,
        EvalResultLevel::Verified,
        EvalResultLevel::Verified,
        true
    );
    test_alaga!(
        ve_ok,
        EvalResultLevel::Verified,
        EvalResultLevel::ProbablyOk,
        true
    );
    test_alaga!(
        ve_va,
        EvalResultLevel::Verified,
        EvalResultLevel::AtLeastValid,
        true
    );
    test_alaga!(
        ve_in,
        EvalResultLevel::Verified,
        EvalResultLevel::Invalid,
        true
    );
    test_alaga!(
        ok_ve,
        EvalResultLevel::ProbablyOk,
        EvalResultLevel::Verified,
        false
    );
    test_alaga!(
        ok_ok,
        EvalResultLevel::ProbablyOk,
        EvalResultLevel::ProbablyOk,
        true
    );
    test_alaga!(
        ok_va,
        EvalResultLevel::ProbablyOk,
        EvalResultLevel::AtLeastValid,
        true
    );
    test_alaga!(
        ok_in,
        EvalResultLevel::ProbablyOk,
        EvalResultLevel::Invalid,
        true
    );
    test_alaga!(
        va_ve,
        EvalResultLevel::AtLeastValid,
        EvalResultLevel::Verified,
        false
    );
    test_alaga!(
        va_ok,
        EvalResultLevel::AtLeastValid,
        EvalResultLevel::ProbablyOk,
        false
    );
    test_alaga!(
        va_va,
        EvalResultLevel::AtLeastValid,
        EvalResultLevel::AtLeastValid,
        true
    );
    test_alaga!(
        va_in,
        EvalResultLevel::AtLeastValid,
        EvalResultLevel::Invalid,
        true
    );
    test_alaga!(
        in_ve,
        EvalResultLevel::Invalid,
        EvalResultLevel::Verified,
        false
    );
    test_alaga!(
        in_ok,
        EvalResultLevel::Invalid,
        EvalResultLevel::ProbablyOk,
        false
    );
    test_alaga!(
        in_va,
        EvalResultLevel::Invalid,
        EvalResultLevel::AtLeastValid,
        false
    );
    test_alaga!(
        in_in,
        EvalResultLevel::Invalid,
        EvalResultLevel::Invalid,
        true
    );
}
