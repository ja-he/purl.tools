use itertools::Itertools;
use leptos::*;

use crate::purl_data::PurlComponent;

mod purl_data;

#[macro_use]
extern crate lazy_static;
mod purl_eval;
mod purl_eval_cratesio;
mod purl_eval_github;
mod purl_eval_npm;

#[derive(Debug, Clone, PartialEq, Eq)]
enum CheckType {
    CratesIo,
    Github,
    Debian,
    Npm,
}

impl std::fmt::Display for CheckType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CheckType::CratesIo => write!(f, "crates.io/api/v1"),
            CheckType::Github => write!(f, "api.github.com"),
            CheckType::Npm => write!(f, "registry.npmjs.org"),
            CheckType::Debian => write!(f, "debian.org"),
        }
    }
}

#[component]
fn App() -> impl IntoView {
    let (light_theme, set_light_theme) = create_signal(true);
    leptos_meta::provide_meta_context();

    view! {
        <leptos_meta::Title text="purl Builder"></leptos_meta::Title>
        <div id="full-page">
            <div id="header">
                <div id="main-file-issue-hint">
                    <a href="https://github.com/ja-he/purl.tools/issues" target="_blank">
                        <phosphor_leptos::GithubLogo
                            class="github-hint-icon"
                            weight=phosphor_leptos::IconWeight::Fill
                        ></phosphor_leptos::GithubLogo>
                        <div class="github-hint-text-box">
                            <span class="github-hint-text">"Got an issue?"</span>
                        </div>
                    </a>
                </div>
                <div id="main-title">
                    <span id="title-text">"purl Builder"</span>
                    <span id="wip-disclaimer">"under construction"</span>
                </div>
                <div id="theme-toggle">
                    <button
                        id="theme-toggle-button"
                        on:click=move |_| { set_light_theme.update(|prev| { *prev = !*prev }) }
                    >
                        <Show when=move || light_theme.get() fallback=move || view! { "go dark" }>
                            "go light"
                        </Show>
                    </button>
                </div>
            </div>
            <div id="main-content">
                <MainContent/>
            </div>
            <div id="footer">"Created by " <a href="https://hensel.dev">"Jan Hensel"</a> "."</div>
        </div>
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InputOption {
    Select,
    Raw,
}

#[component]
fn MainContent() -> impl IntoView {
    // abtract: scheme:type/namespace/name@version?qualifiers#subpath
    // eg.:     pkg:github/package-url/purl-spec@244fd47e07d1004f0aed9c
    let (typex, set_typex) = create_signal(purl_data::PurlType::Github);
    let (namespace, set_namespace) = create_signal(purl_data::PurlNamespace::new_naive("ja-he"));
    let (name, set_name) = create_signal("dayplan".to_string());
    let (version, set_version) = create_signal(Some("v0.9.4".to_string()));
    let (qualifiers, set_qualifiers) = create_signal(None);
    let (subpath, set_subpath) = create_signal(None);

    let (type_input_option, set_type_input_option) = create_signal(InputOption::Select);
    let get_type_input_field = move || match type_input_option.get() {
        InputOption::Select => view! {
            <select
                class="purl-component-input"
                on:change=move |ev| {
                    let new_value = event_target_value(&ev);
                    set_typex(purl_data::PurlType::new(&new_value));
                }
            >

                {purl_data::PURL_TYPES
                    .iter()
                    .map(|t| {
                        view! {
                            <option
                                class=match t.status() {
                                    purl_data::PurlTypeStatus::WellKnown => "option-well-known",
                                    purl_data::PurlTypeStatus::Proposed => "option-proposed",
                                    purl_data::PurlTypeStatus::Other => "option-other",
                                }

                                value=t.to_string()
                                selected=move || typex().to_string() == t.to_string()
                            >
                                {t.to_string()}
                            </option>
                        }
                    })
                    .collect_view()}

            </select>
        }
        .into_any(),
        InputOption::Raw => view! {
            <input
                class="purl-component-input"
                type="text"
                on:input=move |ev| {
                    set_typex(purl_data::PurlType::new(&event_target_value(&ev)));
                }

                prop:value=move || typex().to_string()
            />
        }
        .into_any(),
    };

    let cycle_type_input_option = move |_| {
        set_type_input_option.update(|prev| {
            *prev = match *prev {
                InputOption::Select => InputOption::Raw,
                InputOption::Raw => InputOption::Select,
            }
        })
    };

    let eval_type = Signal::derive(move || purl_eval::eval_purl_type(typex()));
    let (eval_type_result, set_eval_type_result) =
        create_signal(purl_eval::EvalResultLevel::Verified);
    let (eval_type_result_explanation, set_eval_type_result_explanation) =
        create_signal("well-known identifier".to_string());
    create_effect(move |_| {
        let new = eval_type().level;
        let old = eval_type_result();
        if old != new {
            set_eval_type_result(new);
        }
        let new = eval_type().explanation;
        let old = eval_type_result_explanation();
        if old != new {
            set_eval_type_result_explanation(new);
        }
    });

    let (eval_namespace, set_eval_namespace) = create_signal(purl_eval::EvalResult {
        level: purl_eval::EvalResultLevel::ProbablyOk,
        explanation: "".to_string(),
    });
    create_effect(move |_| {
        set_eval_namespace(purl_eval::eval_purl_namespace(namespace(), typex()))
    });
    let (eval_namespace_result, set_eval_namespace_result) =
        create_signal(purl_eval::EvalResultLevel::ProbablyOk);
    let (eval_namespace_result_explanation, set_eval_namespace_result_explanation) =
        create_signal("".to_string());
    create_effect(move |_| {
        let new = eval_namespace().level;
        let old = eval_namespace_result();
        if old != new {
            set_eval_namespace_result(new);
        }
        let new = eval_namespace().explanation;
        let old = eval_namespace_result_explanation();
        if old != new {
            set_eval_namespace_result_explanation(new);
        }
    });

    let (eval_name, set_eval_name) = create_signal(purl_eval::EvalResult {
        level: purl_eval::EvalResultLevel::ProbablyOk,
        explanation: "".to_string(),
    });
    create_effect(move |_| set_eval_name(purl_eval::eval_purl_name(name(), namespace(), typex())));
    let (eval_name_result, set_eval_name_result) =
        create_signal(purl_eval::EvalResultLevel::ProbablyOk);
    let (eval_name_result_explanation, set_eval_name_result_explanation) =
        create_signal("".to_string());
    create_effect(move |_| {
        let new = eval_name().level;
        let old = eval_name_result();
        if old != new {
            set_eval_name_result(new);
        }
        let new = eval_name().explanation;
        let old = eval_name_result_explanation();
        if old != new {
            set_eval_name_result_explanation(new);
        }
    });

    let (eval_version, set_eval_version) = create_signal(purl_eval::EvalResult {
        level: purl_eval::EvalResultLevel::ProbablyOk,
        explanation: "".to_string(),
    });
    create_effect(move |_| {
        set_eval_version(purl_eval::eval_purl_version(
            typex(),
            namespace(),
            name(),
            version(),
        ))
    });
    let (eval_version_result, set_eval_version_result) =
        create_signal(purl_eval::EvalResultLevel::ProbablyOk);
    let (eval_version_result_explanation, set_eval_version_result_explanation) =
        create_signal("".to_string());
    create_effect(move |_| {
        let new = eval_version().level;
        let old = eval_version_result();
        if old != new {
            set_eval_version_result(new);
        }
        let new = eval_version().explanation;
        let old = eval_version_result_explanation();
        if old != new {
            set_eval_version_result_explanation(new);
        }
    });

    let eval_qualifiers = move || purl_eval::eval_purl_qualifiers(qualifiers());
    let (eval_qualifiers_result, set_eval_qualifiers_result) =
        create_signal(purl_eval::EvalResultLevel::ProbablyOk);
    let (eval_qualifiers_result_explanation, set_eval_qualifiers_result_explanation) =
        create_signal("".to_string());
    create_effect(move |_| {
        let new = eval_qualifiers().level;
        let old = eval_qualifiers_result();
        if old != new {
            set_eval_qualifiers_result(new);
        }
        let new = eval_qualifiers().explanation;
        let old = eval_qualifiers_result_explanation();
        if old != new {
            set_eval_qualifiers_result_explanation(new);
        }
    });

    let eval_subpath = move || purl_eval::eval_purl_subpath(subpath());
    let (eval_subpath_result, set_eval_subpath_result) =
        create_signal(purl_eval::EvalResultLevel::ProbablyOk);
    let (eval_subpath_result_explanation, set_eval_subpath_result_explanation) =
        create_signal("".to_string());
    create_effect(move |_| {
        let new = eval_subpath().level;
        let old = eval_subpath_result();
        if old != new {
            set_eval_subpath_result(new);
        }
        let new = eval_subpath().explanation;
        let old = eval_subpath_result_explanation();
        if old != new {
            set_eval_subpath_result_explanation(new);
        }
    });

    let (all_at_least_probably_ok, set_all_at_least_probably_ok) = create_signal(true);
    let all_at_least_probably_ok_raw = Signal::derive(move || {
        with!(
            |eval_type_result, eval_namespace_result, eval_name_result| {
                let ok = purl_eval::EvalResultLevel::ProbablyOk;
                [eval_type_result, eval_namespace_result, eval_name_result]
                    .iter()
                    .all(|part_result| part_result.at_least_as_good_as(&ok))
            }
        )
    });
    create_effect(move |_| {
        let new = all_at_least_probably_ok_raw();
        let old = all_at_least_probably_ok();
        if new != old {
            set_all_at_least_probably_ok(new);
        }
    });
    let full_purl_debounced = leptos_use::signal_debounced(
        Signal::derive(move || {
            with!(
                |typex, namespace, name, version, all_at_least_probably_ok| {
                    (
                        typex.clone(),
                        namespace.join("/").clone(),
                        name.clone(),
                        version.clone(),
                        *all_at_least_probably_ok,
                    )
                }
            )
        }),
        1000.0,
    );
    let (active_expensive_check, set_active_expensive_check) =
        create_signal::<Option<CheckType>>(None);
    create_effect(move |_| {
        let (t, ns, n, v, ok) = full_purl_debounced();
        if !ok {
            return;
        }

        match t {
            purl_data::PurlType::Cargo => spawn_local(async move {
                set_active_expensive_check(Some(CheckType::CratesIo));
                if let Ok(versions) = purl_eval_cratesio::get_versions(&n).await {
                    set_eval_name(purl_eval::EvalResult {
                        level: purl_eval::EvalResultLevel::Verified,
                        explanation: "exists on crates.io".to_string(),
                    });
                    if let Some(v) = v {
                        if versions.contains(&v) {
                            set_eval_version(purl_eval::EvalResult {
                                level: purl_eval::EvalResultLevel::Verified,
                                explanation: "exists on crates.io".to_string(),
                            })
                        } else {
                            set_eval_version(purl_eval::EvalResult {
                                level: purl_eval::EvalResultLevel::AtLeastValid,
                                explanation: "not found on crates.io".to_string(),
                            })
                        }
                    }
                } else {
                    set_eval_name(purl_eval::EvalResult {
                        level: purl_eval::EvalResultLevel::AtLeastValid,
                        explanation: "not found on crates.io".to_string(),
                    });
                }
                set_active_expensive_check(None);
            }),

            purl_data::PurlType::Github => spawn_local(async move {
                set_active_expensive_check(Some(CheckType::Github));

                let mut found_version = false;
                if let Some(v) = v {
                    match purl_eval_github::repo_exists_with_version(&ns, &n, &v).await {
                        Ok(found) => {
                            found_version = found;
                            if found {
                                set_eval_version(purl_eval::EvalResult {
                                    level: purl_eval::EvalResultLevel::Verified,
                                    explanation: "the version (release tag) exists on GitHub"
                                        .to_string(),
                                });
                                set_eval_name(purl_eval::EvalResult {
                                    level: purl_eval::EvalResultLevel::Verified,
                                    explanation: "the repository exists on GitHub".to_string(),
                                });
                                set_eval_namespace(purl_eval::EvalResult {
                                    level: purl_eval::EvalResultLevel::Verified,
                                    explanation: "the namespace exists on GitHub as a user or org"
                                        .to_string(),
                                });
                            } else {
                                set_eval_version(purl_eval::EvalResult {
                                    level: purl_eval::EvalResultLevel::AtLeastValid,
                                    explanation:
                                        "the version (release tag) does not exist on GitHub"
                                            .to_string(),
                                });
                            }
                        }
                        Err(e) => log::warn!(
                            "an unexpected error occurred checking for a GitHub repository ({e})"
                        ),
                    }
                }

                if !found_version {
                    match purl_eval_github::repo_exists(&ns, &n).await {
                        Ok(true) => {
                            set_eval_name(purl_eval::EvalResult {
                                level: purl_eval::EvalResultLevel::Verified,
                                explanation: "the repository exists on GitHub".to_string(),
                            });
                            set_eval_namespace(purl_eval::EvalResult {
                                level: purl_eval::EvalResultLevel::Verified,
                                explanation: "the namespace exists on GitHub as a user or org"
                                    .to_string(),
                            });
                        }
                        Ok(false) => {
                            set_eval_name(purl_eval::EvalResult {
                                level: purl_eval::EvalResultLevel::AtLeastValid,
                                explanation: "did not find the repository on GitHub".to_string(),
                            });
                            match purl_eval_github::user_or_org_exists(&ns).await {
                                Ok(true) => set_eval_namespace(purl_eval::EvalResult {
                                    level: purl_eval::EvalResultLevel::Verified,
                                    explanation: "the namespace exists on GitHub as a user or org"
                                        .to_string(),
                                }),
                                Ok(false) => set_eval_namespace(purl_eval::EvalResult {
                                    level: purl_eval::EvalResultLevel::AtLeastValid,
                                    explanation: "did not find this as a user or org on GitHub"
                                        .to_string(),
                                }),
                                Err(e) => log::warn!("an unexpected error occurred checking for a GitHub repository ({e})"),
                            }
                        }
                        Err(e) => log::warn!(
                            "an unexpected error occurred checking for a GitHub repository ({e})"
                        ),
                    }
                }

                set_active_expensive_check(None);
            }),
            purl_data::PurlType::Npm => spawn_local(async move {
                set_active_expensive_check(Some(CheckType::Npm));

                match purl_eval_npm::get_package(&n).await {
                    Ok(Some(package)) => {
                        set_eval_name(purl_eval::EvalResult {
                            level: purl_eval::EvalResultLevel::Verified,
                            explanation: "found on NPM".to_string(),
                        });
                        if let Some(version) = v {
                            if package
                                .versions
                                .iter()
                                .any(|(version_as_key, _)| *version_as_key == version)
                            {
                                set_eval_version(purl_eval::EvalResult {
                                    level: purl_eval::EvalResultLevel::Verified,
                                    explanation: "found on NPM".to_string(),
                                });
                            } else {
                                set_eval_version(purl_eval::EvalResult {
                                    level: purl_eval::EvalResultLevel::AtLeastValid,
                                    explanation: "not found on NPM".to_string(),
                                });
                            }
                        }
                    }
                    Ok(None) => {
                        set_eval_name(purl_eval::EvalResult {
                            level: purl_eval::EvalResultLevel::AtLeastValid,
                            explanation: "did not find this package on NPM".to_string(),
                        });
                    }
                    Err(e) => {
                        log::warn!(
                            "an unexpected error occurred checking for an NPM project ({e})"
                        );
                    }
                }

                set_active_expensive_check(None);
            }),

            purl_data::PurlType::Deb => spawn_local(async move {
                let ns = namespace();
                if ns.len() != 1 {
                    set_eval_namespace(purl_eval::EvalResult {
                        level: purl_eval::EvalResultLevel::Invalid,
                        explanation: "deb namespace must be exactly one component".to_string(),
                    });
                }
                match ns[0].as_str() {
                    "debian" => {
                        set_eval_namespace(purl_eval::EvalResult {
                            level: purl_eval::EvalResultLevel::Verified,
                            explanation: "debian namespace is correct".to_string(),
                        });
                        set_active_expensive_check(Some(CheckType::Debian));
                        gloo_timers::future::sleep(std::time::Duration::from_secs(2)).await;
                        set_active_expensive_check(None);
                    }
                    "ubuntu" => {
                        set_eval_namespace(purl_eval::EvalResult {
                            level: purl_eval::EvalResultLevel::Verified,
                            explanation: "ubuntu namespace is correct".to_string(),
                        });
                        log::warn!("not currently checking for Ubuntu packages");
                    }
                    other => {
                        set_eval_namespace(purl_eval::EvalResult {
                            level: purl_eval::EvalResultLevel::AtLeastValid,
                            explanation: format!(
                                "unknown namespace '{other}' but syntactically correct"
                            )
                            .to_string(),
                        });
                    }
                }
            }),

            _ => {}
        }
    });

    let get_type_explanation_box_class =
        move || format!("explanation-box {result}", result = eval_type_result());
    let get_namespace_explanation_box_class =
        move || format!("explanation-box {result}", result = eval_namespace_result());
    let get_name_explanation_box_class =
        move || format!("explanation-box {result}", result = eval_name_result());
    let get_version_explanation_box_class =
        move || format!("explanation-box {result}", result = eval_version_result());
    let get_qualifiers_explanation_box_class = move || {
        format!(
            "explanation-box {result}",
            result = eval_qualifiers_result()
        )
    };
    let get_subpath_explanation_box_class =
        move || format!("explanation-box {result}", result = eval_subpath_result());

    view! {
        <div id="input-form">
            <div class="input-row">
                <span class="input-label">"type"</span>
                {get_type_input_field}
                <button
                    id="type-input-toggle-button"
                    class="purl-input-options-button"
                    on:click=cycle_type_input_option
                >
                    <Show
                        when=move || { type_input_option() == InputOption::Select }
                        fallback=|| {
                            view! {
                                <phosphor_leptos::Cursor
                                    class="button-icon"
                                    weight=phosphor_leptos::IconWeight::Bold
                                ></phosphor_leptos::Cursor>
                            }
                        }
                    >

                        <phosphor_leptos::PencilSimple
                            class="button-icon"
                            weight=phosphor_leptos::IconWeight::Bold
                        ></phosphor_leptos::PencilSimple>
                    </Show>
                </button>
            </div>
            <div class="input-row">
                <span class="input-label">"namespace"</span>
                <input
                    class="purl-component-input"
                    type="text"
                    on:input=move |ev| {
                        set_namespace(
                            purl_data::PurlComponent::new_naive(&event_target_value(&ev)),
                        );
                    }

                    prop:value=move || namespace().join("/")
                />
            </div>
            <div class="input-row">
                <span class="input-label">"name"</span>
                <input
                    class="purl-component-input"
                    type="text"
                    on:input=move |ev| {
                        set_name(urlencoding::encode(&event_target_value(&ev)).into_owned());
                    }

                    prop:value=move || urlencoding::decode(&name()).unwrap_or_default().into_owned()
                />
            </div>
            <div class="input-row">
                <span class="input-label">"version"</span>
                <input
                    class="purl-component-input"
                    type="text"
                    on:input=move |ev| {
                        set_version(
                            if !event_target_value(&ev).is_empty() {
                                Some(urlencoding::encode(&event_target_value(&ev)).into_owned())
                            } else {
                                None
                            },
                        );
                    }

                    prop:value=move || {
                        urlencoding::decode(&version().unwrap_or_default())
                            .unwrap_or_default()
                            .into_owned()
                    }
                />

            </div>
            <div class="input-row">
                <span class="input-label">"qualifiers"</span>
                <input
                    class="purl-component-input"
                    type="text"
                    on:input=move |ev| {
                        set_qualifiers(
                            if !event_target_value(&ev).is_empty() {
                                Some(event_target_value(&ev))
                            } else {
                                None
                            },
                        );
                    }

                    prop:value=move || qualifiers().unwrap_or_default()
                />
            </div>
            <div class="input-row">
                <span class="input-label">"subpath"</span>
                <input
                    class="purl-component-input"
                    type="text"
                    on:input=move |ev| {
                        set_subpath(
                            if !event_target_value(&ev).is_empty() {
                                Some(event_target_value(&ev))
                            } else {
                                None
                            },
                        );
                    }

                    prop:value=move || subpath().unwrap_or_default()
                />
            </div>
        </div>

        <Purl
            typex=typex
            eval_type_result=eval_type_result
            namespace=namespace
            eval_namespace_result=eval_namespace_result
            name=name
            eval_name_result=eval_name_result
            version=version
            eval_version_result=eval_version_result
            qualifiers=qualifiers
            eval_qualifiers_result=eval_qualifiers_result
            subpath=subpath
            eval_subpath_result=eval_subpath_result
        />

        <div class="explanation-box-wrapper">
            <div class="check-indicator">
                <Show
                    when=move || active_expensive_check().is_some()
                    fallback=move || view! { <div class="no-check"></div> }
                >
                    <div class="active-check">

                        {move || {
                            match active_expensive_check() {
                                Some(check) => {
                                    view! {
                                        <phosphor_leptos::CircleNotch
                                            class="loading-indicator-circular"
                                            weight=phosphor_leptos::IconWeight::Bold
                                        ></phosphor_leptos::CircleNotch>
                                        <p class="check-explanation">{check.to_string()}</p>
                                    }
                                }
                                None => {
                                    view! {
                                        <phosphor_leptos::Warning weight=phosphor_leptos::IconWeight::Duotone></phosphor_leptos::Warning>
                                        <p class="check-explanation">
                                            "something went weirdly wrong, but that is fine..."
                                        </p>
                                    }
                                }
                            }
                        }}

                    </div>
                </Show>
            </div>
            <div class=get_type_explanation_box_class>
                {move || match eval_type_result() {
                    purl_eval::EvalResultLevel::Verified => {
                        view! {
                            <phosphor_leptos::Checks
                                class="explanation-icon verified"
                                weight=phosphor_leptos::IconWeight::Bold
                            ></phosphor_leptos::Checks>
                        }
                    }
                    purl_eval::EvalResultLevel::ProbablyOk => {
                        view! {
                            <phosphor_leptos::Check
                                class="explanation-icon ok"
                                weight=phosphor_leptos::IconWeight::Bold
                            ></phosphor_leptos::Check>
                        }
                    }
                    purl_eval::EvalResultLevel::AtLeastValid => {
                        view! {
                            <phosphor_leptos::Question
                                class="explanation-icon valid"
                                weight=phosphor_leptos::IconWeight::Bold
                            ></phosphor_leptos::Question>
                        }
                    }
                    purl_eval::EvalResultLevel::Invalid => {
                        view! {
                            <phosphor_leptos::Warning
                                class="explanation-icon invalid"
                                weight=phosphor_leptos::IconWeight::Bold
                            ></phosphor_leptos::Warning>
                        }
                    }
                }}
                <span class="headline">{move || eval_type_result().to_string()}</span>
                <span class="explanation">{eval_type_result_explanation}</span>
            </div>
            <div class=get_namespace_explanation_box_class>
                {move || match eval_namespace_result() {
                    purl_eval::EvalResultLevel::Verified => {
                        view! {
                            <phosphor_leptos::Checks
                                class="explanation-icon verified"
                                weight=phosphor_leptos::IconWeight::Bold
                            ></phosphor_leptos::Checks>
                        }
                    }
                    purl_eval::EvalResultLevel::ProbablyOk => {
                        view! {
                            <phosphor_leptos::Check
                                class="explanation-icon ok"
                                weight=phosphor_leptos::IconWeight::Bold
                            ></phosphor_leptos::Check>
                        }
                    }
                    purl_eval::EvalResultLevel::AtLeastValid => {
                        view! {
                            <phosphor_leptos::Question
                                class="explanation-icon valid"
                                weight=phosphor_leptos::IconWeight::Bold
                            ></phosphor_leptos::Question>
                        }
                    }
                    purl_eval::EvalResultLevel::Invalid => {
                        view! {
                            <phosphor_leptos::Warning
                                class="explanation-icon invalid"
                                weight=phosphor_leptos::IconWeight::Bold
                            ></phosphor_leptos::Warning>
                        }
                    }
                }}
                <span class="headline">{move || { eval_namespace_result().to_string() }}</span>
                <span class="explanation">{eval_namespace_result_explanation}</span>
            </div>
            <div class=get_name_explanation_box_class>
                {move || match eval_name_result() {
                    purl_eval::EvalResultLevel::Verified => {
                        view! {
                            <phosphor_leptos::Checks
                                class="explanation-icon verified"
                                weight=phosphor_leptos::IconWeight::Bold
                            ></phosphor_leptos::Checks>
                        }
                    }
                    purl_eval::EvalResultLevel::ProbablyOk => {
                        view! {
                            <phosphor_leptos::Check
                                class="explanation-icon ok"
                                weight=phosphor_leptos::IconWeight::Bold
                            ></phosphor_leptos::Check>
                        }
                    }
                    purl_eval::EvalResultLevel::AtLeastValid => {
                        view! {
                            <phosphor_leptos::Question
                                class="explanation-icon valid"
                                weight=phosphor_leptos::IconWeight::Bold
                            ></phosphor_leptos::Question>
                        }
                    }
                    purl_eval::EvalResultLevel::Invalid => {
                        view! {
                            <phosphor_leptos::Warning
                                class="explanation-icon invalid"
                                weight=phosphor_leptos::IconWeight::Bold
                            ></phosphor_leptos::Warning>
                        }
                    }
                }}
                <span class="headline">{move || { eval_name_result().to_string() }}</span>
                <span class="explanation">{eval_name_result_explanation}</span>
            </div>
            <div class=get_version_explanation_box_class>
                {move || match eval_version_result() {
                    purl_eval::EvalResultLevel::Verified => {
                        view! {
                            <phosphor_leptos::Checks
                                class="explanation-icon verified"
                                weight=phosphor_leptos::IconWeight::Bold
                            ></phosphor_leptos::Checks>
                        }
                    }
                    purl_eval::EvalResultLevel::ProbablyOk => {
                        view! {
                            <phosphor_leptos::Check
                                class="explanation-icon ok"
                                weight=phosphor_leptos::IconWeight::Bold
                            ></phosphor_leptos::Check>
                        }
                    }
                    purl_eval::EvalResultLevel::AtLeastValid => {
                        view! {
                            <phosphor_leptos::Question
                                class="explanation-icon valid"
                                weight=phosphor_leptos::IconWeight::Bold
                            ></phosphor_leptos::Question>
                        }
                    }
                    purl_eval::EvalResultLevel::Invalid => {
                        view! {
                            <phosphor_leptos::Warning
                                class="explanation-icon invalid"
                                weight=phosphor_leptos::IconWeight::Bold
                            ></phosphor_leptos::Warning>
                        }
                    }
                }}
                <span class="headline">{move || { eval_version_result().to_string() }}</span>
                <span class="explanation">{eval_version_result_explanation}</span>
            </div>
            <div class=get_qualifiers_explanation_box_class>
                {move || match eval_qualifiers_result() {
                    purl_eval::EvalResultLevel::Verified => {
                        view! {
                            <phosphor_leptos::Checks
                                class="explanation-icon verified"
                                weight=phosphor_leptos::IconWeight::Bold
                            ></phosphor_leptos::Checks>
                        }
                    }
                    purl_eval::EvalResultLevel::ProbablyOk => {
                        view! {
                            <phosphor_leptos::Check
                                class="explanation-icon ok"
                                weight=phosphor_leptos::IconWeight::Bold
                            ></phosphor_leptos::Check>
                        }
                    }
                    purl_eval::EvalResultLevel::AtLeastValid => {
                        view! {
                            <phosphor_leptos::Question
                                class="explanation-icon valid"
                                weight=phosphor_leptos::IconWeight::Bold
                            ></phosphor_leptos::Question>
                        }
                    }
                    purl_eval::EvalResultLevel::Invalid => {
                        view! {
                            <phosphor_leptos::Warning
                                class="explanation-icon invalid"
                                weight=phosphor_leptos::IconWeight::Bold
                            ></phosphor_leptos::Warning>
                        }
                    }
                }}
                <span class="headline">{move || { eval_qualifiers_result().to_string() }}</span>
                <span class="explanation">{eval_qualifiers_result_explanation}</span>
            </div>
            <div class=get_subpath_explanation_box_class>
                {move || match eval_subpath_result() {
                    purl_eval::EvalResultLevel::Verified => {
                        view! {
                            <phosphor_leptos::Checks
                                class="explanation-icon verified"
                                weight=phosphor_leptos::IconWeight::Bold
                            ></phosphor_leptos::Checks>
                        }
                    }
                    purl_eval::EvalResultLevel::ProbablyOk => {
                        view! {
                            <phosphor_leptos::Check
                                class="explanation-icon ok"
                                weight=phosphor_leptos::IconWeight::Bold
                            ></phosphor_leptos::Check>
                        }
                    }
                    purl_eval::EvalResultLevel::AtLeastValid => {
                        view! {
                            <phosphor_leptos::Question
                                class="explanation-icon valid"
                                weight=phosphor_leptos::IconWeight::Bold
                            ></phosphor_leptos::Question>
                        }
                    }
                    purl_eval::EvalResultLevel::Invalid => {
                        view! {
                            <phosphor_leptos::Warning
                                class="explanation-icon invalid"
                                weight=phosphor_leptos::IconWeight::Bold
                            ></phosphor_leptos::Warning>
                        }
                    }
                }}
                <span class="headline">{move || { eval_subpath_result().to_string() }}</span>
                <span class="explanation">{eval_subpath_result_explanation}</span>
            </div>
        </div>
    }
}

#[component]
fn EvalIcon(eval_result: ReadSignal<purl_eval::EvalResult>) -> impl IntoView {
    view! {
        {match eval_result().level {
            purl_eval::EvalResultLevel::Verified => {
                view! {
                    <phosphor_leptos::Checks
                        class="explanation-icon verified"
                        weight=phosphor_leptos::IconWeight::Bold
                    ></phosphor_leptos::Checks>
                }
            }
            purl_eval::EvalResultLevel::ProbablyOk => {
                view! {
                    <phosphor_leptos::Check
                        class="explanation-icon ok"
                        weight=phosphor_leptos::IconWeight::Bold
                    ></phosphor_leptos::Check>
                }
            }
            purl_eval::EvalResultLevel::AtLeastValid => {
                view! {
                    <phosphor_leptos::Question
                        class="explanation-icon valid"
                        weight=phosphor_leptos::IconWeight::Bold
                    ></phosphor_leptos::Question>
                }
            }
            purl_eval::EvalResultLevel::Invalid => {
                view! {
                    <phosphor_leptos::Warning
                        class="explanation-icon invalid"
                        weight=phosphor_leptos::IconWeight::Bold
                    ></phosphor_leptos::Warning>
                }
            }
        }}
    }
}

#[component]
fn Purl(
    typex: ReadSignal<purl_data::PurlType>,
    eval_type_result: ReadSignal<purl_eval::EvalResultLevel>,
    namespace: ReadSignal<Vec<String>>,
    eval_namespace_result: ReadSignal<purl_eval::EvalResultLevel>,
    name: ReadSignal<String>,
    eval_name_result: ReadSignal<purl_eval::EvalResultLevel>,
    version: ReadSignal<Option<String>>,
    eval_version_result: ReadSignal<purl_eval::EvalResultLevel>,
    qualifiers: ReadSignal<Option<String>>,
    eval_qualifiers_result: ReadSignal<purl_eval::EvalResultLevel>,
    subpath: ReadSignal<Option<String>>,
    eval_subpath_result: ReadSignal<purl_eval::EvalResultLevel>,
) -> impl IntoView {
    let get_purl_type_classes =
        move || format!("purl-type identifier-{result}", result = eval_type_result());
    let get_purl_namespace_classes = move || {
        format!(
            "purl-namespace-full identifier-{result}",
            result = eval_namespace_result()
        )
    };
    let get_purl_name_classes =
        move || format!("purl-name identifier-{result}", result = eval_name_result());
    let get_purl_version_classes = move || {
        format!(
            "purl-version identifier-{result}",
            result = eval_version_result()
        )
    };
    let get_purl_qualifiers_classes = move || {
        format!(
            "purl-qualifiers identifier-{result}",
            result = eval_qualifiers_result()
        )
    };
    let get_purl_subpath_classes = move || {
        format!(
            "purl-subpath identifier-{result}",
            result = eval_subpath_result()
        )
    };

    let namespace_and_leading_slash = move || {
        let namespace_view = move || {
            namespace()
                .as_canonical()
                .iter()
                .map(|ns_part| view! { <span class="purl-namespace-part">{ns_part}</span> })
                .intersperse_with(
                    || view! { <span class="purl-sep namespace-inner-sep">"/"</span> },
                )
                .collect_view()
        };
        view! {
            <Show when=move || !namespace().as_canonical().is_empty()>
                <span class="purl-sep">"/"</span>
                <span class=get_purl_namespace_classes>{namespace_view}</span>
            </Show>
        }
    };

    // abtract: scheme:type/namespace/name@version?qualifiers#subpath
    view! {
        <div class="purl">
            <span class="purl-scheme">"pkg"</span>
            <span class="purl-sep">:</span>
            <span class=get_purl_type_classes>
                {move || typex.with(purl_data::PurlType::to_string)}
            </span>
            {namespace_and_leading_slash}
            <span class="purl-sep">/</span>
            <span class=get_purl_name_classes>{name}</span>
            {move || {
                version
                    .get()
                    .is_some()
                    .then(|| {
                        view! {
                            <span class="purl-sep">@</span>
                            <span class=get_purl_version_classes>{version}</span>
                        }
                    })
            }}

            {move || {
                qualifiers
                    .get()
                    .is_some()
                    .then(|| {
                        view! {
                            <span class="purl-sep">?</span>
                            <span class=get_purl_qualifiers_classes>
                                {move || qualifiers.get()}
                            </span>
                        }
                    })
            }}

            {move || {
                subpath
                    .get()
                    .is_some()
                    .then(|| {
                        view! {
                            <span class="purl-sep">#</span>
                            <span class=get_purl_subpath_classes>{subpath}</span>
                        }
                    })
            }}

        </div>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    leptos::mount_to_body(|| view! { <App/> })
}
