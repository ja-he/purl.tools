use itertools::Itertools;
use leptos::*;

use crate::purl_data::PurlComponent;

mod purl_data;

#[macro_use]
extern crate lazy_static;
mod purl_eval;

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
    let (typex, set_typex) = create_signal(purl_data::PurlType::Github);
    let (namespace, set_namespace) = create_signal(purl_data::PurlNamespace::new_naive("ja-he"));
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
                    set_typex(purl_data::PurlType::new(&new_value));
                }>
                    {
                        purl_data::PURL_TYPES.iter()
                            .map(|t| view! {
                                <option
                                    class={ match t.status() {
                                        purl_data::PurlTypeStatus::WellKnown => "option-well-known",
                                        purl_data::PurlTypeStatus::Proposed => "option-proposed",
                                        purl_data::PurlTypeStatus::Other => "option-other", // this case would not happen, normally
                                    }}
                                    value=t.to_string()
                                    selected=move || typex().to_string() == t.to_string()
                                >
                                    {t.to_string()}
                                </option>
                            })
                            .collect_view()
                     }
                </select>
        }
        .into_any(),
        InputOption::Raw => view! {
                <input class="purl-component-input" type="text"
                    on:input=move |ev| { set_typex(purl_data::PurlType::new(&event_target_value(&ev))); }
                    prop:value={move || typex().to_string()}
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

    let eval_type = move || purl_eval::eval_purl_type(typex());
    let (eval_type_result, set_eval_type_result) = create_signal("verified".to_string());
    let (eval_type_result_explanation, set_eval_type_result_explanation) =
        create_signal("well-known identifier".to_string());
    create_effect(move |_| {
        let new = eval_type().summary();
        let old = eval_type_result();
        if old != new {
            set_eval_type_result(new);
        }
        let new = eval_type().explanation();
        let old = eval_type_result_explanation();
        if old != new {
            set_eval_type_result_explanation(new);
        }
    });

    let eval_namespace = move || purl_eval::eval_purl_namespace(namespace(), typex());
    let (eval_namespace_result, set_eval_namespace_result) = create_signal("verified".to_string());
    let (eval_namespace_result_explanation, set_eval_namespace_result_explanation) =
        create_signal("well-known identifier".to_string());
    create_effect(move |_| {
        let new = eval_namespace().summary();
        let old = eval_namespace_result();
        if old != new {
            set_eval_namespace_result(new);
        }
        let new = eval_namespace().explanation();
        let old = eval_namespace_result_explanation();
        if old != new {
            set_eval_namespace_result_explanation(new);
        }
    });

    let eval_name = move || purl_eval::eval_purl_name(name(), namespace(), typex());
    let (eval_name_result, set_eval_name_result) = create_signal("verified".to_string());
    let (eval_name_result_explanation, set_eval_name_result_explanation) =
        create_signal("well-known identifier".to_string());
    create_effect(move |_| {
        let new = eval_name().summary();
        let old = eval_name_result();
        if old != new {
            set_eval_name_result(new);
        }
        let new = eval_name().explanation();
        let old = eval_name_result_explanation();
        if old != new {
            set_eval_name_result_explanation(new);
        }
    });

    let get_type_explanation_box_class =
        move || format!("explanation-box {result}", result = eval_type_result());
    let get_namespace_explanation_box_class =
        move || format!("explanation-box {result}", result = eval_namespace_result());
    let get_name_explanation_box_class =
        move || format!("explanation-box {result}", result = eval_name_result());

    view! {
        <div id="input-form">
            <div class="input-row">
                <span class="input-label">"type"</span>
                {get_type_input_field}
                <button
                    id="type-input-toggle-button"
                    class="purl-input-options-button"
                    on:click=cycle_type_input_option>
                    <Show when=move || { type_input_option() == InputOption::Select } fallback=|| view! {<phosphor_leptos::Cursor class="button-icon" weight=phosphor_leptos::IconWeight::Bold />} >
                        <phosphor_leptos::PencilSimple class="button-icon" weight=phosphor_leptos::IconWeight::Bold />
                    </Show>
                </button>
            </div>
            <div class="input-row">
                <span class="input-label">"namespace"</span>
                <input class="purl-component-input" type="text"
                    on:input=move |ev| {
                        set_namespace(purl_data::PurlComponent::new_naive(&event_target_value(&ev)));
                    }
                    prop:value={move || namespace().join("/")}
                />
            </div>
            <div class="input-row">
                <span class="input-label">"name"</span>
                <input class="purl-component-input" type="text"
                    on:input=move |ev| { set_name(urlencoding::encode(&event_target_value(&ev)).into_owned()); }
                    prop:value=move || urlencoding::decode(&name()).unwrap_or_default().into_owned()
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
                    prop:value={move || version().unwrap_or_default()}
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
                    prop:value={move || qualifiers().unwrap_or_default()}
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
                    prop:value={move || subpath().unwrap_or_default()}
                />
            </div>
        </div>

        <Purl
            typex={typex}
            eval_type_result={eval_type_result}
            namespace={namespace}
            eval_namespace_result={eval_namespace_result}
            name={name}
            eval_name_result={eval_name_result}
            version={version}
            qualifiers={qualifiers}
            subpath={subpath}
        />

        <div class="explanation-box-wrapper">
            <div class={get_type_explanation_box_class}>
                {move || match eval_type_result().as_str() {
                    "verified" => view!{<phosphor_leptos::Checks class="explanation-icon verified" weight=phosphor_leptos::IconWeight::Bold />},
                    "ok" => view!{<phosphor_leptos::Check class="explanation-icon ok" weight=phosphor_leptos::IconWeight::Bold />}       ,
                    "valid" => view!{<phosphor_leptos::Question class="explanation-icon valid" weight=phosphor_leptos::IconWeight::Bold />} ,
                    "invalid" => view!{<phosphor_leptos::Warning class="explanation-icon invalid" weight=phosphor_leptos::IconWeight::Bold />},
                    _ => view!{<phosphor_leptos::Warning class="explanation-icon error" weight=phosphor_leptos::IconWeight::Duotone />},
                }}
                <span class="headline">{eval_type_result}</span>
                <span class="explanation">{eval_type_result_explanation}</span>
            </div>
            <div class={get_namespace_explanation_box_class}>
                {move || match eval_namespace_result().as_str() {
                    "verified" => view!{<phosphor_leptos::Checks class="explanation-icon verified" weight=phosphor_leptos::IconWeight::Bold />},
                    "ok" => view!{<phosphor_leptos::Check class="explanation-icon ok" weight=phosphor_leptos::IconWeight::Bold />}       ,
                    "valid" => view!{<phosphor_leptos::Question class="explanation-icon valid" weight=phosphor_leptos::IconWeight::Bold />} ,
                    "invalid" => view!{<phosphor_leptos::Warning class="explanation-icon invalid" weight=phosphor_leptos::IconWeight::Bold />},
                    _ => view!{<phosphor_leptos::Warning class="explanation-icon error" weight=phosphor_leptos::IconWeight::Duotone />},
                }}
                <span class="headline">{eval_namespace_result}</span>
                <span class="explanation">{eval_namespace_result_explanation}</span>
            </div>
            <div class={get_name_explanation_box_class}>
                {move || match eval_name_result().as_str() {
                    "verified" => view!{<phosphor_leptos::Checks class="explanation-icon verified" weight=phosphor_leptos::IconWeight::Bold />},
                    "ok" => view!{<phosphor_leptos::Check class="explanation-icon ok" weight=phosphor_leptos::IconWeight::Bold />}       ,
                    "valid" => view!{<phosphor_leptos::Question class="explanation-icon valid" weight=phosphor_leptos::IconWeight::Bold />} ,
                    "invalid" => view!{<phosphor_leptos::Warning class="explanation-icon invalid" weight=phosphor_leptos::IconWeight::Bold />},
                    _ => view!{<phosphor_leptos::Warning class="explanation-icon error" weight=phosphor_leptos::IconWeight::Duotone />},
                }}
                <span class="headline">{eval_name_result}</span>
                <span class="explanation">{eval_name_result_explanation}</span>
            </div>
        </div>
    }
}

