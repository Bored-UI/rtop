pub mod components;
pub mod get_sys_info;
pub mod tui;
pub mod types;
pub mod utils;
use components::*;

use tui::*;

fn main() {
    tui();
}
