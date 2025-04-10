use leptos::{ev::SubmitEvent, html, prelude::*, task::spawn_local};

#[component]
pub fn GameOfLifePage() -> impl IntoView {
    view! {
        <MongoTest />
        <GameOfLife />
    }
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

        <Board board=board set_board=set_board width=width/>
    }
}

#[component]
fn Board(
    board: ReadSignal<Vec<Vec<bool>>>,
    set_board: WriteSignal<Vec<Vec<bool>>>,
    width: ReadSignal<usize>,
) -> impl IntoView {
    let cells = move || {
        board
            .get()
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(move |(x, &is_alive)| {
                        view! {
                            <Cell set_board=set_board x=x y=y is_alive=is_alive />
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
fn Cell(
    set_board: WriteSignal<Vec<Vec<bool>>>,
    x: usize,
    y: usize,
    is_alive: bool,
) -> impl IntoView {
    let on_click = move |_| {
        set_board.update(|board| {
            if let Some(row_vec) = board.get_mut(y) {
                if let Some(cell) = row_vec.get_mut(x) {
                    *cell = !*cell;
                }
            }
        });
    };

    let color = move || if is_alive { "black" } else { "gray" };

    view! {
        <div class="cell" style:background-color=color on:click=on_click></div>
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
