mod page;

use dioxus::prelude::*;

use page::{Home, Portfolio};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/portfolio")]
    Portfolio {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const RESUME: Asset = asset!("/assets/resume.pdf");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

/// Shared navbar component
#[component]
fn Navbar() -> Element {
    rsx! {
        nav { class: "navbar bg-base-100 shadow-sm",
            id: "navbar",
            Link { class: "btn btn-ghost text-xl",
                to: Route::Home {},
                "Home"
            }
            Link { class: "btn btn-ghost text-xl",
                to: Route::Portfolio {},
                "Portfolio"
            }
        }

        Outlet::<Route> {}
    }
}
