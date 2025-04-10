use leptos::{prelude::*, task::spawn_local};

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <MongoTest />
        <Intro/>
        <Projects/>
        <QnA/>
    }
}

#[component]
fn Intro() -> impl IntoView {
    view! {
        <h1>"Hi 👋"</h1>
        <p class="section">
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
        <p class="section">
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
