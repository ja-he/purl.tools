use leptos::*;

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
fn Purl(
    typestr: ReadSignal<String>,
    namespace: ReadSignal<String>,
    name: ReadSignal<String>,
    version: ReadSignal<String>,
    qualifiers: ReadSignal<Option<String>>,
    subpath: ReadSignal<Option<String>>,
) -> impl IntoView {
    let qualifiers_rendered = move || {
        qualifiers.get().is_some().then(|| {
            view! {
                <span class="purl-sep">?</span>
                <span class="purl-qualifiers">{qualifiers}</span>
            }
        })
    };
    let subpath_rendered = move || {
        subpath.get().is_some().then(|| {
            view! {
                <span class="purl-sep">?</span>
                <span class="purl-subpath">{subpath}</span>
            }
        })
    };

    // abtract: scheme:type/namespace/name@version?qualifiers#subpath
    view! {
        <div class="purl">
            <span class="purl-scheme">"pkg"</span>
            <span class="purl-sep">:</span>
            <span class="purl-type">{typestr}</span>
            <span class="purl-sep">/</span>
            <span class="purl-namespace">{namespace}</span>
            <span class="purl-sep">/</span>
            <span class="purl-name">{name}</span>
            <span class="purl-sep">@</span>
            <span class="purl-version">{version}</span>
            {qualifiers_rendered()}
            {subpath_rendered()}
        </div>
    }
}

fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}
