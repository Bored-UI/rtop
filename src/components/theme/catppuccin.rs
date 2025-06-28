use ratatui::style::Color;

use crate::components::theme::types::AppColorInfo;

pub const CATPPUCCIN_MOCHA: AppColorInfo = AppColorInfo {
    background_color: Color::Rgb(30, 30, 46),
    base_app_text_color: Color::Rgb(205, 214, 244),
    key_text_color: Color::Rgb(137, 180, 250),
    app_title_color: Color::Rgb(180, 190, 254),
    pop_up_color: Color::Rgb(69, 71, 90),
    pop_up_selected_color_bg: Color::Rgb(203, 166, 247),
    pop_up_blur_bg: Color::Rgb(49, 50, 68),

    cpu_container_selected_color: Color::Rgb(166, 227, 161),
    cpu_main_block_color: Color::Rgb(49, 50, 68),
    cpu_selected_color: Color::Rgb(166, 227, 161),
    cpu_base_graph_color: Color::Rgb(166, 227, 161),
    cpu_info_block_color: Color::Rgb(49, 50, 68),
    cpu_text_color: Color::Rgb(166, 227, 161),

    memory_container_selected_color: Color::Rgb(249, 226, 175),
    memory_main_block_color: Color::Rgb(49, 50, 68),
    used_memory_base_graph_color: Color::Rgb(243, 139, 168),
    available_memory_base_graph_color: Color::Rgb(166, 227, 161),
    free_memory_base_graph_color: Color::Rgb(116, 199, 236),
    cached_memory_base_graph_color: Color::Rgb(250, 179, 135),
    swap_memory_base_graph_color: Color::Rgb(203, 166, 247),
    memory_text_color: Color::Rgb(249, 226, 175),

    disk_container_selected_color: Color::Rgb(137, 220, 235),
    disk_main_block_color: Color::Rgb(49, 50, 68),
    disk_bytes_written_base_graph_color: Color::Rgb(250, 179, 135),
    disk_bytes_read_base_graph_color: Color::Rgb(148, 226, 213),
    disk_text_color: Color::Rgb(137, 220, 235),

    network_container_selected_color: Color::Rgb(242, 205, 205),
    network_main_block_color: Color::Rgb(49, 50, 68),
    network_received_base_graph_color: Color::Rgb(137, 180, 250),
    network_transmitted_base_graph_color: Color::Rgb(245, 194, 231),
    network_info_block_color: Color::Rgb(49, 50, 68),
    network_text_color: Color::Rgb(242, 205, 205),

    process_container_selected_color: Color::Rgb(245, 224, 220),
    process_main_block_color: Color::Rgb(49, 50, 68),
    process_base_graph_color: Color::Rgb(203, 166, 247),
    process_info_block_color: Color::Rgb(49, 50, 68),
    process_title_color: Color::Rgb(245, 224, 220),
    process_text_color: Color::Rgb(166, 173, 200),
    process_selected_color_bg: Color::Rgb(203, 166, 247),
    process_selected_color_fg: Color::Rgb(30, 30, 46),
};
