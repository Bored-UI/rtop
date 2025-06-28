use ratatui::style::Color;

use crate::components::theme::types::AppColorInfo;

pub const FLATREMIX: AppColorInfo = AppColorInfo {
    background_color: Color::Rgb(45, 45, 45),
    base_app_text_color: Color::Rgb(230, 230, 230),
    key_text_color: Color::Rgb(144, 144, 144),
    app_title_color: Color::Rgb(255, 255, 255),
    pop_up_color: Color::Rgb(80, 80, 80),
    pop_up_selected_color_bg: Color::Rgb(184, 23, 76),
    pop_up_blur_bg: Color::Rgb(64, 64, 64),

    cpu_container_selected_color: Color::Rgb(54, 123, 240),
    cpu_main_block_color: Color::Rgb(80, 80, 80),
    cpu_selected_color: Color::Rgb(184, 23, 76),
    cpu_base_graph_color: Color::Rgb(54, 123, 240),
    cpu_info_block_color: Color::Rgb(80, 80, 80),
    cpu_text_color: Color::Rgb(54, 123, 240),

    memory_container_selected_color: Color::Rgb(25, 161, 135),
    memory_main_block_color: Color::Rgb(80, 80, 80),
    used_memory_base_graph_color: Color::Rgb(18, 113, 95),
    available_memory_base_graph_color: Color::Rgb(254, 164, 76),
    free_memory_base_graph_color: Color::Rgb(129, 16, 53),
    cached_memory_base_graph_color: Color::Rgb(38, 86, 168),
    swap_memory_base_graph_color: Color::Rgb(129, 16, 53),
    memory_text_color: Color::Rgb(25, 161, 135),

    disk_container_selected_color: Color::Rgb(25, 161, 135),
    disk_main_block_color: Color::Rgb(80, 80, 80),
    disk_bytes_written_base_graph_color: Color::Rgb(140, 66, 171),
    disk_bytes_read_base_graph_color: Color::Rgb(54, 123, 240),
    disk_text_color: Color::Rgb(25, 161, 135),

    network_container_selected_color: Color::Rgb(253, 53, 53),
    network_main_block_color: Color::Rgb(80, 80, 80),
    network_received_base_graph_color: Color::Rgb(54, 123, 240),
    network_transmitted_base_graph_color: Color::Rgb(140, 66, 171),
    network_info_block_color: Color::Rgb(80, 80, 80),
    network_text_color: Color::Rgb(253, 53, 53),

    process_container_selected_color: Color::Rgb(74, 174, 230),
    process_main_block_color: Color::Rgb(80, 80, 80),
    process_base_graph_color: Color::Rgb(54, 123, 240),
    process_info_block_color: Color::Rgb(80, 80, 80),
    process_title_color: Color::Rgb(74, 174, 230),
    process_text_color: Color::Rgb(54, 123, 240),
    process_selected_color_bg: Color::Rgb(184, 23, 76),
    process_selected_color_fg: Color::Rgb(255, 255, 255),
};

pub const FLATREMIX_LIGHT: AppColorInfo = AppColorInfo {
    background_color: Color::Rgb(228, 228, 231),
    base_app_text_color: Color::Rgb(115, 118, 128),
    key_text_color: Color::Rgb(144, 144, 144),
    app_title_color: Color::Rgb(39, 42, 52),
    pop_up_color: Color::Rgb(80, 80, 80),
    pop_up_selected_color_bg: Color::Rgb(184, 23, 76),
    pop_up_blur_bg: Color::Rgb(220, 220, 223),

    cpu_container_selected_color: Color::Rgb(54, 123, 240),
    cpu_main_block_color: Color::Rgb(80, 80, 80),
    cpu_selected_color: Color::Rgb(184, 23, 76),
    cpu_base_graph_color: Color::Rgb(54, 123, 240),
    cpu_info_block_color: Color::Rgb(80, 80, 80),
    cpu_text_color: Color::Rgb(54, 123, 240),

    memory_container_selected_color: Color::Rgb(25, 161, 135),
    memory_main_block_color: Color::Rgb(80, 80, 80),
    used_memory_base_graph_color: Color::Rgb(18, 113, 95),
    available_memory_base_graph_color: Color::Rgb(254, 164, 76),
    free_memory_base_graph_color: Color::Rgb(129, 16, 53),
    cached_memory_base_graph_color: Color::Rgb(38, 86, 168),
    swap_memory_base_graph_color: Color::Rgb(129, 16, 53),
    memory_text_color: Color::Rgb(25, 161, 135),

    disk_container_selected_color: Color::Rgb(25, 161, 135),
    disk_main_block_color: Color::Rgb(80, 80, 80),
    disk_bytes_written_base_graph_color: Color::Rgb(140, 66, 171),
    disk_bytes_read_base_graph_color: Color::Rgb(54, 123, 240),
    disk_text_color: Color::Rgb(25, 161, 135),

    network_container_selected_color: Color::Rgb(253, 53, 53),
    network_main_block_color: Color::Rgb(80, 80, 80),
    network_received_base_graph_color: Color::Rgb(54, 123, 240),
    network_transmitted_base_graph_color: Color::Rgb(140, 66, 171),
    network_info_block_color: Color::Rgb(80, 80, 80),
    network_text_color: Color::Rgb(253, 53, 53),

    process_container_selected_color: Color::Rgb(74, 174, 230),
    process_main_block_color: Color::Rgb(80, 80, 80),
    process_base_graph_color: Color::Rgb(54, 123, 240),
    process_info_block_color: Color::Rgb(80, 80, 80),
    process_title_color: Color::Rgb(74, 174, 230),
    process_text_color: Color::Rgb(54, 123, 240),
    process_selected_color_bg: Color::Rgb(184, 23, 76),
    process_selected_color_fg: Color::Rgb(255, 255, 255),
};
