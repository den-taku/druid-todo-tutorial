mod data;
use data::AppState;

mod view;
use view::build_ui;

use druid::{AppLauncher, WindowDesc};

fn main() {
    let main_window = WindowDesc::new(build_ui)
        .title("Todo Tutorial")
        .window_size((400.0, 400.0));

    let initial_state = AppState::load_from_json();

    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}
