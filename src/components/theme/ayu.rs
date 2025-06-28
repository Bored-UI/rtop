use ratatui::style::Color;

use crate::components::theme::types::AppColorInfo;

pub const AYU: AppColorInfo = AppColorInfo {
    background_color: Color::Rgb(11, 14, 20),
    base_app_text_color: Color::Rgb(191, 189, 182),
    key_text_color: Color::Rgb(230, 180, 80),
    app_title_color: Color::Rgb(191, 189, 182),
    pop_up_color: Color::Rgb(86, 91, 102),
    pop_up_selected_color_bg: Color::Rgb(230, 180, 80),
    pop_up_blur_bg: Color::Rgb(28, 32, 40),

    cpu_container_selected_color: Color::Rgb(223, 191, 255),
    cpu_main_block_color: Color::Rgb(86, 91, 102),
    cpu_selected_color: Color::Rgb(230, 180, 80),
    cpu_base_graph_color: Color::Rgb(223, 191, 255),
    cpu_info_block_color: Color::Rgb(86, 91, 102),
    cpu_text_color: Color::Rgb(223, 191, 255),

    memory_container_selected_color: Color::Rgb(149, 230, 203),
    memory_main_block_color: Color::Rgb(86, 91, 102),
    used_memory_base_graph_color: Color::Rgb(149, 230, 203),
    available_memory_base_graph_color: Color::Rgb(149, 230, 203),
    free_memory_base_graph_color: Color::Rgb(149, 230, 203),
    cached_memory_base_graph_color: Color::Rgb(149, 230, 203),
    swap_memory_base_graph_color: Color::Rgb(149, 230, 203),
    memory_text_color: Color::Rgb(149, 230, 203),

    disk_container_selected_color: Color::Rgb(149, 230, 203),
    disk_main_block_color: Color::Rgb(86, 91, 102),
    disk_bytes_written_base_graph_color: Color::Rgb(149, 230, 203),
    disk_bytes_read_base_graph_color: Color::Rgb(149, 230, 203),
    disk_text_color: Color::Rgb(149, 230, 203),

    network_container_selected_color: Color::Rgb(242, 135, 121),
    network_main_block_color: Color::Rgb(86, 91, 102),
    network_received_base_graph_color: Color::Rgb(242, 135, 121),
    network_transmitted_base_graph_color: Color::Rgb(115, 208, 255),
    network_info_block_color: Color::Rgb(86, 91, 102),
    network_text_color: Color::Rgb(242, 135, 121),

    process_container_selected_color: Color::Rgb(230, 182, 115),
    process_main_block_color: Color::Rgb(86, 91, 102),
    process_base_graph_color: Color::Rgb(255, 204, 102),
    process_info_block_color: Color::Rgb(86, 91, 102),
    process_title_color: Color::Rgb(230, 182, 115),
    process_text_color: Color::Rgb(223, 191, 255),
    process_selected_color_bg: Color::Rgb(230, 180, 80),
    process_selected_color_fg: Color::Rgb(248, 248, 242),
};
