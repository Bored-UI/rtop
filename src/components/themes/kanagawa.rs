use ratatui::style::Color;

use crate::types::AppColorInfo;

pub const KANAGAWA_LOTUS: AppColorInfo = AppColorInfo {
    background_color: Color::Rgb(242, 236, 188),
    base_app_text_color: Color::Rgb(84, 84, 100),
    key_text_color: Color::Rgb(200, 64, 83),
    app_title_color: Color::Rgb(84, 84, 100),
    pop_up_color: Color::Rgb(138, 137, 128),
    pop_up_selected_color_bg: Color::Rgb(67, 67, 108),
    pop_up_blur_bg: Color::Rgb(138, 137, 128),

    cpu_container_selected_color: Color::Rgb(67, 67, 108),
    cpu_main_block_color: Color::Rgb(138, 137, 128),
    cpu_selected_color: Color::Rgb(67, 67, 108),
    cpu_base_graph_color: Color::Rgb(110, 145, 95),
    cpu_info_block_color: Color::Rgb(138, 137, 128),
    cpu_text_color: Color::Rgb(110, 145, 95),

    memory_container_selected_color: Color::Rgb(67, 67, 108),
    memory_main_block_color: Color::Rgb(138, 137, 128),
    used_memory_base_graph_color: Color::Rgb(181, 203, 210),
    available_memory_base_graph_color: Color::Rgb(118, 107, 144),
    free_memory_base_graph_color: Color::Rgb(215, 71, 75),
    cached_memory_base_graph_color: Color::Rgb(119, 113, 63),
    swap_memory_base_graph_color: Color::Rgb(181, 203, 210),
    memory_text_color: Color::Rgb(89, 123, 117),

    disk_container_selected_color: Color::Rgb(67, 67, 108),
    disk_main_block_color: Color::Rgb(138, 137, 128),
    disk_bytes_written_base_graph_color: Color::Rgb(204, 109, 0),
    disk_bytes_read_base_graph_color: Color::Rgb(77, 105, 155),
    disk_text_color: Color::Rgb(89, 123, 117),

    network_container_selected_color: Color::Rgb(67, 67, 108),
    network_main_block_color: Color::Rgb(138, 137, 128),
    network_received_base_graph_color: Color::Rgb(77, 105, 155),
    network_transmitted_base_graph_color: Color::Rgb(204, 109, 0),
    network_info_block_color: Color::Rgb(138, 137, 128),
    network_text_color: Color::Rgb(89, 123, 117),

    process_container_selected_color: Color::Rgb(67, 67, 108),
    process_main_block_color: Color::Rgb(138, 137, 128),
    process_base_graph_color: Color::Rgb(110, 145, 95),
    process_info_block_color: Color::Rgb(138, 137, 128),
    process_title_color: Color::Rgb(89, 123, 117),
    process_text_color: Color::Rgb(89, 123, 117),
    process_selected_color_bg: Color::Rgb(201, 203, 209),
    process_selected_color_fg: Color::Rgb(67, 67, 108),
};

pub const KANAGAWA_WAVE: AppColorInfo = AppColorInfo {
    background_color: Color::Rgb(22, 22, 29),
    base_app_text_color: Color::Rgb(220, 215, 186),
    key_text_color: Color::Rgb(195, 64, 67),
    app_title_color: Color::Rgb(220, 215, 186),
    pop_up_color: Color::Rgb(34, 50, 73),
    pop_up_selected_color_bg: Color::Rgb(220, 165, 97),
    pop_up_blur_bg: Color::Rgb(114, 113, 105),

    cpu_container_selected_color: Color::Rgb(220, 165, 97),
    cpu_main_block_color: Color::Rgb(114, 113, 105),
    cpu_selected_color: Color::Rgb(220, 165, 97),
    cpu_base_graph_color: Color::Rgb(152, 187, 108),
    cpu_info_block_color: Color::Rgb(114, 113, 105),
    cpu_text_color: Color::Rgb(152, 187, 108),

    memory_container_selected_color: Color::Rgb(220, 165, 97),
    memory_main_block_color: Color::Rgb(114, 113, 105),
    used_memory_base_graph_color: Color::Rgb(101, 133, 148),
    available_memory_base_graph_color: Color::Rgb(147, 138, 169),
    free_memory_base_graph_color: Color::Rgb(232, 36, 36),
    cached_memory_base_graph_color: Color::Rgb(192, 163, 110),
    swap_memory_base_graph_color: Color::Rgb(101, 133, 148),
    memory_text_color: Color::Rgb(122, 168, 159),

    disk_container_selected_color: Color::Rgb(220, 165, 97),
    disk_main_block_color: Color::Rgb(114, 113, 105),
    disk_bytes_written_base_graph_color: Color::Rgb(220, 165, 97),
    disk_bytes_read_base_graph_color: Color::Rgb(126, 156, 219),
    disk_text_color: Color::Rgb(122, 168, 159),

    network_container_selected_color: Color::Rgb(220, 165, 97),
    network_main_block_color: Color::Rgb(114, 113, 105),
    network_received_base_graph_color: Color::Rgb(126, 156, 219),
    network_transmitted_base_graph_color: Color::Rgb(220, 165, 97),
    network_info_block_color: Color::Rgb(114, 113, 105),
    network_text_color: Color::Rgb(126, 156, 219),

    process_container_selected_color: Color::Rgb(220, 165, 97),
    process_main_block_color: Color::Rgb(114, 113, 105),
    process_base_graph_color: Color::Rgb(152, 187, 108),
    process_info_block_color: Color::Rgb(114, 113, 105),
    process_title_color: Color::Rgb(122, 168, 159),
    process_text_color: Color::Rgb(122, 168, 159),
    process_selected_color_bg: Color::Rgb(34, 50, 73),
    process_selected_color_fg: Color::Rgb(220, 165, 97),
};
