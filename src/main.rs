pub mod app;
pub mod components;
pub mod get_sys_info;
pub mod types;
pub mod utils;

use clap::Parser;
use components::*;

use app::*;
use inquire::Select;

use crate::components::theme::theme::set_theme;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Arg {
    #[arg(long)]
    theme: bool,
}

fn main() {
    let args = Arg::parse();
    if args.theme {
        prompt_for_theme();
    } else {
        app();
    }
}

fn prompt_for_theme() {
    println!("Starting in theme selection mode...");

    let themes = vec![
        "default",
        "dracula",
        "gruvbox_dark",
        "gruvbox_light",
        "gruvbox_mat_dark",
        "ayu",
        "everforest_dark",
        "everforest_light",
        "flatremix",
        "flatremix_light",
        "grayscale",
        "horizon",
        "kanagawa_wave",
        "kanagawa_lotus",
        "monokai",
        "onedark",
        "nightowl",
        "rosepine",
        "matcha_dark_sea",
        "paper",
        "solarized_dark",
        "solarized_light",
        "tokyo_night",
        "tokyo_storm",
        "catppuccin_mocha",
        "github_dark",
    ];
    let ans = Select::new("Please choose a color theme:", themes).prompt();

    match ans {
        Ok(theme) => {
            println!("You chose: {}. Setting theme...", theme);
            set_theme(theme.to_string());
            println!("Theme set")
        }
        Err(_) => {
            println!("Fail to set chosen theme");
        }
    }
}
