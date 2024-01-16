use leptos::*;

mod purl_data;

// const TYPE_REGEX: regex::Regex = regex::Regex::new(r"^[a-zA-Z\.\+\-][a-zA-Z0-9\.\+\-]*$").unwrap();

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref TYPE_REGEX: regex::Regex =
        regex::Regex::new(r"^[a-zA-Z\.\+\-][a-zA-Z0-9\.\+\-]*$").unwrap();
}

#[component]
fn App() -> impl IntoView {
    let (light_theme, set_light_theme) = create_signal(true);
    leptos_meta::provide_meta_context();

    view! {
        <leptos_meta::Title text="purl Builder"/>
        <div id="full-page">
            <div id="header">
                <div  id="main-title">
                    <span id="title-text">"purl Builder"</span>
                    <span id="wip-disclaimer">"under construction"</span>
                </div>
                <div id="theme-toggle">
                    <button id="theme-toggle-button" on:click=move |_| { set_light_theme.update(|prev| { *prev = !*prev }) }>
                        <Show when=move || light_theme.get() fallback=move || view! { "go dark" }>"go light"</Show>
                    </button>
                </div>
            </div>
            <div id="main-content">
                <MainContent/>
            </div>
            <div id="footer">
                "Created by "
                <a href="https://hensel.dev">
                    "Jan Hensel"
                </a>
                "."
            </div>
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
    let (typestr, set_typestr) = create_signal("github".to_string());
    let (namespace, set_namespace) = create_signal("ja-he".to_string());
    let (name, set_name) = create_signal("dayplan".to_string());
    let (version, set_version) = create_signal(Some("v0.9.4".to_string()));
    let (qualifiers, set_qualifiers) = create_signal(None);
    let (subpath, set_subpath) = create_signal(None);

    let (type_input_option, set_type_input_option) = create_signal(InputOption::Select);
    let get_type_input_field = move || {
        match type_input_option.get() {
        InputOption::Select => view! {
                <select class="purl-component-input" on:change=move |ev| {
                    let new_value = event_target_value(&ev);
                    set_typestr(new_value);
                }>
                    {
                        purl_data::PURL_TYPES.iter()
                            .map(|(type_option, choice_status)| view! {
                                <PurlTypeOption typestr=typestr is=type_option status=*choice_status/>
                            })
                            .collect_view()
                     }
                </select>
        }
        .into_any(),
        InputOption::Raw => view! {
                <input class="purl-component-input" type="text"
                    on:input=move |ev| { set_typestr(event_target_value(&ev)); }
                    prop:value=typestr
                />
        }
        .into_any(),
    }
    };

    let cycle_type_input_option = move |_| {
        set_type_input_option.update(|prev| {
            *prev = match *prev {
                InputOption::Select => InputOption::Raw,
                InputOption::Raw => InputOption::Select,
            }
        })
    };

    view! {
        <div id="input-form">
            <div class="input-row">
                <span class="input-label">"type"</span>
                {get_type_input_field}
                <button
                    id="type-input-toggle-button"
                    class="purl-input-options-button"
                    on:click=cycle_type_input_option>
                    "switch input"
                </button>
            </div>
            <div class="input-row">
                <span class="input-label">"namespace"</span>
                <input class="purl-component-input" type="text"
                    on:input=move |ev| { set_namespace(event_target_value(&ev)); }
                    prop:value=namespace
                />
            </div>
            <div class="input-row">
                <span class="input-label">"name"</span>
                <input class="purl-component-input" type="text"
                    on:input=move |ev| { set_name(event_target_value(&ev)); }
                    prop:value=name
                />
            </div>
            <div class="input-row">
                <span class="input-label">"version"</span>
                <input class="purl-component-input" type="text"
                    on:input=move |ev| { set_version(
                        if !event_target_value(&ev).is_empty() {
                            Some(event_target_value(&ev))
                        } else {
                            None
                        }
                    ); }
                    prop:value=version().unwrap_or_default()
                />
            </div>
            <div class="input-row">
                <span class="input-label">"qualifiers"</span>
                <input class="purl-component-input" type="text"
                    on:input=move |ev| { set_qualifiers(
                        if !event_target_value(&ev).is_empty() {
                            Some(event_target_value(&ev))
                        } else {
                            None
                        }
                    ); }
                    prop:value=qualifiers().unwrap_or_default()
                />
            </div>
            <div class="input-row">
                <span class="input-label">"subpath"</span>
                <input class="purl-component-input" type="text"
                    on:input=move |ev| { set_subpath(
                        if !event_target_value(&ev).is_empty() {
                            Some(event_target_value(&ev))
                        } else {
                            None
                        }
                    ); }
                    prop:value=subpath().unwrap_or_default()
                />
            </div>
        </div>

        <Purl
            typestr={typestr}
            namespace={namespace}
            name={name}
            version={version}
            qualifiers={qualifiers}
            subpath={subpath}
        />
    }
}

#[component]
pub fn PurlTypeOption(
    typestr: ReadSignal<String>,
    is: &'static str,
    status: purl_data::PurlTypeStatus,
) -> impl IntoView {
    view! {
        <option
            class={ match status {
                purl_data::PurlTypeStatus::WellKnown => "option-well-known",
                purl_data::PurlTypeStatus::Proposed => "option-proposed",
                purl_data::PurlTypeStatus::Other => "option-other", // this case would not happen, normally
            }}
            value=is
            selected=move || typestr() == is
        >
            {is}
        </option>
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum EvalResult {
    Verified(String),
    ProbablyOk(String),
    AtLeastValid(String),
    Invalid(String),
}

#[component]
fn Purl(
    typestr: ReadSignal<String>,
    namespace: ReadSignal<String>,
    name: ReadSignal<String>,
    version: ReadSignal<Option<String>>,
    qualifiers: ReadSignal<Option<String>>,
    subpath: ReadSignal<Option<String>>,
) -> impl IntoView {
    let eval_type = move || {
        match purl_data::get_purl_type_status(&typestr()) {
            purl_data::PurlTypeStatus::WellKnown => {
                EvalResult::Verified("well-known identifier".to_string())
            }
            purl_data::PurlTypeStatus::Proposed => {
                EvalResult::ProbablyOk("officially proposed identifier".to_string())
            }
            purl_data::PurlTypeStatus::Other => {
                if typestr.get().is_empty() {
                    EvalResult::Invalid("type must not be empty".to_string())
                } else if TYPE_REGEX.is_match(&typestr.get()) {
                    EvalResult::AtLeastValid("valid identifier".to_string())
                } else {
                    EvalResult::Invalid("does not match regex".to_string())
                }
            }
        }
    };

    let get_purl_type_classes = {
        move || {
            format!(
                "{t} {status}",
                t = "purl-type",
                status = match eval_type() {
                    EvalResult::Verified(_) => "identifier-verified",
                    EvalResult::ProbablyOk(_) => "identifier-ok",
                    EvalResult::AtLeastValid(_) => "identifier-unknown",
                    EvalResult::Invalid(_) => "identifier-invalid",
                }
            )
        }
    };

    // abtract: scheme:type/namespace/name@version?qualifiers#subpath
    view! {
        <div class="purl">
            <span class="purl-scheme">"pkg"</span>
            <span class="purl-sep">:</span>
            <span class=get_purl_type_classes>{typestr}</span>
            <span class="purl-sep">/</span>
            <span class="purl-namespace">{namespace}</span>
            <span class="purl-sep">/</span>
            <span class="purl-name">{name}</span>
            { move || {
                version.get().is_some().then(|| {
                    view! {
                        <span class="purl-sep">@</span>
                        <span class="purl-version">{version}</span>
                    }
                })
            }}
            { move || {
                qualifiers.get().is_some().then(|| {
                    view! {
                        <span class="purl-sep">?</span>
                        <span class="purl-qualifiers">{move || qualifiers.get()}</span>
                    }
                })
            }}
            {move || {
                subpath.get().is_some().then(|| {
                    view! {
                        <span class="purl-sep">#</span>
                        <span class="purl-subpath">{subpath}</span>
                    }
                })
            }}
            // {subpath_rendered()}
        </div>
        { move || {
            let (class, headline, message) = match eval_type() {
                EvalResult::Verified(s) => ("verified", "verified", s),
                EvalResult::ProbablyOk(s) => ("ok", "ok", s),
                EvalResult::AtLeastValid(s) => ("valid", "valid", s),
                EvalResult::Invalid(s) => ("invalid", "invalid", s),
            };
            view !{
                <div class=format!("explanation-box {class}")>
                    <span class="headline">{headline}</span>
                    <span class="explanation">{message}</span>
                </div>
            }
        }}
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    leptos::mount_to_body(|| view! { <App/> })
}
