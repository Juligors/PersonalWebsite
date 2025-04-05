use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

use crate::{cv_page::CvPage, home_page::HomePage};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <link rel="icon" href="/favicon.png" type="image/x-icon" />
                <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.5.0/css/all.min.css" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>, id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/webella.css" />
        <Title text="Hello there 👋" />

        <Router>
            <NavBar/>
            <main class="container">
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=path!("/") view=HomePage />
                    <Route path=path!("/cv") view=CvPage />
                </Routes>
            </main>
            <Footer/>
        </Router>

    }
}

#[component]
fn NavBar() -> impl IntoView {
    view! {
        <nav>
            <div id="links">
                <a href="/">
                    <i class="fas fa-house" />"Home"
                </a>
                <a href="/cv" target="_blank">
                    <i class="fas fa-file" /> "CV"
                </a>
                <a href="https://github.com/Juligors" target="_blank">
                    <i class="fab fa-github" /> "GitHub"
                </a>
                <a href="https://linkedin.com/in/gorski-julian/" target="_blank">
                    <i class="fab fa-linkedin"></i> "LinkedIn"
                </a>
            </div>
            <div id="theme-picker">
                <ThemePicker/>
            </div>
        </nav>
    }
}

#[component]
fn Footer() -> impl IntoView {
    view! {
        <footer>
            <p>
                "Website made with"
                <a href="https://www.rust-lang.org/" target="_blank" rel="noopener noreferrer">
                    " Rust"
                </a>" and"<a href="https://leptos.dev/" target="_blank" rel="noopener noreferrer">
                    " Leptos"
                </a>.
            </p>
        </footer>
    }
}

#[component]
fn ThemePicker() -> impl IntoView {
    view! {
    <div id="theme-switcher">
        <button data-theme="dark" aria-pressed="true">Dark</button>
        <button data-theme="light" aria-pressed="false">Light</button>
    </div>
    }
}
