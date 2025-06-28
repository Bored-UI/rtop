use ratatui::style::Color;

use crate::components::theme::types::AppColorInfo;

pub const EVERFOREST_DARK: AppColorInfo = AppColorInfo {
    background_color: Color::Rgb(39, 46, 51),
    base_app_text_color: Color::Rgb(211, 198, 170),
    key_text_color: Color::Rgb(230, 126, 128),
    app_title_color: Color::Rgb(211, 198, 170),
    pop_up_color: Color::Rgb(55, 65, 69),
    pop_up_selected_color_bg: Color::Rgb(219, 188, 127),
    pop_up_blur_bg: Color::Rgb(55, 65, 69),

    cpu_container_selected_color: Color::Rgb(219, 188, 127),
    cpu_main_block_color: Color::Rgb(55, 65, 69),
    cpu_selected_color: Color::Rgb(219, 188, 127),
    cpu_base_graph_color: Color::Rgb(167, 192, 128),
    cpu_info_block_color: Color::Rgb(55, 65, 69),
    cpu_text_color: Color::Rgb(167, 192, 128),

    memory_container_selected_color: Color::Rgb(219, 188, 127),
    memory_main_block_color: Color::Rgb(55, 65, 69),
    used_memory_base_graph_color: Color::Rgb(167, 192, 128),
    available_memory_base_graph_color: Color::Rgb(248, 85, 82),
    free_memory_base_graph_color: Color::Rgb(248, 85, 82),
    cached_memory_base_graph_color: Color::Rgb(127, 187, 179),
    swap_memory_base_graph_color: Color::Rgb(127, 187, 179),
    memory_text_color: Color::Rgb(127, 187, 179),

    disk_container_selected_color: Color::Rgb(219, 188, 127),
    disk_main_block_color: Color::Rgb(55, 65, 69),
    disk_bytes_written_base_graph_color: Color::Rgb(219, 188, 127),
    disk_bytes_read_base_graph_color: Color::Rgb(167, 192, 128),
    disk_text_color: Color::Rgb(127, 187, 179),

    network_container_selected_color: Color::Rgb(219, 188, 127),
    network_main_block_color: Color::Rgb(55, 65, 69),
    network_received_base_graph_color: Color::Rgb(167, 192, 128),
    network_transmitted_base_graph_color: Color::Rgb(219, 188, 127),
    network_info_block_color: Color::Rgb(55, 65, 69),
    network_text_color: Color::Rgb(219, 188, 127),

    process_container_selected_color: Color::Rgb(219, 188, 127),
    process_main_block_color: Color::Rgb(55, 65, 69),
    process_base_graph_color: Color::Rgb(167, 192, 128),
    process_info_block_color: Color::Rgb(55, 65, 69),
    process_title_color: Color::Rgb(230, 126, 128),
    process_text_color: Color::Rgb(167, 192, 128),
    process_selected_color_bg: Color::Rgb(55, 65, 69),
    process_selected_color_fg: Color::Rgb(219, 188, 127),
};

pub const EVERFOREST_LIGHT: AppColorInfo = AppColorInfo {
    background_color: Color::Rgb(253, 246, 227),
    base_app_text_color: Color::Rgb(92, 106, 114),
    key_text_color: Color::Rgb(223, 105, 186),
    app_title_color: Color::Rgb(92, 106, 114),
    pop_up_color: Color::Rgb(79, 88, 94),
    pop_up_selected_color_bg: Color::Rgb(223, 160, 0),
    pop_up_blur_bg: Color::Rgb(157, 169, 160),

    cpu_container_selected_color: Color::Rgb(223, 160, 0),
    cpu_main_block_color: Color::Rgb(79, 88, 94),
    cpu_selected_color: Color::Rgb(223, 160, 0),
    cpu_base_graph_color: Color::Rgb(141, 161, 1),
    cpu_info_block_color: Color::Rgb(79, 88, 94),
    cpu_text_color: Color::Rgb(141, 161, 1),

    memory_container_selected_color: Color::Rgb(223, 160, 0),
    memory_main_block_color: Color::Rgb(79, 88, 94),
    used_memory_base_graph_color: Color::Rgb(141, 161, 1),
    available_memory_base_graph_color: Color::Rgb(248, 85, 82),
    free_memory_base_graph_color: Color::Rgb(248, 85, 82),
    cached_memory_base_graph_color: Color::Rgb(57, 148, 197),
    swap_memory_base_graph_color: Color::Rgb(57, 148, 197),
    memory_text_color: Color::Rgb(57, 148, 197),

    disk_container_selected_color: Color::Rgb(223, 160, 0),
    disk_main_block_color: Color::Rgb(79, 88, 94),
    disk_bytes_written_base_graph_color: Color::Rgb(223, 160, 0),
    disk_bytes_read_base_graph_color: Color::Rgb(141, 161, 1),
    disk_text_color: Color::Rgb(57, 148, 197),

    network_container_selected_color: Color::Rgb(223, 160, 0),
    network_main_block_color: Color::Rgb(79, 88, 94),
    network_received_base_graph_color: Color::Rgb(141, 161, 1),
    network_transmitted_base_graph_color: Color::Rgb(223, 160, 0),
    network_info_block_color: Color::Rgb(79, 88, 94),
    network_text_color: Color::Rgb(223, 160, 0),

    process_container_selected_color: Color::Rgb(223, 160, 0),
    process_main_block_color: Color::Rgb(79, 88, 94),
    process_base_graph_color: Color::Rgb(141, 161, 1),
    process_info_block_color: Color::Rgb(79, 88, 94),
    process_title_color: Color::Rgb(223, 105, 186),
    process_text_color: Color::Rgb(141, 161, 1),
    process_selected_color_bg: Color::Rgb(79, 88, 94),
    process_selected_color_fg: Color::Rgb(223, 160, 0),
};
