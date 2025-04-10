use leptos::{ev::SubmitEvent, html, prelude::*, task::spawn_local};

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <script type="module">{include_str!("load_script.js")}</script>

        <GameOfLife />
        <MongoTest />
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

#[component]
fn MongoTest() -> impl IntoView {
    view! {
        <button on:click=move |_|{
            spawn_local(async {
                call_me().await;
            })
        }
        >"Hi there, click me baby!"</button>
    }
}

#[server]
async fn call_me() -> Result<(), ServerFnError> {
    use mongodb::{
        bson::{doc, Document},
        Client, Collection,
    };

    let username = std::env::var("MONGO_USERNAME")?;
    let password = std::env::var("MONGO_PASSWORD")?;

    let uri = format!("mongodb+srv://{}:{}@cluster0.vbddlih.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0", username, password);
    let client = Client::with_uri_str(uri).await?;
    let database = client.database("sample_mflix");
    let my_coll: Collection<Document> = database.collection("movies");
    let my_movie = my_coll
        .find_one(doc! { "title": "The Perils of Pauline" })
        .await?;

    println!("Found a movie:\n{:#?}", my_movie);

    Ok(())
}

#[component]
fn GameOfLife() -> impl IntoView {
    const INITIAL_WIDTH: usize = 10;
    const MIN_WIDTH: usize = 3;
    const MAX_WIDTH: usize = 30;
    const INITIAL_HEIGHT: usize = 10;
    const MIN_HEIGHT: usize = 3;
    const MAX_HEIGHT: usize = 30;

    let (width, set_width) = signal(INITIAL_WIDTH);
    let (height, set_height) = signal(INITIAL_HEIGHT);

    let width_input_el: NodeRef<html::Input> = NodeRef::new();
    let height_input_el: NodeRef<html::Input> = NodeRef::new();

    let (board, set_board) = signal(vec![vec![false; INITIAL_WIDTH]; INITIAL_HEIGHT]);

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let new_width = width_input_el
            .get()
            .expect("<input> of width should definitely be Some at this point")
            .value()
            .parse()
            .unwrap_or(INITIAL_WIDTH)
            .clamp(MIN_WIDTH, MAX_WIDTH);
        let new_height = height_input_el
            .get()
            .expect("<input> of height should definitely be Some at this point")
            .value()
            .parse()
            .unwrap_or(INITIAL_WIDTH)
            .clamp(MIN_HEIGHT, MAX_HEIGHT);

        set_width.set(new_width);
        set_height.set(new_height);
        set_board.set(vec![vec![false; new_width]; new_height]);
    };

    view! {
        <h1>"Game of Life"</h1>
        <form on:submit=on_submit>
            <label for="width">"Width: "</label>
            <input id="width" type="number" value=width node_ref=width_input_el />

            <label for="height">"Height: "</label>
            <input id="height" type="number" value=height node_ref=height_input_el />

            <input type="submit" value="Set" />
        </form>

        <Board board=board width=width/>
    }
}

#[component]
fn Board(board: ReadSignal<Vec<Vec<bool>>>, width: ReadSignal<usize>) -> impl IntoView {
    let cells = move || {
        board
            .get()
            .iter()
            .map(|row| {
                row.iter()
                    .map(move |&is_alive| {
                        view! {
                            <Cell is_alive=is_alive />
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    };

    let grid_template_columns = move || format!("repeat({}, 20px)", width.get());

    view! {
        <div class="board" style:grid-template-columns=grid_template_columns>{cells}</div>
    }
}

#[component]
fn Cell(is_alive: bool) -> impl IntoView {
    let color = move || if is_alive { "black" } else { "gray" };

    view! {
        <div class="cell" style:background-color=color></div>
    }
}
