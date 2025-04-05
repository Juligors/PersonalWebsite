use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <script type="module">{include_str!("load_script.js")}</script>

        <Intro/>
        <Bella/>
        <Projects/>
        <QnA/>
    }
}

#[component]
fn Intro() -> impl IntoView {
    view! {
        <h1>"Hi 👋"</h1>
        <p id="section-intro" class="section">
            "I'm Julian Górski, a student of"
            <a href="https://www.agh.edu.pl/en" target="_blank">" AGH"</a>
            " in"
            <a href="https://www.eaiib.agh.edu.pl/informatyka-i-systemy-inteligentne/" target="_blank">
                " IT and Intellighent Systems, specialization Artificial Intelligence and Data Analysis."
            </a>
             " I'm currently working on an agent simulation written in"
            <a href="https://www.rust-lang.org/" target="_blank">" Rust"</a>
            " with"
            <a href="https://bevyengine.org/" target="_blank">" Bevy."</a>
            " It's for my Master's Thesis titled"
            <i>" \"Evolution in multi-agent based systems. Theoretic and application-based aspects\"."</i>
            " You can see my progress so far by interacting with embedded version of the simulation"
            <a href="#section-bella">" below."</a>
            " I update it on this website whenever I make enough progress to warrant it."
        </p>
    }
}

#[component]
fn Bella() -> impl IntoView {
    view! {
        <div id="section-bella" class="section">
            <h1>"Bella"</h1>
            <div>
                <p>
                    "This is my Master's Thesis project that I've been working on for some time now. It's meant to be a simulation of environment with plants as basis of a food chain, followed by herbivorous, carnivorous and omnivorous animals. Like any environment model, it has to be a significant simplification in relation to th real world, especially since I'm working on it on my own. Nonetheless, I've managed to implement those elements so far:
                    "
                    <ul>
                        <li>
                            "Procedurally generated terrain with"
                            <ul>
                                <li>"different biomes with different parameters"</li>
                                <li>"humidity and temperature propagation"</li>
                                <li>"nutrients levels"</li>
                            </ul>
                        </li>
                        <li>"Reproducing and aging organisms, such as"
                            <ul>
                                <li>"plants"</li>
                                <li>"animals with AI, capable of moving and attacking"</li>
                            </ul>
                        </li>
                        <li>"Night and day cycle"</li>
                        <li>"Carcass system for dead organisms"</li>
                        <li>"Dominant and recessive genes"</li>
                        <li>"Data collection system"</li>
                        <li>"Ability to pause or restart simulation"</li>
                        <li>"Debugging tools and gizmos"</li>
                        <li>
                            "Multiple builds"
                            <ul>
                                <li>"Native windowed"</li>
                                <li>"WASM-based windowed build for browser you're seeing now"</li>
                                <li>"Native headless, for maximum performance for experiments"</li>
                            </ul>
                        </li>
                    </ul>
                    <h3>"If you'd like to take a closer look at Bella you can:"</h3>
                    <ul>
                        <li>"Move around with" <b>" right mouse button"</b></li>
                        <li><b>"Scroll"</b>" to zoom in/out"</li>
                        <li>"Reset with" <b>" 'R'"</b></li>
                        <li>"Pause with" <b>" 'P'"</b></li>
                        <li>"Inspect any entity and changing its components by choosing it with" <b>" left mouse button"</b></li>
                    </ul>
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
fn Projects() -> impl IntoView {
    view! {
        <h1>"Other projects"</h1>
        <p id="section-projects" class="section">
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
fn QnA() -> impl IntoView {
    view! {
        <h1>"About me"</h1>
        <p id="section-qna" class="section">
            <div class="question-and-answer">
                <div class="question">"Favourite language"</div>
                <div class="answer">"Rust, but I'm fine using anything"</div>
            </div>
            <div class="question-and-answer">
                <div class="question">"Favourite editor"</div>
                <div class="answer">"VS Code with Vim Motions"</div>
            </div>
            <div class="question-and-answer">
                <div class="question">"Cats or dogs"</div>
                <div class="answer">"Cats"</div>
            </div>
            <div class="question-and-answer">
                <div class="question">"Favourite operating system"</div>
                <div class="answer">"Linux"</div>
            </div>
            <div class="question-and-answer">
                <div class="question">"Favourite book"</div>
                <div class="answer">"Twilight"</div>
            </div>
        </p>
    }
}
