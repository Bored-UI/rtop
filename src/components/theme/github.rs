use ratatui::style::Color;

use crate::components::theme::types::AppColorInfo;

pub const GITHUB_DARK: AppColorInfo = AppColorInfo {
    background_color: Color::Rgb(13, 17, 23),
    base_app_text_color: Color::Rgb(201, 209, 217),
    key_text_color: Color::Rgb(88, 166, 255),
    app_title_color: Color::Rgb(201, 209, 217),
    pop_up_color: Color::Rgb(22, 27, 34),
    pop_up_selected_color_bg: Color::Rgb(88, 166, 255),
    pop_up_blur_bg: Color::Rgb(33, 38, 45),

    cpu_container_selected_color: Color::Rgb(63, 185, 80),
    cpu_main_block_color: Color::Rgb(22, 27, 34),
    cpu_selected_color: Color::Rgb(63, 185, 80),
    cpu_base_graph_color: Color::Rgb(63, 185, 80),
    cpu_info_block_color: Color::Rgb(22, 27, 34),
    cpu_text_color: Color::Rgb(63, 185, 80),

    memory_container_selected_color: Color::Rgb(163, 113, 247),
    memory_main_block_color: Color::Rgb(22, 27, 34),
    used_memory_base_graph_color: Color::Rgb(255, 123, 114),
    available_memory_base_graph_color: Color::Rgb(63, 185, 80),
    free_memory_base_graph_color: Color::Rgb(63, 185, 80),
    cached_memory_base_graph_color: Color::Rgb(219, 109, 40),
    swap_memory_base_graph_color: Color::Rgb(163, 113, 247),
    memory_text_color: Color::Rgb(163, 113, 247),

    disk_container_selected_color: Color::Rgb(219, 109, 40),
    disk_main_block_color: Color::Rgb(22, 27, 34),
    disk_bytes_written_base_graph_color: Color::Rgb(219, 109, 40),
    disk_bytes_read_base_graph_color: Color::Rgb(88, 166, 255),
    disk_text_color: Color::Rgb(219, 109, 40),

    network_container_selected_color: Color::Rgb(88, 166, 255),
    network_main_block_color: Color::Rgb(22, 27, 34),
    network_received_base_graph_color: Color::Rgb(121, 192, 255),
    network_transmitted_base_graph_color: Color::Rgb(63, 185, 80),
    network_info_block_color: Color::Rgb(22, 27, 34),
    network_text_color: Color::Rgb(88, 166, 255),

    process_container_selected_color: Color::Rgb(255, 123, 114),
    process_main_block_color: Color::Rgb(22, 27, 34),
    process_base_graph_color: Color::Rgb(48, 54, 61),
    process_info_block_color: Color::Rgb(22, 27, 34),
    process_title_color: Color::Rgb(255, 123, 114),
    process_text_color: Color::Rgb(201, 209, 217),
    process_selected_color_bg: Color::Rgb(88, 166, 255),
    process_selected_color_fg: Color::Rgb(240, 246, 252),
};
