use std::time::Duration;

use board::Board;
use leptos::{ev::SubmitEvent, html, prelude::*, task::spawn_local};

mod board;
mod cell;

#[component]
pub fn GameOfLifePage() -> impl IntoView {
    view! {
        <MongoTest />
        <GameOfLife />
    }
}
const INITIAL_WIDTH: usize = 10;
const MIN_WIDTH: usize = 3;
const MAX_WIDTH: usize = 30;
const INITIAL_HEIGHT: usize = 10;
const MIN_HEIGHT: usize = 3;
const MAX_HEIGHT: usize = 30;

const INTERVAL_DURATION: Duration = Duration::from_millis(500);

fn create_board(width: usize, height: usize) -> Vec<Vec<bool>> {
    vec![vec![false; width]; height]
}

#[component]
fn GameOfLife() -> impl IntoView {
    let (width, set_width) = signal(INITIAL_WIDTH);
    let (height, set_height) = signal(INITIAL_HEIGHT);
    let width_input_el: NodeRef<html::Input> = NodeRef::new();
    let height_input_el: NodeRef<html::Input> = NodeRef::new();
    let (board, set_board) = signal(create_board(INITIAL_WIDTH, INITIAL_HEIGHT));

    let (running, set_running) = signal(false);
    let interval_handle = StoredValue::new_local(None::<IntervalHandle>);

    let start = move || {
        set_running.set(true);
        let handle = set_interval_with_handle(
            move || {
                set_board.update(|board| {
                    *board = compute_next_generation(board, width.get(), height.get())
                })
            },
            INTERVAL_DURATION,
        )
        .expect("Failed to create interval");
        interval_handle.set_value(Some(handle));
    };

    let stop = move || {
        set_running.set(false);
        if let Some(handle) = interval_handle.get_value() {
            handle.clear();
            interval_handle.set_value(None);
        }
    };

    let start_stop = move || {
        if running.get() {
            stop();
        } else {
            start();
        }
    };

    let reset = move || {
        stop();
        set_board.set(create_board(width.get(), height.get()));
    };

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

        <button on:click=move |_| start_stop()>{move || if running.get() {"Stop"} else {"Start"}}</button>
        <button on:click=move |_| reset()>"Reset"</button>

        <Board board=board set_board=set_board width=width/>
    }
}

fn compute_next_generation(board: &Vec<Vec<bool>>, width: usize, height: usize) -> Vec<Vec<bool>> {
    let mut next_board = create_board(width, height);

    for row in 0..height {
        for col in 0..width {
            let live_neighbors = count_live_neighbors(board, row, col);
            next_board[row][col] = match (board[row][col], live_neighbors) {
                (true, 2) | (true, 3) => true,
                (false, 3) => true,
                _ => false,
            };
        }
    }

    next_board
}

fn count_live_neighbors(board: &[Vec<bool>], row: usize, col: usize) -> usize {
    let height = board.len() as isize;
    let width = board[0].len() as isize;
    let mut count = 0;

    for dr in -1..=1 {
        for dc in -1..=1 {
            if dr == 0 && dc == 0 {
                continue;
            }
            let new_row = row as isize + dr;
            let new_col = col as isize + dc;
            if new_row >= 0
                && new_row < height
                && new_col >= 0
                && new_col < width
                && board[new_row as usize][new_col as usize]
            {
                count += 1;
            }
        }
    }

    count
}

/////

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
