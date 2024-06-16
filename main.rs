use leptos::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::window;

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    leptos::start_app("app", app);
}

#[derive(Clone, Default)]
struct TimerState {
    elapsed: u64,
    running: bool,
}

fn app(cx: Scope) -> Element {
    let state = use_state(cx, || TimerState::default());
    let interval_id = use_state(cx, || None);

    let start_timer = {
        let state = state.clone();
        let interval_id = interval_id.clone();
        move |_| {
            if state.get().running {
                return;
            }
            state.update(|s| s.running = true);

            let window = window().expect("no global `window` exists");
            let interval = window
                .set_interval_with_callback_and_timeout_and_arguments_0(
                    Closure::wrap(Box::new(move || {
                        state.update(|s| s.elapsed += 1);
                    }) as Box<dyn Fn()>)
                    .as_ref()
                    .unchecked_ref(),
                    1000,
                )
                .expect("should register `setInterval` OK");

            interval_id.set(Some(interval));
        }
    };

    let stop_timer = {
        let state = state.clone();
        let interval_id = interval_id.clone();
        move |_| {
            state.update(|s| s.running = false);
            if let Some(id) = interval_id.get() {
                window()
                    .expect("no global `window` exists")
                    .clear_interval_with_handle(id);
                interval_id.set(None);
            }
        }
    };

    let reset_timer = {
        let state = state.clone();
        let interval_id = interval_id.clone();
        move |_| {
            state.update(|s| {
                s.elapsed = 0;
                s.running = false;
            });
            if let Some(id) = interval_id.get() {
                window()
                    .expect("no global `window` exists")
                    .clear_interval_with_handle(id);
                interval_id.set(None);
            }
        }
    };

    view! { cx,
        div(class="timer-app") {
            h1 { "Timer" }
            p { (state.get().elapsed) " seconds" }
            button(on:click=start_timer) { "Start" }
            button(on:click=stop_timer) { "Stop" }
            button(on:click=reset_timer) { "Reset" }
        }
    }
}
