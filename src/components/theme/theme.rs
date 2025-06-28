use std::fs::{File, OpenOptions};

use crate::{
    components::theme::{
        ayu::AYU,
        catppuccin::CATPPUCCIN_MOCHA,
        default::DEFAULT,
        dracula::DRACULA,
        everforest::{EVERFOREST_DARK, EVERFOREST_LIGHT},
        flatremix::{FLATREMIX, FLATREMIX_LIGHT},
        github::GITHUB_DARK,
        grayscale::GRAYSCALE,
        gruvbox::{GRUVBOX_DARK, GRUVBOX_LIGHT, GRUVBOX_MAT_DARK},
        horizon::HORIZON,
        kanagawa::{KANAGAWA_LOTUS, KANAGAWA_WAVE},
        matcha::MATCHA_DARK_SEA,
        monokai::MONOKAI,
        nightowl::NIGHT_OWL,
        onedark::ONEDARK,
        paper::PAPER,
        rosepine::ROSE_PINE,
        solarized::{SOLARIZED_DARK, SOLARIZED_LIGHT},
        tokyo::{TOKYO_NIGHT, TOKYO_STORM},
        types::AppColorInfo,
    },
    types::ThemeConfig,
    utils::{create_file_with_dirs, get_user_directory},
};

pub fn get_and_return_app_color_info() -> AppColorInfo {
    let theme_config_filepath = get_user_directory().join(".rtop/settings.json");
    if !theme_config_filepath.exists() {
        let theme_config = ThemeConfig {
            theme: "default".to_string(),
        };

        create_file_with_dirs(theme_config_filepath.to_str().unwrap());
        let file = OpenOptions::new()
            .write(true)
            .create(true) // Create the file if it doesn't exist
            .truncate(true) // Truncate the file to ensure it's empty before writing
            .open(&theme_config_filepath)
            .unwrap();

        // write the data into the json file
        let _ = serde_json::to_writer(file, &theme_config);

        return DEFAULT;
    }
    // read the json file to configure the settings instead if it exist
    let file = File::open(theme_config_filepath).unwrap();
    let theme_config: ThemeConfig = serde_json::from_reader(file).unwrap();

    let theme_str: &str = &theme_config.theme;
    match theme_str {
        "default" => return DEFAULT,
        "dracula" => return DRACULA,
        "gruvbox_dark" => return GRUVBOX_DARK,
        "gruvbox_light" => return GRUVBOX_LIGHT,
        "gruvbox_mat_dark" => return GRUVBOX_MAT_DARK,
        "ayu" => return AYU,
        "everforest_dark" => return EVERFOREST_DARK,
        "everforest_light" => return EVERFOREST_LIGHT,
        "flatremix" => return FLATREMIX,
        "flatremix_light" => return FLATREMIX_LIGHT,
        "grayscale" => return GRAYSCALE,
        "horizon" => return HORIZON,
        "kanagawa_wave" => return KANAGAWA_WAVE,
        "kanagawa_lotus" => return KANAGAWA_LOTUS,
        "monokai" => return MONOKAI,
        "onedark" => return ONEDARK,
        "nightowl" => return NIGHT_OWL,
        "rosepine" => return ROSE_PINE,
        "matcha_dark_sea" => return MATCHA_DARK_SEA,
        "paper" => return PAPER,
        "solarized_dark" => return SOLARIZED_DARK,
        "solarized_light" => return SOLARIZED_LIGHT,
        "tokyo_night" => return TOKYO_NIGHT,
        "tokyo_storm" => return TOKYO_STORM,
        "catppuccin_mocha" => return CATPPUCCIN_MOCHA,
        "github_dark" => return GITHUB_DARK,
        _ => return DEFAULT,
    }
}

pub fn set_theme(theme_string: String) {
    let theme_config_filepath = get_user_directory().join(".rtop/settings.json");
    let theme_config = ThemeConfig {
        theme: theme_string,
    };

    create_file_with_dirs(theme_config_filepath.to_str().unwrap());
    let file = OpenOptions::new()
        .write(true)
        .create(true) // Create the file if it doesn't exist
        .truncate(true) // Truncate the file to ensure it's empty before writing
        .open(&theme_config_filepath)
        .unwrap();

    // write the data into the json file
    let _ = serde_json::to_writer(file, &theme_config);
}
