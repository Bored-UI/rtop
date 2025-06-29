use ratatui::style::Color;

use crate::types::AppColorInfo;

pub const PAPER: AppColorInfo = AppColorInfo {
    background_color: Color::Rgb(238, 238, 238),
    base_app_text_color: Color::Rgb(0, 0, 0),
    key_text_color: Color::Rgb(204, 62, 40),
    app_title_color: Color::Rgb(0, 0, 0),
    pop_up_color: Color::Rgb(216, 213, 199),
    pop_up_selected_color_bg: Color::Rgb(204, 62, 40),
    pop_up_blur_bg: Color::Rgb(216, 213, 199),

    cpu_container_selected_color: Color::Rgb(204, 62, 40),
    cpu_main_block_color: Color::Rgb(216, 213, 199),
    cpu_selected_color: Color::Rgb(204, 62, 40),
    cpu_base_graph_color: Color::Rgb(85, 85, 85),
    cpu_info_block_color: Color::Rgb(216, 213, 199),
    cpu_text_color: Color::Rgb(0, 0, 0),

    memory_container_selected_color: Color::Rgb(204, 62, 40),
    memory_main_block_color: Color::Rgb(216, 213, 199),
    used_memory_base_graph_color: Color::Rgb(204, 62, 40),
    available_memory_base_graph_color: Color::Rgb(33, 102, 9),
    free_memory_base_graph_color: Color::Rgb(33, 102, 9),
    cached_memory_base_graph_color: Color::Rgb(30, 111, 204),
    swap_memory_base_graph_color: Color::Rgb(30, 111, 204),
    memory_text_color: Color::Rgb(0, 0, 0),

    disk_container_selected_color: Color::Rgb(204, 62, 40),
    disk_main_block_color: Color::Rgb(216, 213, 199),
    disk_bytes_written_base_graph_color: Color::Rgb(204, 62, 40),
    disk_bytes_read_base_graph_color: Color::Rgb(85, 85, 85),
    disk_text_color: Color::Rgb(0, 0, 0),

    network_container_selected_color: Color::Rgb(204, 62, 40),
    network_main_block_color: Color::Rgb(216, 213, 199),
    network_received_base_graph_color: Color::Rgb(85, 85, 85),
    network_transmitted_base_graph_color: Color::Rgb(85, 85, 85),
    network_info_block_color: Color::Rgb(216, 213, 199),
    network_text_color: Color::Rgb(0, 0, 0),

    process_container_selected_color: Color::Rgb(204, 62, 40),
    process_main_block_color: Color::Rgb(216, 213, 199),
    process_base_graph_color: Color::Rgb(85, 85, 85),
    process_info_block_color: Color::Rgb(216, 213, 199),
    process_title_color: Color::Rgb(0, 0, 0),
    process_text_color: Color::Rgb(0, 0, 0),
    process_selected_color_bg: Color::Rgb(216, 213, 199),
    process_selected_color_fg: Color::Rgb(0, 0, 0),
};
