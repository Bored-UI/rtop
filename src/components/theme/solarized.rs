use ratatui::style::Color;

use crate::components::theme::types::AppColorInfo;

pub const SOLARIZED_DARK: AppColorInfo = AppColorInfo {
    background_color: Color::Rgb(0, 43, 54),
    base_app_text_color: Color::Rgb(238, 232, 213),
    key_text_color: Color::Rgb(181, 137, 0),
    app_title_color: Color::Rgb(253, 246, 227),
    pop_up_color: Color::Rgb(7, 54, 66),
    pop_up_selected_color_bg: Color::Rgb(214, 162, 0),
    pop_up_blur_bg: Color::Rgb(7, 54, 66),

    cpu_container_selected_color: Color::Rgb(133, 153, 0),
    cpu_main_block_color: Color::Rgb(7, 54, 66),
    cpu_selected_color: Color::Rgb(214, 162, 0),
    cpu_base_graph_color: Color::Rgb(133, 153, 0),
    cpu_info_block_color: Color::Rgb(7, 54, 66),
    cpu_text_color: Color::Rgb(133, 153, 0),

    memory_container_selected_color: Color::Rgb(38, 139, 210),
    memory_main_block_color: Color::Rgb(7, 54, 66),
    used_memory_base_graph_color: Color::Rgb(220, 50, 47),
    available_memory_base_graph_color: Color::Rgb(181, 137, 0),
    free_memory_base_graph_color: Color::Rgb(133, 153, 0),
    cached_memory_base_graph_color: Color::Rgb(38, 139, 210),
    swap_memory_base_graph_color: Color::Rgb(203, 75, 22),
    memory_text_color: Color::Rgb(38, 139, 210),

    disk_container_selected_color: Color::Rgb(42, 161, 152),
    disk_main_block_color: Color::Rgb(7, 54, 66),
    disk_bytes_written_base_graph_color: Color::Rgb(211, 54, 130),
    disk_bytes_read_base_graph_color: Color::Rgb(42, 161, 152),
    disk_text_color: Color::Rgb(42, 161, 152),

    network_container_selected_color: Color::Rgb(108, 113, 196),
    network_main_block_color: Color::Rgb(7, 54, 66),
    network_received_base_graph_color: Color::Rgb(108, 113, 196),
    network_transmitted_base_graph_color: Color::Rgb(211, 54, 130),
    network_info_block_color: Color::Rgb(7, 54, 66),
    network_text_color: Color::Rgb(108, 113, 196),

    process_container_selected_color: Color::Rgb(181, 137, 0),
    process_main_block_color: Color::Rgb(7, 54, 66),
    process_base_graph_color: Color::Rgb(133, 153, 0),
    process_info_block_color: Color::Rgb(7, 54, 66),
    process_title_color: Color::Rgb(181, 137, 0),
    process_text_color: Color::Rgb(133, 153, 0),
    process_selected_color_bg: Color::Rgb(7, 54, 66),
    process_selected_color_fg: Color::Rgb(214, 162, 0),
};

pub const SOLARIZED_LIGHT: AppColorInfo = AppColorInfo {
    background_color: Color::Rgb(253, 246, 227),
    base_app_text_color: Color::Rgb(88, 110, 117),
    key_text_color: Color::Rgb(181, 137, 0),
    app_title_color: Color::Rgb(0, 43, 54),
    pop_up_color: Color::Rgb(238, 232, 213),
    pop_up_selected_color_bg: Color::Rgb(181, 137, 0),
    pop_up_blur_bg: Color::Rgb(238, 232, 213),

    cpu_container_selected_color: Color::Rgb(181, 137, 0),
    cpu_main_block_color: Color::Rgb(147, 161, 161),
    cpu_selected_color: Color::Rgb(181, 137, 0),
    cpu_base_graph_color: Color::Rgb(173, 199, 0),
    cpu_info_block_color: Color::Rgb(147, 161, 161),
    cpu_text_color: Color::Rgb(173, 199, 0),

    memory_container_selected_color: Color::Rgb(181, 137, 0),
    memory_main_block_color: Color::Rgb(147, 161, 161),
    used_memory_base_graph_color: Color::Rgb(110, 23, 24),
    available_memory_base_graph_color: Color::Rgb(112, 85, 0),
    free_memory_base_graph_color: Color::Rgb(78, 89, 0),
    cached_memory_base_graph_color: Color::Rgb(17, 64, 97),
    swap_memory_base_graph_color: Color::Rgb(211, 54, 130),
    memory_text_color: Color::Rgb(17, 64, 97),

    disk_container_selected_color: Color::Rgb(181, 137, 0),
    disk_main_block_color: Color::Rgb(147, 161, 161),
    disk_bytes_written_base_graph_color: Color::Rgb(112, 28, 69),
    disk_bytes_read_base_graph_color: Color::Rgb(61, 64, 112),
    disk_text_color: Color::Rgb(17, 64, 97),

    network_container_selected_color: Color::Rgb(181, 137, 0),
    network_main_block_color: Color::Rgb(147, 161, 161),
    network_received_base_graph_color: Color::Rgb(61, 64, 112),
    network_transmitted_base_graph_color: Color::Rgb(112, 28, 69),
    network_info_block_color: Color::Rgb(147, 161, 161),
    network_text_color: Color::Rgb(61, 64, 112),

    process_container_selected_color: Color::Rgb(181, 137, 0),
    process_main_block_color: Color::Rgb(147, 161, 161),
    process_base_graph_color: Color::Rgb(211, 54, 130),
    process_info_block_color: Color::Rgb(147, 161, 161),
    process_title_color: Color::Rgb(211, 54, 130),
    process_text_color: Color::Rgb(211, 54, 130),
    process_selected_color_bg: Color::Rgb(238, 232, 213),
    process_selected_color_fg: Color::Rgb(181, 137, 0),
};
