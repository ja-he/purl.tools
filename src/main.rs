use leptos::*;
use leptos_meta::{provide_meta_context, Title};

#[component]
fn App() -> impl IntoView {
    let (devstatus_accepted, set_devstatus_accepted) = create_signal(false);
    let (light_theme, set_light_theme) = create_signal(true);
    provide_meta_context();

    view! {
        <Title text="purl Builder"/>
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
                    <Show
                        when=move || devstatus_accepted.get()
                        fallback=move || view! {
                            <p class="disclaimer-note">
                                "This is in development. "
                                "That means, this doesn't do much for now..."
                            </p>
                            <button id="devstatus-accept-button" on:click=move |_| { set_devstatus_accepted.set(true) }>
                                "Accept"
                            </button>
                        }>
                        <p class="disclaimer-note">
                            "So like I said before, its in development, so right now there's nothing more than this here to show."
                        </p>
                    </Show>
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

fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}
