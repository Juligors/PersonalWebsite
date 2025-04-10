use leptos::{ev::SubmitEvent, html, prelude::*};

#[component]
pub fn GameOfLifePage() -> impl IntoView {
    view! {
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
