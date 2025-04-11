use chrono::{DateTime, Utc};
use leptos::prelude::*;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};

use crate::game_of_life_page::{board::Board, INITIAL_HEIGHT, INITIAL_WIDTH};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ship {
    #[serde(rename = "_id")]
    pub id: String,
    name: String,
    creation_date: DateTime<Utc>,
    cells: Vec<CellData>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CellData {
    x: usize,
    y: usize,
    is_alive: bool,
}

#[component]
pub fn ShipListComponent() -> impl IntoView {
    let (count, _) = signal(0);

    let ships = Resource::new(
        move || count.get(),
        |_| async { get_ships().await.unwrap_or_else(|_| vec![]) },
    );

    view! {
        <Suspense fallback=|| view! { <p>"Loading ships..."</p> }>
            {move || ships.get().map(|ship_list| view! {
                <table>
                    <thead>
                        <tr>
                            <th>"Name"</th>
                            <th>"Creation Date"</th>
                            <th>"Actions"</th>
                        </tr>
                    </thead>
                    <tbody>
                        { ship_list.iter().map(|ship| view! { <ShipRow ship=ship.clone() /> }).collect::<Vec<_>>()}
                    </tbody>
                </table>
            })}
        </Suspense>
    }
}

#[component]
fn ShipRow(ship: Ship) -> impl IntoView {
    let (dialog_open, set_dialog_open) = signal(false);

    view! {
        <tr>
            <td>{ship.name.into_view()}</td>
            <td>{ship.creation_date.format("%Y-%m-%d %H:%M:%S").to_string().into_view()}</td>
            <td>
                <button on:click=move |_| set_dialog_open.set(true)>
                    "View Drawing"
                </button>
                <Show when=move || dialog_open.get()>
                    <dialog open>
                        <button on:click=move |_| set_dialog_open.set(false)>
                            "Close"
                        </button>
                        <ShipDrawing cells=ship.cells.clone() />
                    </dialog>
                </Show>
            </td>
        </tr>
    }
}

#[component]
pub fn ShipDrawing(cells: Vec<CellData>) -> impl IntoView {
    let mut board = Board::new(INITIAL_WIDTH, INITIAL_HEIGHT);
    for CellData { x, y, is_alive } in cells {
        board.cells[y][x] = is_alive;
    }

    let cells = move || {
        board
            .cells
            .iter()
            .map(|row| {
                row.iter()
                    .map(move |&is_alive| {
                        let color = move || if is_alive { "black" } else { "gray" };
                        view! {
                            <div class="cell" style:background-color=color></div>
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    };

    let grid_template_columns = move || format!("repeat({}, 20px)", board.width);

    view! {
        <div class="board" style:grid-template-columns=grid_template_columns>{cells}</div>
    }
}

#[server]
async fn get_ships() -> Result<Vec<Ship>, ServerFnError> {
    use mongodb::{bson::doc, Client, Collection};

    let username = std::env::var("MONGO_USERNAME")?;
    let password = std::env::var("MONGO_PASSWORD")?;
    let uri = format!("mongodb+srv://{}:{}@cluster0.vbddlih.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0", username, password);

    let collection: Collection<Ship> = Client::with_uri_str(uri)
        .await?
        .database("game_of_life")
        .collection("ships");

    let mut cursor = collection.find(doc! {}).await?;
    let mut ships = Vec::new();
    while cursor.advance().await? {
        if let Ok(ship) = cursor.deserialize_current() {
            ships.push(ship);
        }
    }

    Ok(ships)
}

#[server]
pub async fn save_ship(board: Board) -> Result<(), ServerFnError> {
    use mongodb::{Client, Collection};

    let ship = Ship {
        id: nanoid!(),
        name: "nothing for now".into(),
        creation_date: chrono::offset::Utc::now(),
        cells: board
            .cells
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x, &is_alive)| CellData { x, y, is_alive })
                    .collect::<Vec<_>>()
            })
            .collect(),
    };

    let username = std::env::var("MONGO_USERNAME")?;
    let password = std::env::var("MONGO_PASSWORD")?;
    let uri = format!("mongodb+srv://{}:{}@cluster0.vbddlih.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0", username, password);

    let collection: Collection<Ship> = Client::with_uri_str(uri)
        .await?
        .database("game_of_life")
        .collection("ships");

    collection.insert_one(ship).await?;

    Ok(())
}
