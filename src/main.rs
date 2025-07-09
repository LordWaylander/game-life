use leptos::mount::mount_to_body;
use leptos::prelude::*;
use leptos::logging::log;

const ROWS: i32 = 3;
const COLS: i32 = 3;

#[derive(Debug, Clone, Eq, Hash, PartialEq, Copy)]
struct Cell {
    row: i32,
    col: i32,
    is_alive: bool, //reactif here
    nb_voisin_vivants: i32
}

fn initialize_grid() -> Vec<Vec<Cell>> {
    (0..ROWS).map(|i| 
        (0..COLS).map(|j| {
            //let alive = fastrand::bool();
            Cell {
                row: i,
                col: j,
                is_alive: false, //reactif here
                nb_voisin_vivants: 0
            }
        }
).collect()
    ).collect()
}

// algo
// get cells vivante
// pour chaque cell look voisin si vivant = count+1
//for chaque cells if voisin_viant > x -> dead ou alive au choix

fn get_voisins_cell(row: i32, col: i32, grid: ReadSignal<Vec<Vec<Cell>>>) -> Vec<Cell> {
    let mut voisins: Vec<Cell> = Vec::new();

    // Traitement de la ligne précédente
    if row - 1 >= 0 {
        if col - 1 >= 0 {
            voisins.push(grid.get()[(row-1) as usize ][(col-1) as usize]);
        }
        if col + 1 < COLS {
            voisins.push(grid.get()[(row-1) as usize ][(col+1) as usize]);
        }
        voisins.push(grid.get()[(row-1) as usize ][(col) as usize]);
    }
    // Traitement de la ligne en cours
    if col - 1 >= 0 {
        voisins.push(grid.get()[(row) as usize ][(col-1) as usize]);
    }
    if col + 1 < COLS {
        voisins.push(grid.get()[(row) as usize ][(col+1) as usize]);
    }
    // Traitement de la ligne suivante
    if row + 1 < ROWS {
        if col - 1 >= 0 {
            voisins.push(grid.get()[(row+1) as usize ][(col-1) as usize]);
        }
        if col + 1 < COLS{
            voisins.push(grid.get()[(row+1) as usize ][(col+1) as usize]);
        }
        voisins.push(grid.get()[(row+1) as usize ][(col) as usize]);
    }
    return voisins;
}


#[component]
pub fn App() -> impl IntoView {
    let (grid, set_grid) = signal(initialize_grid());

    let next_step = move || {
        let current_grid = grid.get();
    
        let cells_alive: Vec<Cell> = current_grid.into_iter()
        .flat_map(|row| row.into_iter())
        .filter(|cell| cell.is_alive).collect();

        for cell in cells_alive  {
            let voisins_cell = get_voisins_cell(cell.row, cell.col, grid);

            for voisin in voisins_cell {
                set_grid.update(|grid| {
                    grid[voisin.row as usize][voisin.col as usize].nb_voisin_vivants +=1;
                });

            }
        }

        // let ns = grid.get();
        // log!("{:?}", ns);


    };
    
    view! {
        <div>
            <button
                on:click=move |_| next_step()
            >
                "Next Step"
            </button>

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