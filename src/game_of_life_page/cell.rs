use leptos::prelude::*;

#[component]
pub fn Cell(
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