#[component]
fn EvalIcon(eval_result: ReadSignal<purl_eval::EvalResult>) -> impl IntoView {
    view! {
        {
            match eval_result() {
                purl_eval::EvalResult::Verified(_)     => view!{<phosphor_leptos::Checks class="explanation-icon verified" weight=phosphor_leptos::IconWeight::Bold />},
                purl_eval::EvalResult::ProbablyOk(_)   => view!{<phosphor_leptos::Check class="explanation-icon ok" weight=phosphor_leptos::IconWeight::Bold />}       ,
                purl_eval::EvalResult::AtLeastValid(_) => view!{<phosphor_leptos::Question class="explanation-icon valid" weight=phosphor_leptos::IconWeight::Bold />} ,
                purl_eval::EvalResult::Invalid(_)      => view!{<phosphor_leptos::Warning class="explanation-icon invalid" weight=phosphor_leptos::IconWeight::Bold />},
            }
        }
    }
}

#[component]
fn Purl(
    typex: ReadSignal<purl_data::PurlType>,
    eval_type_result: ReadSignal<String>,
    namespace: ReadSignal<Vec<String>>,
    eval_namespace_result: ReadSignal<String>,
    name: ReadSignal<String>,
    eval_name_result: ReadSignal<String>,
    version: ReadSignal<Option<String>>,
    qualifiers: ReadSignal<Option<String>>,
    subpath: ReadSignal<Option<String>>,
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
                <span class={get_purl_namespace_classes}>
                    {namespace_view}
                </span>
            </Show>
        }
    };

    // abtract: scheme:type/namespace/name@version?qualifiers#subpath
    view! {
        <div class="purl">
            <span class="purl-scheme">"pkg"</span>
            <span class="purl-sep">:</span>
            <span class=get_purl_type_classes>{move || typex.with(purl_data::PurlType::to_string)}</span>
            {namespace_and_leading_slash}
            <span class="purl-sep">/</span>
            <span class=get_purl_name_classes>{name}</span>
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
        </div>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    leptos::mount_to_body(|| view! { <App/> })
}
