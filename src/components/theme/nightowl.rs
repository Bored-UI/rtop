use ratatui::style::Color;

use crate::components::theme::types::AppColorInfo;

pub const NIGHT_OWL: AppColorInfo = AppColorInfo {
    background_color: Color::Rgb(1, 22, 39),
    base_app_text_color: Color::Rgb(214, 222, 235),
    key_text_color: Color::Rgb(173, 219, 103),
    app_title_color: Color::Rgb(255, 255, 255),
    pop_up_color: Color::Rgb(87, 86, 86),
    pop_up_selected_color_bg: Color::Rgb(255, 235, 149),
    pop_up_blur_bg: Color::Rgb(87, 86, 86),

    cpu_container_selected_color: Color::Rgb(255, 235, 149),
    cpu_main_block_color: Color::Rgb(87, 86, 86),
    cpu_selected_color: Color::Rgb(255, 235, 149),
    cpu_base_graph_color: Color::Rgb(34, 218, 110),
    cpu_info_block_color: Color::Rgb(87, 86, 86),
    cpu_text_color: Color::Rgb(34, 218, 110),

    memory_container_selected_color: Color::Rgb(255, 235, 149),
    memory_main_block_color: Color::Rgb(87, 86, 86),
    used_memory_base_graph_color: Color::Rgb(239, 83, 80),
    available_memory_base_graph_color: Color::Rgb(173, 219, 103),
    free_memory_base_graph_color: Color::Rgb(34, 218, 110),
    cached_memory_base_graph_color: Color::Rgb(130, 170, 255),
    swap_memory_base_graph_color: Color::Rgb(130, 170, 255),
    memory_text_color: Color::Rgb(130, 170, 255),

    disk_container_selected_color: Color::Rgb(255, 235, 149),
    disk_main_block_color: Color::Rgb(87, 86, 86),
    disk_bytes_written_base_graph_color: Color::Rgb(112, 28, 69),
    disk_bytes_read_base_graph_color: Color::Rgb(61, 64, 112),
    disk_text_color: Color::Rgb(130, 170, 255),

    network_container_selected_color: Color::Rgb(255, 235, 149),
    network_main_block_color: Color::Rgb(87, 86, 86),
    network_received_base_graph_color: Color::Rgb(61, 64, 112),
    network_transmitted_base_graph_color: Color::Rgb(112, 28, 69),
    network_info_block_color: Color::Rgb(87, 86, 86),
    network_text_color: Color::Rgb(199, 146, 234),

    process_container_selected_color: Color::Rgb(255, 235, 149),
    process_main_block_color: Color::Rgb(87, 86, 86),
    process_base_graph_color: Color::Rgb(34, 218, 110),
    process_info_block_color: Color::Rgb(87, 86, 86),
    process_title_color: Color::Rgb(173, 219, 103),
    process_text_color: Color::Rgb(34, 218, 110),
    process_selected_color_bg: Color::Rgb(0, 0, 0),
    process_selected_color_fg: Color::Rgb(255, 235, 149),
};
