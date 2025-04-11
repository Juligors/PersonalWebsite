use std::{rc::Rc, sync::Arc};

use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[component]
pub fn BoardComponent(
    board: ReadSignal<Board>,
    set_board: WriteSignal<Board>,
    width: ReadSignal<usize>,
) -> impl IntoView {
    let cells =
        move || {
            board
                .read()
                .cells
                .iter()
                .enumerate()
                .map(|(y, row)| {
                    row.iter()
                .enumerate()
                    .map(move |(x, &is_alive)| {
                        let color = move || if is_alive { "black" } else { "gray" };
                        let on_click = move |_| set_board.update(|board|{
                            board.cells[y][x] = !board.cells[y][x];
                        });

                        view! {
                            <div class="cell" style:background-color=color on:click=on_click></div>
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Board {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<bool>>,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Board {
        Board {
            width,
            height,
            cells: vec![vec![false; width]; height],
        }
    }

    pub fn compute_next_generation(&self) -> Board {
        let mut next_board = Board::new(self.width, self.height);

        for row in 0..self.height {
            for col in 0..self.width {
                let live_neighbors = self.count_live_neighbors(row, col);
                next_board.cells[row][col] = match (self.cells[row][col], live_neighbors) {
                    (true, 2) | (true, 3) => true,
                    (false, 3) => true,
                    _ => false,
                };
            }
        }

        next_board
    }

    fn count_live_neighbors(&self, row: usize, col: usize) -> usize {
        let mut count = 0;

        for dr in -1..=1 {
            for dc in -1..=1 {
                if dr == 0 && dc == 0 {
                    continue;
                }
                let new_row = row as isize + dr;
                let new_col = col as isize + dc;
                if new_row >= 0
                    && new_row < self.height as isize
                    && new_col >= 0
                    && new_col < self.width as isize
                    && self.cells[new_row as usize][new_col as usize]
                {
                    count += 1;
                }
            }
        }

        count
    }
}
