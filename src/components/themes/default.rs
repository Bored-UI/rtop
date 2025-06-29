use ratatui::style::Color;

use crate::types::AppColorInfo;

pub const DEFAULT: AppColorInfo = AppColorInfo {
    background_color: Color::Rgb(46, 52, 64),
    base_app_text_color: Color::Rgb(216, 222, 233),
    key_text_color: Color::Rgb(94, 129, 172),
    app_title_color: Color::Rgb(143, 188, 187),
    pop_up_color: Color::Rgb(76, 86, 106),
    pop_up_selected_color_bg: Color::Rgb(76, 86, 106),
    pop_up_blur_bg: Color::Rgb(70, 76, 88),

    cpu_container_selected_color: Color::Rgb(94, 129, 172),
    cpu_main_block_color: Color::Rgb(76, 86, 106),
    cpu_selected_color: Color::Rgb(94, 129, 172),
    cpu_base_graph_color: Color::Rgb(129, 161, 193),
    cpu_info_block_color: Color::Rgb(76, 86, 106),
    cpu_text_color: Color::Rgb(94, 129, 172),

    memory_container_selected_color: Color::Rgb(94, 129, 172),
    memory_main_block_color: Color::Rgb(76, 86, 106),
    used_memory_base_graph_color: Color::Rgb(129, 161, 193),
    available_memory_base_graph_color: Color::Rgb(129, 161, 193),
    free_memory_base_graph_color: Color::Rgb(129, 161, 193),
    cached_memory_base_graph_color: Color::Rgb(129, 161, 193),
    swap_memory_base_graph_color: Color::Rgb(129, 161, 193),
    memory_text_color: Color::Rgb(143, 188, 187),

    disk_container_selected_color: Color::Rgb(94, 129, 172),
    disk_main_block_color: Color::Rgb(76, 86, 106),
    disk_bytes_written_base_graph_color: Color::Rgb(129, 161, 193),
    disk_bytes_read_base_graph_color: Color::Rgb(129, 161, 193),
    disk_text_color: Color::Rgb(143, 188, 187),

    network_container_selected_color: Color::Rgb(94, 129, 172),
    network_main_block_color: Color::Rgb(76, 86, 106),
    network_received_base_graph_color: Color::Rgb(129, 161, 193),
    network_transmitted_base_graph_color: Color::Rgb(129, 161, 193),
    network_info_block_color: Color::Rgb(76, 86, 106),
    network_text_color: Color::Rgb(143, 188, 187),

    process_container_selected_color: Color::Rgb(94, 129, 172),
    process_main_block_color: Color::Rgb(76, 86, 106),
    process_base_graph_color: Color::Rgb(129, 161, 193),
    process_info_block_color: Color::Rgb(76, 86, 106),
    process_title_color: Color::Rgb(143, 188, 187),
    process_text_color: Color::Rgb(94, 129, 172),
    process_selected_color_bg: Color::Rgb(76, 86, 106),
    process_selected_color_fg: Color::Rgb(236, 239, 244),
};
