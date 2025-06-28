use ratatui::style::Color;

use crate::components::theme::types::AppColorInfo;

pub const ONEDARK: AppColorInfo = AppColorInfo {
    background_color: Color::Rgb(40, 44, 52),
    base_app_text_color: Color::Rgb(171, 178, 191),
    key_text_color: Color::Rgb(97, 175, 239),
    app_title_color: Color::Rgb(171, 178, 191),
    pop_up_color: Color::Rgb(92, 99, 112),
    pop_up_selected_color_bg: Color::Rgb(97, 175, 239),
    pop_up_blur_bg: Color::Rgb(92, 99, 112),

    cpu_container_selected_color: Color::Rgb(97, 175, 239),
    cpu_main_block_color: Color::Rgb(92, 99, 112),
    cpu_selected_color: Color::Rgb(97, 175, 239),
    cpu_base_graph_color: Color::Rgb(152, 195, 121),
    cpu_info_block_color: Color::Rgb(92, 99, 112),
    cpu_text_color: Color::Rgb(152, 195, 121),

    memory_container_selected_color: Color::Rgb(229, 192, 123),
    memory_main_block_color: Color::Rgb(92, 99, 112),
    used_memory_base_graph_color: Color::Rgb(224, 108, 117),
    available_memory_base_graph_color: Color::Rgb(152, 195, 121),
    free_memory_base_graph_color: Color::Rgb(152, 195, 121),
    cached_memory_base_graph_color: Color::Rgb(229, 192, 123),
    swap_memory_base_graph_color: Color::Rgb(224, 108, 117),
    memory_text_color: Color::Rgb(229, 192, 123),

    disk_container_selected_color: Color::Rgb(224, 108, 117),
    disk_main_block_color: Color::Rgb(92, 99, 112),
    disk_bytes_written_base_graph_color: Color::Rgb(152, 195, 121),
    disk_bytes_read_base_graph_color: Color::Rgb(229, 192, 123),
    disk_text_color: Color::Rgb(224, 108, 117),

    network_container_selected_color: Color::Rgb(97, 175, 239),
    network_main_block_color: Color::Rgb(92, 99, 112),
    network_received_base_graph_color: Color::Rgb(152, 195, 121),
    network_transmitted_base_graph_color: Color::Rgb(229, 192, 123),
    network_info_block_color: Color::Rgb(92, 99, 112),
    network_text_color: Color::Rgb(97, 175, 239),

    process_container_selected_color: Color::Rgb(97, 175, 239),
    process_main_block_color: Color::Rgb(92, 99, 112),
    process_base_graph_color: Color::Rgb(152, 195, 121),
    process_info_block_color: Color::Rgb(92, 99, 112),
    process_title_color: Color::Rgb(97, 175, 239),
    process_text_color: Color::Rgb(97, 175, 239),
    process_selected_color_bg: Color::Rgb(44, 49, 60),
    process_selected_color_fg: Color::Rgb(171, 178, 191),
};
