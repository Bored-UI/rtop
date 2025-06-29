use ratatui::style::Color;

use crate::types::AppColorInfo;

pub const GRAYSCALE: AppColorInfo = AppColorInfo {
    background_color: Color::Rgb(0, 0, 0),
    base_app_text_color: Color::Rgb(187, 187, 187),
    key_text_color: Color::Rgb(144, 144, 144),
    app_title_color: Color::Rgb(204, 204, 204),
    pop_up_color: Color::Rgb(48, 48, 48),
    pop_up_selected_color_bg: Color::Rgb(255, 255, 255),
    pop_up_blur_bg: Color::Rgb(48, 48, 48),

    cpu_container_selected_color: Color::Rgb(144, 144, 144),
    cpu_main_block_color: Color::Rgb(48, 48, 48),
    cpu_selected_color: Color::Rgb(255, 255, 255),
    cpu_base_graph_color: Color::Rgb(80, 80, 80),
    cpu_info_block_color: Color::Rgb(48, 48, 48),
    cpu_text_color: Color::Rgb(144, 144, 144),

    memory_container_selected_color: Color::Rgb(144, 144, 144),
    memory_main_block_color: Color::Rgb(48, 48, 48),
    used_memory_base_graph_color: Color::Rgb(80, 80, 80),
    available_memory_base_graph_color: Color::Rgb(80, 80, 80),
    free_memory_base_graph_color: Color::Rgb(80, 80, 80),
    cached_memory_base_graph_color: Color::Rgb(80, 80, 80),
    swap_memory_base_graph_color: Color::Rgb(80, 80, 80),
    memory_text_color: Color::Rgb(144, 144, 144),

    disk_container_selected_color: Color::Rgb(144, 144, 144),
    disk_main_block_color: Color::Rgb(48, 48, 48),
    disk_bytes_written_base_graph_color: Color::Rgb(48, 48, 48),
    disk_bytes_read_base_graph_color: Color::Rgb(48, 48, 48),
    disk_text_color: Color::Rgb(144, 144, 144),

    network_container_selected_color: Color::Rgb(144, 144, 144),
    network_main_block_color: Color::Rgb(48, 48, 48),
    network_received_base_graph_color: Color::Rgb(48, 48, 48),
    network_transmitted_base_graph_color: Color::Rgb(48, 48, 48),
    network_info_block_color: Color::Rgb(48, 48, 48),
    network_text_color: Color::Rgb(144, 144, 144),

    process_container_selected_color: Color::Rgb(144, 144, 144),
    process_main_block_color: Color::Rgb(48, 48, 48),
    process_base_graph_color: Color::Rgb(144, 144, 144),
    process_info_block_color: Color::Rgb(48, 48, 48),
    process_title_color: Color::Rgb(144, 144, 144),
    process_text_color: Color::Rgb(144, 144, 144),
    process_selected_color_bg: Color::Rgb(255, 255, 255),
    process_selected_color_fg: Color::Rgb(0, 0, 0),
};
