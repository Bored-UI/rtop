use ratatui::style::Color;

use crate::types::AppColorInfo;

pub const MATCHA_DARK_SEA: AppColorInfo = AppColorInfo {
    background_color: Color::Rgb(27, 27, 27),
    base_app_text_color: Color::Rgb(248, 248, 242),
    key_text_color: Color::Rgb(46, 179, 152),
    app_title_color: Color::Rgb(248, 248, 242),
    pop_up_color: Color::Rgb(89, 86, 71),
    pop_up_selected_color_bg: Color::Rgb(46, 179, 152),
    pop_up_blur_bg: Color::Rgb(89, 86, 71),

    cpu_container_selected_color: Color::Rgb(51, 177, 101),
    cpu_main_block_color: Color::Rgb(89, 86, 71),
    cpu_selected_color: Color::Rgb(46, 179, 152),
    cpu_base_graph_color: Color::Rgb(51, 177, 101),
    cpu_info_block_color: Color::Rgb(89, 86, 71),
    cpu_text_color: Color::Rgb(51, 177, 101),

    memory_container_selected_color: Color::Rgb(46, 179, 152),
    memory_main_block_color: Color::Rgb(89, 86, 71),
    used_memory_base_graph_color: Color::Rgb(46, 179, 152),
    available_memory_base_graph_color: Color::Rgb(230, 219, 116),
    free_memory_base_graph_color: Color::Rgb(117, 113, 94),
    cached_memory_base_graph_color: Color::Rgb(102, 217, 239),
    swap_memory_base_graph_color: Color::Rgb(121, 118, 183),
    memory_text_color: Color::Rgb(46, 179, 152),

    disk_container_selected_color: Color::Rgb(121, 118, 183),
    disk_main_block_color: Color::Rgb(89, 86, 71),
    disk_bytes_written_base_graph_color: Color::Rgb(13, 73, 61),
    disk_bytes_read_base_graph_color: Color::Rgb(45, 32, 66),
    disk_text_color: Color::Rgb(121, 118, 183),

    network_container_selected_color: Color::Rgb(51, 177, 101),
    network_main_block_color: Color::Rgb(89, 86, 71),
    network_received_base_graph_color: Color::Rgb(45, 32, 66),
    network_transmitted_base_graph_color: Color::Rgb(13, 73, 61),
    network_info_block_color: Color::Rgb(89, 86, 71),
    network_text_color: Color::Rgb(46, 179, 152),

    process_container_selected_color: Color::Rgb(46, 179, 152),
    process_main_block_color: Color::Rgb(89, 86, 71),
    process_base_graph_color: Color::Rgb(51, 177, 101),
    process_info_block_color: Color::Rgb(89, 86, 71),
    process_title_color: Color::Rgb(46, 179, 152),
    process_text_color: Color::Rgb(51, 177, 101),
    process_selected_color_bg: Color::Rgb(13, 73, 61),
    process_selected_color_fg: Color::Rgb(248, 248, 242),
};
