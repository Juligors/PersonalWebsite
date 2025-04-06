use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <script type="module">{include_str!("load_script.js")}</script>

        <Intro/>
        <Projects/>
        <Bella/>
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
            " I'm passionate about programming and I'd like to share some of my projects and skills here."
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
                    "This is my Master's Thesis"
                    <i>" \"Evolution in multi-agent based systems. Theoretic and application-based aspects\"."</i>

                    " project that I've been working on for some time now. It's meant to be an agent simulation of environment with plants as basis of a food chain, followed by herbivorous, carnivorous and omnivorous animals. Like any environment model, it has to be a significant simplification in relation to th real world, especially since I'm working on it on my own. Nonetheless, I've managed to implement those elements so far:
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
                    "I update Bella version on this website whenever I make enough progress to warrant it."
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
        <h1>"Projects"</h1>
        <p>
            "Some of the more interesting projects I've done during my university studies. You can check all of them and more on my"
            <a href="https://github.com/Juligors" target="_blank">" GitHub."</a>
        </p>
        <ProjectCard
            name="Bella"
            description="A multi-agent ecosystem simulation built with Rust and Bevy. Scroll down to see and test it out 😉"
            github="https://github.com/Juligors/Bella"
            tech_stack=vec!["Rust", "Bevy", "ECS", "WASM"]
        />
        <ProjectCard
            name="AGH Coin"
            description="Cryptocurrency with Proof Of Stake consensus algorithm. This projects contains blockchain itself, desktop wallet and desktop explorer."
            github="https://github.com/Juligors/AGH_Sem6_ProjectStudio2"
            tech_stack=vec!["C#", ".NET Core", "gRPC", "LiteDB", "CLI"]
        />
        <ProjectCard
            name="This website"
            description="It's made with Rust, HTML and CSS with a sprinkle of JavaScript. It's dockerized to make it easier to host it on Heroku."
            github="https://github.com/Juligors/PersonalWebsite"
            tech_stack=vec!["Rust", "Leptos", "WASM", "JavaScript", "HTML", "SCSS", "Docker"]
        />
        <ProjectCard
            name="Genetic programming framework"
            description="Project implementing a Genetic Programming framework, designed to evolve programs written in a custom programming language. These programs are generated, evaluated, and refined over successive generations to solve a given problem or optimize a specific objective."
            github="https://github.com/Juligors/AGH_Sem9_AdvancedPythonProgramming"
            tech_stack=vec!["Python", "Antlr", "Docker"]
        />
        <ProjectCard
            name="Mel"
            description="My Engineering Thesis, Agent based simulation game, supporting gameplay through both desktop application and network interface based on gRPC"
            github="https://github.com/Juligors/Mel"
            tech_stack=vec!["C#", ".NET Core", "MonoGame", "gRPC"]
        />
    }
}

#[component]
fn ProjectCard(
    name: &'static str,
    description: &'static str,
    github: &'static str,
    tech_stack: Vec<&'static str>,
) -> impl IntoView {
    view! {
        <a href={github} target="_blank" class="project-card" role="group">
            <div class="card-inner">
                <h2>{name}</h2>
                <p>{description}</p>

                <div class="tech-tags">
                    { tech_stack.into_iter().map(|tech| view! {
                        <span class="tag">{tech}</span>
                    }).collect_view() }
                </div>
            </div>
        </a>
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
