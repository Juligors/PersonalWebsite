use leptos::prelude::*;

use crate::game_of_life_page::cell::Cell;

#[component]
pub fn Board(
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
