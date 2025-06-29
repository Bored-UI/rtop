use ratatui::style::Color;

use crate::types::AppColorInfo;

pub const MONOKAI: AppColorInfo = AppColorInfo {
    background_color: Color::Rgb(6, 6, 4),
    base_app_text_color: Color::Rgb(248, 248, 242),
    key_text_color: Color::Rgb(249, 38, 114),
    app_title_color: Color::Rgb(248, 248, 242),
    pop_up_color: Color::Rgb(89, 86, 71),
    pop_up_selected_color_bg: Color::Rgb(249, 38, 114),
    pop_up_blur_bg: Color::Rgb(89, 86, 71),

    cpu_container_selected_color: Color::Rgb(166, 226, 46),
    cpu_main_block_color: Color::Rgb(89, 86, 71),
    cpu_selected_color: Color::Rgb(249, 38, 114),
    cpu_base_graph_color: Color::Rgb(166, 226, 46),
    cpu_info_block_color: Color::Rgb(89, 86, 71),
    cpu_text_color: Color::Rgb(166, 226, 46),

    memory_container_selected_color: Color::Rgb(102, 217, 239),
    memory_main_block_color: Color::Rgb(89, 86, 71),
    used_memory_base_graph_color: Color::Rgb(249, 38, 114),
    available_memory_base_graph_color: Color::Rgb(230, 219, 116),
    free_memory_base_graph_color: Color::Rgb(117, 113, 94),
    cached_memory_base_graph_color: Color::Rgb(102, 217, 239),
    swap_memory_base_graph_color: Color::Rgb(121, 118, 183),
    memory_text_color: Color::Rgb(102, 217, 239),

    disk_container_selected_color: Color::Rgb(230, 219, 116),
    disk_main_block_color: Color::Rgb(89, 86, 71),
    disk_bytes_written_base_graph_color: Color::Rgb(207, 39, 125),
    disk_bytes_read_base_graph_color: Color::Rgb(115, 82, 168),
    disk_text_color: Color::Rgb(230, 219, 116),

    network_container_selected_color: Color::Rgb(121, 118, 183),
    network_main_block_color: Color::Rgb(89, 86, 71),
    network_received_base_graph_color: Color::Rgb(45, 32, 66),
    network_transmitted_base_graph_color: Color::Rgb(87, 13, 51),
    network_info_block_color: Color::Rgb(89, 86, 71),
    network_text_color: Color::Rgb(121, 118, 183),

    process_container_selected_color: Color::Rgb(249, 38, 114),
    process_main_block_color: Color::Rgb(89, 86, 71),
    process_base_graph_color: Color::Rgb(166, 226, 46),
    process_info_block_color: Color::Rgb(89, 86, 71),
    process_title_color: Color::Rgb(249, 38, 114),
    process_text_color: Color::Rgb(166, 226, 46),
    process_selected_color_bg: Color::Rgb(122, 17, 55),
    process_selected_color_fg: Color::Rgb(248, 248, 242),
};
