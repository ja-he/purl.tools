use leptos::*;

mod purl_data;

#[component]
fn App() -> impl IntoView {
    let (light_theme, set_light_theme) = create_signal(true);
    leptos_meta::provide_meta_context();

    view! {
        <leptos_meta::Title text="purl Builder"/>
            <div id="full-page">
                <div id="header">
                    <div  id="main-title">purl Builder</div>
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

#[component]
fn MainContent() -> impl IntoView {
    // abtract: scheme:type/namespace/name@version?qualifiers#subpath
    // eg.:     pkg:github/package-url/purl-spec@244fd47e07d1004f0aed9c
    let (typestr, set_typestr) = create_signal("github".to_string());
    let (namespace, set_namespace) = create_signal("ja-he".to_string());
    let (name, set_name) = create_signal("dayplan".to_string());
    let (version, set_version) = create_signal("v0.9.4".to_string());
    let (qualifiers, set_qualifiers) = create_signal(None);
    let (subpath, set_subpath) = create_signal(None);

    view! {
        <div id="input-form">
            <span class="input-label">"type"</span>
            <select on:change=move |ev| {
                let new_value = event_target_value(&ev);
                set_typestr(new_value);
            }>
                {
                    purl_data::PURL_TYPES.iter()
                        .map(|(type_option, choice_status)| view! {
                            <PurlTypeOption typestr is=type_option status=*choice_status/>
                        })
                        .collect_view()
                 }
            </select>
            <span class="input-label">"namespace"</span>
            <input type="text"
                on:input=move |ev| { set_namespace(event_target_value(&ev)); }
                prop:value=namespace
            />
            <span class="input-label">"name"</span>
            <input type="text"
                on:input=move |ev| { set_name(event_target_value(&ev)); }
                prop:value=name
            />
            <span class="input-label">"version"</span>
            <input type="text"
                on:input=move |ev| { set_version(event_target_value(&ev)); }
                prop:value=version
            />
            <span class="input-label">"qualifiers"</span>
            <input type="text"
                on:input=move |ev| { set_qualifiers(
                    if !event_target_value(&ev).is_empty() {
                        Some(event_target_value(&ev))
                    } else {
                        None
                    }
                ); }
                prop:value=""
            />
            <span class="input-label">"subpath"</span>
            <input type="text"
                on:input=move |ev| { set_subpath(
                    if !event_target_value(&ev).is_empty() {
                        Some(event_target_value(&ev))
                    } else {
                        None
                    }
                ); }
                prop:value=""
            />
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
    is: &'static str,
    typestr: ReadSignal<String>,
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

#[component]
fn Purl(
    typestr: ReadSignal<String>,
    namespace: ReadSignal<String>,
    name: ReadSignal<String>,
    version: ReadSignal<String>,
    qualifiers: ReadSignal<Option<String>>,
    subpath: ReadSignal<Option<String>>,
) -> impl IntoView {
    let get_purl_type_classes = {
        move || {
            format!(
                "{t} {status}",
                t = "purl-type",
                status = match purl_data::get_purl_type_status(&typestr.get()) {
                    purl_data::PurlTypeStatus::WellKnown => "identifier-well-known",
                    purl_data::PurlTypeStatus::Proposed => "identifier-proposed",
                    purl_data::PurlTypeStatus::Other => "identifier-other",
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
            <span class="purl-sep">@</span>
            <span class="purl-version">{version}</span>
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
    }
}

fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}
