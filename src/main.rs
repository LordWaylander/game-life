use leptos::mount::mount_to_body;
use leptos::prelude::*;
use leptos::logging::log;
use leptos_use::use_interval_fn;
use web_sys::window;

const ROWS: i32 = 40;
const COLS: i32 = 50;
const INTERVAL: u64 = 20;

#[derive(Debug, Clone, Eq, Hash, PartialEq, Copy)]
struct Cell {
    row: i32,
    col: i32,
    is_alive: bool,
    nb_voisin_vivants: i32
}

fn initialize_grid() -> Vec<Vec<Cell>> {
    (0..ROWS).map(|i| 
        (0..COLS).map(|j| {
            //let alive = fastrand::bool();
            Cell {
                row: i,
                col: j,
                is_alive: fastrand::bool(),
                nb_voisin_vivants: 0
            }
        }).collect()
    ).collect()
}

fn get_neighbor_cell(row: i32, col: i32, grid: &Vec<Vec<Cell>>) -> Vec<Cell> {
    let mut neighbors: Vec<Cell> = Vec::new();
    // Traitement de la ligne précédente
    if row - 1 >= 0 {
        if col - 1 >= 0 {
            neighbors.push(grid[(row-1) as usize ][(col-1) as usize]);
        }
        if col + 1 < COLS {
            neighbors.push(grid[(row-1) as usize ][(col+1) as usize]);
        }
        neighbors.push(grid[(row-1) as usize ][(col) as usize]);
    }
    // Traitement de la ligne en cours
    if col - 1 >= 0 {
        neighbors.push(grid[(row) as usize ][(col-1) as usize]);
    }
    if col + 1 < COLS {
        neighbors.push(grid[(row) as usize ][(col+1) as usize]);
    }
    // Traitement de la ligne suivante
    if row + 1 < ROWS {
        if col - 1 >= 0 {
            neighbors.push(grid[(row+1) as usize ][(col-1) as usize]);
        }
        if col + 1 < COLS{
            neighbors.push(grid[(row+1) as usize ][(col+1) as usize]);
        }
        neighbors.push(grid[(row+1) as usize ][(col) as usize]);
    }
    return neighbors;
}

fn set_up_cells_alive(current_grid: Vec<Vec<Cell>>) -> Vec<Vec<Cell>> {

    current_grid.into_iter()
    .map(|row| {
        row.into_iter()
        .map(|cell| {
            let new_is_alive = if cell.is_alive && (cell.nb_voisin_vivants == 2 || cell.nb_voisin_vivants == 3) {
                true
            } else if !cell.is_alive && cell.nb_voisin_vivants == 3 {
                true
            } else {
                false
            };
            Cell {
                is_alive: new_is_alive,
                nb_voisin_vivants: 0,
                ..cell
            }
        }).collect()
    }).collect()
}

fn next_step(grid: ReadSignal<Vec<Vec<Cell>>>, set_grid: WriteSignal<Vec<Vec<Cell>>>) {
    let now = window().unwrap().performance().unwrap().now();

    let mut current_grid = grid.get();

    let cells_alive: Vec<Cell> = current_grid.clone().into_iter()
    .flat_map(|row| row.into_iter())
    .filter(|cell| cell.is_alive).collect();

    for cell in cells_alive  {
        let voisins_cell = get_neighbor_cell(cell.row, cell.col, &current_grid);

        for voisin in voisins_cell {
            current_grid[voisin.row as usize][voisin.col as usize].nb_voisin_vivants +=1;
        }
    }
    let new_grid = set_up_cells_alive(current_grid);
    set_grid.set(new_grid);
    let end = window().unwrap().performance().unwrap().now();
    log!("time: {:.2}ms", end - now);
}

#[component]
pub fn App() -> impl IntoView {
    let (grid, set_grid) = signal(initialize_grid());
    let (is_playing, set_is_playing) = signal(false);
    let (interval, _set_interval) = signal(INTERVAL);

    use_interval_fn(
        move || {
            if is_playing.get() {
                next_step(grid, set_grid);
            }
        },
        interval.get_untracked()
    );
    
    view! {
        <div>
            <div>
                <button
                    on:click=move |_| next_step(grid, set_grid)
                >
                    "Next Step"
                </button>
                <button
                    on:click=move |_| *set_is_playing.write() = true
                >
                    "Play"
                </button>
                <button
                    on:click=move |_| *set_is_playing.write() = false
                >
                    "pause"
                </button>
                // <button
                //     on:click=move |_| stop()
                // >
                //     "stop"
                // </button>
            </div>
            <table>
                <For
                    each=|| (0..ROWS).enumerate()
                    key=|(i, _)| i.clone()
                    let((i, _))
                >
                    <tr>
                        <For
                        each=move || (0..COLS).enumerate()
                        key=|(j, _)| j.clone()
                        let ((j, _))
                        >
                        {
                            let is_alive = move || grid.get()[i][j].is_alive;
                            view! {
                                <td
                                    class:isAlive=is_alive
                                    on:click=move |_| {
                                        set_grid.update(|grid| {
                                            grid[i][j].is_alive = !grid[i][j].is_alive;
                                        });
                                    }
                                >
                                    //{move || if is_alive() { "●" } else { "○" } }
                                </td>
                            }
                        }
                        </For>
                    </tr>
                </For>
            </table>  
        </div>
    }
}

fn main() {
    mount_to_body(App);
}