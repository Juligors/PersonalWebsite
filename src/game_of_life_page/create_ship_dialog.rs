use leptos::{prelude::*, task::spawn_local};

use crate::game_of_life_page::{board::BoardComponent, ship::save_ship};

use super::board::Board;

#[component]
pub fn CreateShipDialog(
    dialog_is_open: ReadSignal<bool>,
    set_dialog_is_open: WriteSignal<bool>,
    board: ReadSignal<Board>,
    set_board: WriteSignal<Board>,
    width: ReadSignal<usize>,
) -> impl IntoView {
    view! {
        <button on:click=move |_| set_dialog_is_open.set(true)>"Create a ship!"</button>
        <dialog open=move || dialog_is_open.get()>
        <button on:click=move |_| set_dialog_is_open.set(false)>"Close"</button>
            <button on:click=move |_|{
                let board = board.get();
                spawn_local(async { let _ = save_ship(board).await; });
                set_dialog_is_open.set(false);
                set_board.set(Board::new_for_drawing());
            }>"Save ship"</button>
            <BoardComponent board=board set_board=set_board width=width/>
        </dialog>
    }
}
