use ratatui::style::Color;

use crate::components::theme::types::AppColorInfo;

pub const ROSE_PINE: AppColorInfo = AppColorInfo {
    // Background: Dark gray-purple
    background_color: Color::Rgb(25, 23, 36), // Rosé Pine Base
    // General text: Light gray for readability
    base_app_text_color: Color::Rgb(224, 222, 244), // Rosé Pine Text
    // Key text: Blue for emphasis
    key_text_color: Color::Rgb(122, 162, 247), // Rosé Pine Iris
    // Title: Pink for prominence
    app_title_color: Color::Rgb(235, 188, 186), // Rosé Pine Love
    // Pop-up background: Slightly lighter than main background
    pop_up_color: Color::Rgb(43, 42, 51), // Rosé Pine Surface
    pop_up_selected_color_bg: Color::Rgb(43, 42, 51), // Same for selection
    // Dimming layer: Lighter shade of background
    pop_up_blur_bg: Color::Rgb(35, 34, 45), // Lighter Rosé Pine Base

    // CPU
    cpu_container_selected_color: Color::Rgb(122, 162, 247), // Rosé Pine Iris
    cpu_main_block_color: Color::Rgb(43, 42, 51),            // Rosé Pine Surface
    cpu_selected_color: Color::Rgb(122, 162, 247),           // Rosé Pine Iris
    cpu_base_graph_color: Color::Rgb(166, 218, 149),         // Rosé Pine Pine
    cpu_info_block_color: Color::Rgb(43, 42, 51),            // Rosé Pine Surface
    cpu_text_color: Color::Rgb(122, 162, 247),               // Rosé Pine Iris

    // Memory
    memory_container_selected_color: Color::Rgb(122, 162, 247), // Rosé Pine Iris
    memory_main_block_color: Color::Rgb(43, 42, 51),            // Rosé Pine Surface
    used_memory_base_graph_color: Color::Rgb(166, 218, 149),    // Rosé Pine Pine
    available_memory_base_graph_color: Color::Rgb(166, 218, 149), // Rosé Pine Pine
    free_memory_base_graph_color: Color::Rgb(166, 218, 149),    // Rosé Pine Pine
    cached_memory_base_graph_color: Color::Rgb(166, 218, 149),  // Rosé Pine Pine
    swap_memory_base_graph_color: Color::Rgb(166, 218, 149),    // Rosé Pine Pine
    memory_text_color: Color::Rgb(235, 188, 186),               // Rosé Pine Love

    // Disk
    disk_container_selected_color: Color::Rgb(122, 162, 247), // Rosé Pine Iris
    disk_main_block_color: Color::Rgb(43, 42, 51),            // Rosé Pine Surface
    disk_bytes_written_base_graph_color: Color::Rgb(166, 218, 149), // Rosé Pine Pine
    disk_bytes_read_base_graph_color: Color::Rgb(166, 218, 149), // Rosé Pine Pine
    disk_text_color: Color::Rgb(235, 188, 186),               // Rosé Pine Love

    // Network
    network_container_selected_color: Color::Rgb(122, 162, 247), // Rosé Pine Iris
    network_main_block_color: Color::Rgb(43, 42, 51),            // Rosé Pine Surface
    network_received_base_graph_color: Color::Rgb(166, 218, 149), // Rosé Pine Pine
    network_transmitted_base_graph_color: Color::Rgb(166, 218, 149), // Rosé Pine Pine
    network_info_block_color: Color::Rgb(43, 42, 51),            // Rosé Pine Surface
    network_text_color: Color::Rgb(235, 188, 186),               // Rosé Pine Love

    // Process
    process_container_selected_color: Color::Rgb(122, 162, 247), // Rosé Pine Iris
    process_main_block_color: Color::Rgb(43, 42, 51),            // Rosé Pine Surface
    process_base_graph_color: Color::Rgb(166, 218, 149),         // Rosé Pine Pine
    process_info_block_color: Color::Rgb(43, 42, 51),            // Rosé Pine Surface
    process_title_color: Color::Rgb(235, 188, 186),              // Rosé Pine Love
    process_text_color: Color::Rgb(122, 162, 247),               // Rosé Pine Iris
    process_selected_color_bg: Color::Rgb(43, 42, 51),           // Rosé Pine Surface
    process_selected_color_fg: Color::Rgb(224, 222, 244),        // Rosé Pine Text
};
