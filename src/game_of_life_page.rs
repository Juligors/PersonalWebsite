use std::time::Duration;

use board::Board;
use create_ship_dialog::CreateShipDialog;
use leptos::{ev::SubmitEvent, html, prelude::*};
use ship::{ShipDrawing, ShipListComponent};

mod board;
mod create_ship_dialog;
mod ship;

#[component]
pub fn GameOfLifePage() -> impl IntoView {
    view! {
        <GameOfLife />
    }
}
pub const INITIAL_WIDTH: usize = 10;
const MIN_WIDTH: usize = 3;
const MAX_WIDTH: usize = 30;
pub const INITIAL_HEIGHT: usize = 10;
const MIN_HEIGHT: usize = 3;
const MAX_HEIGHT: usize = 30;

const INTERVAL_DURATION: Duration = Duration::from_millis(500);

#[component]
fn GameOfLife() -> impl IntoView {
    let (width, set_width) = signal(INITIAL_WIDTH);
    let (height, set_height) = signal(INITIAL_HEIGHT);
    let width_input_el: NodeRef<html::Input> = NodeRef::new();
    let height_input_el: NodeRef<html::Input> = NodeRef::new();
    let (board, set_board) = signal(Board::new(INITIAL_WIDTH, INITIAL_HEIGHT));

    let (running, set_running) = signal(false);
    let interval_handle = StoredValue::new_local(None::<IntervalHandle>);

    // let start = move || {
    //     set_running.set(true);
    //     let handle = set_interval_with_handle(
    //         move || {
    //             set_board.update(|board| {
    //                 *board = compute_next_generation(board, width.get(), height.get())
    //             })
    //         },
    //         INTERVAL_DURATION,
    //     )
    //     .expect("Failed to create interval");
    //     interval_handle.set_value(Some(handle));
    // };

    // let stop = move || {
    //     set_running.set(false);
    //     if let Some(handle) = interval_handle.get_value() {
    //         handle.clear();
    //         interval_handle.set_value(None);
    //     }
    // };

    // let start_stop = move || {
    //     if running.get() {
    //         stop();
    //     } else {
    //         start();
    //     }
    // };

    // let reset = move || {
    //     stop();
    //     set_board.set(create_board(width.get(), height.get()));
    // };

    let (dialog_is_open, set_dialog_is_open) = signal(false);

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
        set_board.set(Board::new(new_width, new_height));
    };

    view! {
        <h1>"Game of Life"</h1>

        <CreateShipDialog dialog_is_open=dialog_is_open set_dialog_is_open=set_dialog_is_open board=board set_board=set_board width=width/>

        <form on:submit=on_submit>
            <label for="width">"Width: "</label>
            <input id="width" type="number" value=width node_ref=width_input_el />

            <label for="height">"Height: "</label>
            <input id="height" type="number" value=height node_ref=height_input_el />

            <input type="submit" value="Set" />
        </form>

        <ShipListComponent/>

        // <button on:click=move |_| start_stop()>{move || if running.get() {"Stop"} else {"Start"}}</button>
        // <button on:click=move |_| reset()>"Reset"</button>
    }
}
