use ratatui::style::Color;

use crate::types::AppColorInfo;

pub const DRACULA: AppColorInfo = AppColorInfo {
    background_color: Color::Rgb(40, 42, 54),
    base_app_text_color: Color::Rgb(248, 248, 242),
    key_text_color: Color::Rgb(98, 114, 164),
    app_title_color: Color::Rgb(248, 248, 242),
    pop_up_color: Color::Rgb(68, 71, 90),
    pop_up_selected_color_bg: Color::Rgb(255, 121, 198),
    pop_up_blur_bg: Color::Rgb(68, 71, 90),

    cpu_container_selected_color: Color::Rgb(189, 147, 249),
    cpu_main_block_color: Color::Rgb(68, 71, 90),
    cpu_selected_color: Color::Rgb(255, 121, 198),
    cpu_base_graph_color: Color::Rgb(189, 147, 249),
    cpu_info_block_color: Color::Rgb(68, 71, 90),
    cpu_text_color: Color::Rgb(189, 147, 249),

    memory_container_selected_color: Color::Rgb(80, 250, 123),
    memory_main_block_color: Color::Rgb(68, 71, 90),
    used_memory_base_graph_color: Color::Rgb(150, 250, 175),
    available_memory_base_graph_color: Color::Rgb(255, 212, 166),
    free_memory_base_graph_color: Color::Rgb(255, 166, 217),
    cached_memory_base_graph_color: Color::Rgb(177, 240, 253),
    swap_memory_base_graph_color: Color::Rgb(255, 166, 217),
    memory_text_color: Color::Rgb(80, 250, 123),

    disk_container_selected_color: Color::Rgb(80, 250, 123),
    disk_main_block_color: Color::Rgb(68, 71, 90),
    disk_bytes_written_base_graph_color: Color::Rgb(150, 250, 175),
    disk_bytes_read_base_graph_color: Color::Rgb(255, 212, 166),
    disk_text_color: Color::Rgb(80, 250, 123),

    network_container_selected_color: Color::Rgb(255, 85, 85),
    network_main_block_color: Color::Rgb(68, 71, 90),
    network_received_base_graph_color: Color::Rgb(189, 147, 249),
    network_transmitted_base_graph_color: Color::Rgb(140, 66, 171),
    network_info_block_color: Color::Rgb(68, 71, 90),
    network_text_color: Color::Rgb(255, 85, 85),

    process_container_selected_color: Color::Rgb(139, 233, 253),
    process_main_block_color: Color::Rgb(68, 71, 90),
    process_base_graph_color: Color::Rgb(80, 250, 123),
    process_info_block_color: Color::Rgb(68, 71, 90),
    process_title_color: Color::Rgb(139, 233, 253),
    process_text_color: Color::Rgb(189, 147, 249),
    process_selected_color_bg: Color::Rgb(255, 121, 198),
    process_selected_color_fg: Color::Rgb(248, 248, 242),
};
