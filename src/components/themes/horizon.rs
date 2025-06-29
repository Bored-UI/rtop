use ratatui::style::Color;

use crate::types::AppColorInfo;

pub const HORIZON: AppColorInfo = AppColorInfo {
    background_color: Color::Rgb(28, 30, 38),
    base_app_text_color: Color::Rgb(248, 248, 242),
    key_text_color: Color::Rgb(184, 119, 219),
    app_title_color: Color::Rgb(248, 248, 242),
    pop_up_color: Color::Rgb(39, 46, 51),
    pop_up_selected_color_bg: Color::Rgb(184, 119, 219),
    pop_up_blur_bg: Color::Rgb(39, 46, 51),

    cpu_container_selected_color: Color::Rgb(184, 119, 219),
    cpu_main_block_color: Color::Rgb(39, 46, 51),
    cpu_selected_color: Color::Rgb(184, 119, 219),
    cpu_base_graph_color: Color::Rgb(39, 215, 150),
    cpu_info_block_color: Color::Rgb(39, 46, 51),
    cpu_text_color: Color::Rgb(184, 119, 219),

    memory_container_selected_color: Color::Rgb(39, 215, 150),
    memory_main_block_color: Color::Rgb(39, 46, 51),
    used_memory_base_graph_color: Color::Rgb(39, 215, 150),
    available_memory_base_graph_color: Color::Rgb(39, 215, 150),
    free_memory_base_graph_color: Color::Rgb(233, 86, 120),
    cached_memory_base_graph_color: Color::Rgb(39, 215, 150),
    swap_memory_base_graph_color: Color::Rgb(39, 215, 150),
    memory_text_color: Color::Rgb(39, 215, 150),

    disk_container_selected_color: Color::Rgb(39, 215, 150),
    disk_main_block_color: Color::Rgb(39, 46, 51),
    disk_bytes_written_base_graph_color: Color::Rgb(39, 215, 150),
    disk_bytes_read_base_graph_color: Color::Rgb(39, 215, 150),
    disk_text_color: Color::Rgb(39, 215, 150),

    network_container_selected_color: Color::Rgb(233, 86, 120),
    network_main_block_color: Color::Rgb(39, 46, 51),
    network_received_base_graph_color: Color::Rgb(39, 215, 150),
    network_transmitted_base_graph_color: Color::Rgb(39, 215, 150),
    network_info_block_color: Color::Rgb(39, 46, 51),
    network_text_color: Color::Rgb(233, 86, 120),

    process_container_selected_color: Color::Rgb(37, 178, 188),
    process_main_block_color: Color::Rgb(39, 46, 51),
    process_base_graph_color: Color::Rgb(39, 215, 150),
    process_info_block_color: Color::Rgb(39, 46, 51),
    process_title_color: Color::Rgb(37, 178, 188),
    process_text_color: Color::Rgb(39, 215, 150),
    process_selected_color_bg: Color::Rgb(40, 43, 55),
    process_selected_color_fg: Color::Rgb(248, 248, 242),
};
