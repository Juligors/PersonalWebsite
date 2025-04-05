use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
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
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=path!("/") view=HomePage />
                    <Route path=path!("/projects/bella") view=HomePage />
                </Routes>
            </main>
            <Footer/>
        </Router>

    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <script type="module">{include_str!("load_script.js")}</script>

        <Intro/>
        <Bella/>
        <Projects/>
        <QnA/>
    }
}

#[component]
fn Bella() -> impl IntoView {
    view! {
        <div id="section-bella">
            <div>
                <p>
                    <h3>"You can:"</h3>
                    <ul>
                        <li>"Move around with" <b>" right mouse button"</b></li>
                        <li>
                            <b>"Scroll"</b>
                            " to zoom in/out"
                        </li>
                        <li>"Reset with" <b>" 'R'"</b></li>
                        <li>"Pause with" <b>" 'P'"</b></li>
                        <li>
                            "Inspect and change any entity by choosing it with"
                            <b>" left mouse button"</b>
                        </li>
                    </ul>
                    <h3>
                        "There should be some plants near the"
                        <span style="color:blue">" water"</span> " :D"
                    </h3>
                </p>
            </div>
            <div class="simulation-container">
                <div id="loading-message">"Loading the simulation 😎 Please wait"</div>
                <canvas id="bevy-bella">
                    "Your browser does not support the canvas element :("
                </canvas>
            </div>
        </div>
    }
}

#[component]
fn QnA() -> impl IntoView {
    view! {
        <p id="section-qna">
            <div class="question-and-answer">
                <div class="question">"Cats or dogs"</div>
                <div class="answer">"Cats"</div>
            </div>
            <div class="question-and-answer">
                <div class="question">"Favourite language"</div>
                <div class="answer">"Rust"</div>
            </div>
            <div class="question-and-answer">
                <div class="question">"Favourite book"</div>
                <div class="answer">"Twilight"</div>
            </div>
            <div class="question-and-answer">
                <div class="question">"Favourite operating system"</div>
                <div class="answer">"Linux"</div>
            </div>
            <div class="question-and-answer">
                <div class="question">"Favourite editor"</div>
                <div class="answer">"VS Code with Vim Motions"</div>
            </div>
        </p>
    }
}

#[component]
fn Intro() -> impl IntoView {
    view! {
        <p id="section-intro">
            "Hi, I'm Julian Górski, an (almost) graduate student of"
            <a href="https://www.agh.edu.pl/en" target="_blank">" AGH"</a>
            " in"
            <a href="https://www.eaiib.agh.edu.pl/informatyka-i-systemy-inteligentne/" target="_blank">
                " IT and Intellighent Systems, specialization Artificial Intelligence and Data Analysis."
            </a>
             "I'm currently working on an agent simulation written in"
            <a href="https://www.rust-lang.org/" target="_blank">" Rust"</a>
            " with"
            <a href="https://bevyengine.org/" target="_blank">" Bevy."</a>
            "It's for my Master's Thesis titled /title. You can see my progress so far by interacting with embedded version of the simulation"
            <a href="#section-bella">" below."</a>
            "I update it on this website whenever I make enough progress to warrant it."
        </p>
    }
}

#[component]
fn Projects() -> impl IntoView {
    view! {
        <p id="section-projects">
            """Some of the more interesting projects I've done during my university studies.
            Jeśli gdzieś znajdę linki to info o tym, że uczestniczę w projekcie Porównania Platform Agentowych z Gliderem/h
            
            Format do wszystkich w stylu
            /description
            /h Github
            /solo or duo, link to Dominik's Github"""
        </p>
    }
}

#[component]
fn NavBar() -> impl IntoView {
    view! {
        <nav>
            <div id="links">
                <a href="#section-intro">"Introduction"</a>
                <a href="#section-qna">"Q&A"</a>
                <a href="#section-bella">"Bella"</a>
                <a href="#section-projects">"Projects"</a>
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
