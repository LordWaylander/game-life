/// Documentation for [`Nav`]

use leptos::prelude::*;
use crate::{Cell, next_step, initialize_grid, INTERVAL};
use leptos::logging::log;

#[component]
pub fn Nav(
    grid : (ReadSignal<Vec<Vec<Cell>>>, WriteSignal<Vec<Vec<Cell>>>),
    playing: (ReadSignal<bool>, WriteSignal<bool>),
    interval: (ReadSignal<u64>, WriteSignal<u64>)
) -> impl IntoView {
    

    view! {
        <div>
            <button
                on:click=move |_| next_step(grid.0, grid.1)
            >
                "Next Step"
            </button>
            <Show
                when=move || { !playing.0.get() }
            >
                <button
                    on:click=move |_| playing.1.set(true)
                >
                    "Play"
                </button>
            </Show>
            <Show
                when=move || { playing.0.get() }
            >
                <button
                    on:click=move |_| playing.1.set(false)
                >
                    "Pause"
                </button>
            </Show>
            <button
                on:click=move |_| {
                    grid.1.set(initialize_grid());
                    playing.1.set(false);
                }
            >
                "Reset"
            </button>
        </div>
        <div>
            <button
            on:click=move |_| {
                interval.1.set(INTERVAL);
            }
            >
                "x1" // 75
            </button>
            <button
            on:click=move |_| {
                log!("x2");
                log!("{:?}", interval.0.get());
                interval.1.set(INTERVAL/2);
                log!("{:?}", interval.0.get());
            }
            >
                "x2" // 75x2
            </button>
            <button
            on:click=move |_| {
                interval.1.set(INTERVAL/3);
            }
            >
                "x3" // 75x3
            </button>
        </div>
    }
}