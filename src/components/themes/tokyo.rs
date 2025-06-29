use ratatui::style::Color;

use crate::types::AppColorInfo;

pub const TOKYO_NIGHT: AppColorInfo = AppColorInfo {
    background_color: Color::Rgb(26, 27, 38),
    base_app_text_color: Color::Rgb(207, 201, 194),
    key_text_color: Color::Rgb(125, 207, 255),
    app_title_color: Color::Rgb(207, 201, 194),
    pop_up_color: Color::Rgb(65, 72, 104),
    pop_up_selected_color_bg: Color::Rgb(125, 207, 255),
    pop_up_blur_bg: Color::Rgb(86, 95, 137),

    cpu_container_selected_color: Color::Rgb(158, 206, 106),
    cpu_main_block_color: Color::Rgb(86, 95, 137),
    cpu_selected_color: Color::Rgb(125, 207, 255),
    cpu_base_graph_color: Color::Rgb(158, 206, 106),
    cpu_info_block_color: Color::Rgb(86, 95, 137),
    cpu_text_color: Color::Rgb(158, 206, 106),

    memory_container_selected_color: Color::Rgb(224, 175, 104),
    memory_main_block_color: Color::Rgb(86, 95, 137),
    used_memory_base_graph_color: Color::Rgb(247, 118, 142),
    available_memory_base_graph_color: Color::Rgb(158, 206, 106),
    free_memory_base_graph_color: Color::Rgb(158, 206, 106),
    cached_memory_base_graph_color: Color::Rgb(224, 175, 104),
    swap_memory_base_graph_color: Color::Rgb(247, 118, 142),
    memory_text_color: Color::Rgb(224, 175, 104),

    disk_container_selected_color: Color::Rgb(247, 118, 142),
    disk_main_block_color: Color::Rgb(86, 95, 137),
    disk_bytes_written_base_graph_color: Color::Rgb(158, 206, 106),
    disk_bytes_read_base_graph_color: Color::Rgb(224, 175, 104),
    disk_text_color: Color::Rgb(247, 118, 142),

    network_container_selected_color: Color::Rgb(125, 207, 255),
    network_main_block_color: Color::Rgb(86, 95, 137),
    network_received_base_graph_color: Color::Rgb(158, 206, 106),
    network_transmitted_base_graph_color: Color::Rgb(224, 175, 104),
    network_info_block_color: Color::Rgb(86, 95, 137),
    network_text_color: Color::Rgb(125, 207, 255),

    process_container_selected_color: Color::Rgb(125, 207, 255),
    process_main_block_color: Color::Rgb(86, 95, 137),
    process_base_graph_color: Color::Rgb(158, 206, 106),
    process_info_block_color: Color::Rgb(86, 95, 137),
    process_title_color: Color::Rgb(125, 207, 255),
    process_text_color: Color::Rgb(125, 207, 255),
    process_selected_color_bg: Color::Rgb(65, 72, 104),
    process_selected_color_fg: Color::Rgb(207, 201, 194),
};

pub const TOKYO_STORM: AppColorInfo = AppColorInfo {
    background_color: Color::Rgb(36, 40, 59),
    base_app_text_color: Color::Rgb(207, 201, 194),
    key_text_color: Color::Rgb(125, 207, 255),
    app_title_color: Color::Rgb(207, 201, 194),
    pop_up_color: Color::Rgb(65, 72, 104),
    pop_up_selected_color_bg: Color::Rgb(125, 207, 255),
    pop_up_blur_bg: Color::Rgb(86, 95, 137),

    cpu_container_selected_color: Color::Rgb(158, 206, 106),
    cpu_main_block_color: Color::Rgb(86, 95, 137),
    cpu_selected_color: Color::Rgb(125, 207, 255),
    cpu_base_graph_color: Color::Rgb(158, 206, 106),
    cpu_info_block_color: Color::Rgb(86, 95, 137),
    cpu_text_color: Color::Rgb(158, 206, 106),

    memory_container_selected_color: Color::Rgb(224, 175, 104),
    memory_main_block_color: Color::Rgb(86, 95, 137),
    used_memory_base_graph_color: Color::Rgb(247, 118, 142),
    available_memory_base_graph_color: Color::Rgb(158, 206, 106),
    free_memory_base_graph_color: Color::Rgb(158, 206, 106),
    cached_memory_base_graph_color: Color::Rgb(224, 175, 104),
    swap_memory_base_graph_color: Color::Rgb(247, 118, 142),
    memory_text_color: Color::Rgb(224, 175, 104),

    disk_container_selected_color: Color::Rgb(247, 118, 142),
    disk_main_block_color: Color::Rgb(86, 95, 137),
    disk_bytes_written_base_graph_color: Color::Rgb(158, 206, 106),
    disk_bytes_read_base_graph_color: Color::Rgb(224, 175, 104),
    disk_text_color: Color::Rgb(247, 118, 142),

    network_container_selected_color: Color::Rgb(125, 207, 255),
    network_main_block_color: Color::Rgb(86, 95, 137),
    network_received_base_graph_color: Color::Rgb(158, 206, 106),
    network_transmitted_base_graph_color: Color::Rgb(224, 175, 104),
    network_info_block_color: Color::Rgb(86, 95, 137),
    network_text_color: Color::Rgb(125, 207, 255),

    process_container_selected_color: Color::Rgb(125, 207, 255),
    process_main_block_color: Color::Rgb(86, 95, 137),
    process_base_graph_color: Color::Rgb(158, 206, 106),
    process_info_block_color: Color::Rgb(86, 95, 137),
    process_title_color: Color::Rgb(125, 207, 255),
    process_text_color: Color::Rgb(125, 207, 255),
    process_selected_color_bg: Color::Rgb(65, 72, 104),
    process_selected_color_fg: Color::Rgb(207, 201, 194),
};
