use ratatui::style::Color;

pub struct AppColorInfo {
    pub background_color: Color,
    pub base_app_text_color: Color,
    // key text was the key that triggers certain functionality, like c for selecting cpu container -/+ to chnage the refresh tick
    pub key_text_color: Color,
    pub app_title_color: Color, // this will be used for those text in the title of each main block
    pub pop_up_color: Color,
    pub pop_up_selected_color_bg: Color,
    pub pop_up_blur_bg: Color,

    // for cpu
    pub cpu_container_selected_color: Color,
    pub cpu_main_block_color: Color,
    pub cpu_selected_color: Color,
    pub cpu_base_graph_color: Color,
    pub cpu_info_block_color: Color,
    pub cpu_text_color: Color,

    // for memory
    pub memory_container_selected_color: Color,
    pub memory_main_block_color: Color,
    pub used_memory_base_graph_color: Color,
    pub available_memory_base_graph_color: Color,
    pub free_memory_base_graph_color: Color,
    pub cached_memory_base_graph_color: Color,
    pub swap_memory_base_graph_color: Color,
    pub memory_text_color: Color,

    // for disk
    pub disk_container_selected_color: Color,
    pub disk_main_block_color: Color,
    pub disk_bytes_written_base_graph_color: Color,
    pub disk_bytes_read_base_graph_color: Color,
    pub disk_text_color: Color,

    // for network
    pub network_container_selected_color: Color,
    pub network_main_block_color: Color,
    pub network_received_base_graph_color: Color,
    pub network_transmitted_base_graph_color: Color,
    pub network_info_block_color: Color,
    pub network_text_color: Color,

    // for process
    pub process_container_selected_color: Color,
    pub process_main_block_color: Color,
    pub process_base_graph_color: Color,
    pub process_info_block_color: Color,
    pub process_title_color: Color,
    pub process_text_color: Color,
    pub process_selected_color_bg: Color,
    pub process_selected_color_fg: Color,
}
