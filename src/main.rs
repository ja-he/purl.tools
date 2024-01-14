use leptonic::{
    alert::{Alert, AlertVariant},
    link::{LinkExt, LinkExtTarget},
    root::Root,
    theme::{LeptonicTheme, ThemeToggle}, button::{Button, ButtonVariant},
};
use leptos::*;
use leptos_meta::{provide_meta_context, Title};

#[component]
fn App() -> impl IntoView {
    let (devstatus_accepted, set_devstatus_accepted) = create_signal(false);
    provide_meta_context();

    view! {
        <Title text="purl Builder"/>
        <Root default_theme=LeptonicTheme::default()>
            <div id="full-page">
                <div id="cool-control-stuff">
                    <div  id="main-title">purl Builder</div>
                    <div  id="theme-toggle">
                        <ThemeToggle off=LeptonicTheme::Light on=LeptonicTheme::Dark/>
                    </div>
                </div>
                <div id="main-content">
                    <Show
                        when=move || devstatus_accepted.get()
                        fallback=move || view! {
                            <Alert variant=AlertVariant::Warn title=|| "In Development".into_view()>
                                "This is in development."
                                "That means, this doesn't do much for now..."
                            </Alert>
                            <Button id="devstatus-accept-button" on_click=move |_| { set_devstatus_accepted.set(true) } variant=ButtonVariant::Outlined>
                                "Accept"
                            </Button>
                        }>
                        <Alert variant=AlertVariant::Success title=|| "Good to have you!".into_view()>
                            "So like I said before, its in development, so right now there's nothing more than this here to show."
                        </Alert>
                    </Show>
                </div>
                <div id="footer">
                    "Created by "
                    <LinkExt href="https://hensel.dev" target=LinkExtTarget::Blank>
                        "Jan Hensel"
                    </LinkExt>
                    "."
                </div>
            </div>
        </Root>
    }
}

fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}
