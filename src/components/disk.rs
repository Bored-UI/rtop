use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    symbols::{border, Marker},
    text::{Line, Span},
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType},
    Frame,
};

use crate::{tui::AppColorInfo, types::DiskData, utils::get_tick_line_ui};

// width smaller than this will be consider small width for the disk container
const SMALL_WIDTH: u16 = 20;
const DISK_GRAPH_HEIGHT_PRCENTAGE: u16 = 70;

// this was to indicate that the disk graph y axis will be either shown as 25% or 100% (based on the widget size)
const SMALL_WIDGET_PERCENTAGE: f64 = 50.0;
const BIG_WIDGET_PERCENTAGE: f64 = 100.0;

pub fn draw_disk_info(
    tick: u64,
    disk_data: &DiskData,
    area: Rect,
    frame: &mut Frame,
    graph_show_range: usize,
    is_selected: bool,
    app_color_info: &AppColorInfo,
    is_full_screen: bool,
) {
    let current_graph_percentage = if is_full_screen {
        BIG_WIDGET_PERCENTAGE
    } else {
        SMALL_WIDGET_PERCENTAGE
    };

    let mut disk_name = disk_data.name.clone();
    if area.width <= SMALL_WIDTH + 5 {
        let extension = if disk_name.len() > 8 { ".." } else { "" };
        let new_disk_name = disk_name.get(..8).unwrap_or(disk_name.as_str());
        let new_disk_name_with_ext = new_disk_name.to_string() + extension;
        disk_name = new_disk_name_with_ext;
    } else if area.width <= SMALL_WIDTH + 20 {
        let extension = if disk_name.len() > 25 { ".." } else { "" };
        let new_disk_name = disk_name.get(..25).unwrap_or(disk_name.as_str());
        let new_disk_name_with_ext = new_disk_name.to_string() + extension;
        disk_name = new_disk_name_with_ext;
    }

    let select_instruction = Line::from(vec![
        Span::styled(" ", Style::default().fg(app_color_info.app_title_color)),
        Span::styled("D", Style::default().fg(app_color_info.key_text_color))
            .bold()
            .underlined(),
        Span::styled("isk ", Style::default().fg(app_color_info.app_title_color)),
    ]);

    let disk_switch_instruction = Line::from(vec![
        Span::styled("  ", Style::default().fg(app_color_info.app_title_color)),
        Span::styled("<", Style::default().fg(app_color_info.key_text_color)).bold(),
        Span::styled(
            format!(" {} ", disk_name),
            Style::default().fg(app_color_info.app_title_color),
        ),
        Span::styled(">", Style::default().fg(app_color_info.key_text_color)).bold(),
        Span::styled("  ", Style::default().fg(app_color_info.app_title_color)),
    ]);

    let mut main_block = Block::bordered()
        .title(select_instruction.left_aligned())
        .title_bottom(disk_switch_instruction.centered())
        .style(app_color_info.disk_main_block_color)
        .border_set(border::ROUNDED);
    if is_selected {
        main_block = main_block
            .style(app_color_info.disk_container_selected_color)
            .border_set(border::DOUBLE);
    }
    if is_full_screen {
        let refresh_tick = get_tick_line_ui(tick, app_color_info);

        main_block = main_block.title(refresh_tick.right_aligned())
    }

    // bottom border will be the space where the statistics for used, available space, total bytes written and read etc... will be displayed
    let [_, bottom_border, _] = Layout::vertical([
        Constraint::Percentage(5),
        Constraint::Percentage(90),
        Constraint::Percentage(5),
    ])
    .areas(area);

    // padded the bottom border for some space on the left and right
    let [_, padded_bottom, _] = Layout::horizontal([
        Constraint::Percentage(3),
        Constraint::Percentage(94),
        Constraint::Percentage(3),
    ])
    .areas(bottom_border);

    // top label will be the label for total disk space
    // bottom blocks will be the statistics for used, available space, total bytes written and read etc...
    let [top_label, bottom_blocks] =
        Layout::vertical([Constraint::Percentage(10), Constraint::Percentage(90)])
            .areas(padded_bottom);

    let total_disk_space_label = Line::from("Total:").style(app_color_info.app_title_color);
    let total_disk_space =
        Line::from(format!("{} GiB", disk_data.total_space)).style(app_color_info.app_title_color);
    let top_inner_block = Block::new()
        .title(total_disk_space_label.left_aligned())
        .title(total_disk_space.right_aligned())
        .style(app_color_info.disk_main_block_color)
        .borders(Borders::NONE);

    frame.render_widget(main_block, area);
    frame.render_widget(top_inner_block, top_label);

    // bottom block will be in the follwing order:
    // used space
    // available space
    // file system
    // mount point
    // kind
    // current written bytes [graph]
    // current read bytes [graph]

    let [used_space_layout, available_space_layout, file_system_layout, mount_point_layout, disk_kind_layout, current_bytes_written_layout, current_bytes_read_layout] =
        Layout::vertical([
            Constraint::Percentage(6),
            Constraint::Percentage(6),
            Constraint::Percentage(6),
            Constraint::Percentage(6),
            Constraint::Percentage(6),
            Constraint::Percentage(35),
            Constraint::Percentage(35),
        ])
        .areas(bottom_blocks);

    let border_type = if bottom_blocks.width < SMALL_WIDTH {
        Borders::NONE
    } else {
        Borders::TOP
    };

    // ----------------------------------------
    //
    //       FOR USED DISK SPACE LAYOUT
    //
    // ----------------------------------------
    let used_space_label = if used_space_layout.width < SMALL_WIDTH {
        Line::from("U").style(app_color_info.base_app_text_color)
    } else {
        Line::from("Used:").style(app_color_info.base_app_text_color)
    };

    let used_space_usage =
        Line::from(format!("{} GiB", disk_data.used_space)).style(app_color_info.disk_text_color);
    let used_space_block = Block::bordered()
        .title(used_space_label.left_aligned())
        .title(used_space_usage.right_aligned())
        .style(app_color_info.disk_main_block_color)
        .borders(border_type);

    frame.render_widget(used_space_block, used_space_layout);

    // ----------------------------------------
    //
    //     FOR AVAILABLE DISK SPACE LAYOUT
    //
    // ----------------------------------------
    let available_space_label = if available_space_layout.width < SMALL_WIDTH + 10 {
        Line::from("A").style(app_color_info.base_app_text_color)
    } else {
        Line::from("Available:").style(app_color_info.base_app_text_color)
    };

    let available_space_usage = Line::from(format!("{} GiB", disk_data.available_space))
        .style(app_color_info.disk_text_color);
    let available_space_block = Block::bordered()
        .title(available_space_label.left_aligned())
        .title(available_space_usage.right_aligned())
        .style(app_color_info.disk_main_block_color)
        .borders(border_type);

    frame.render_widget(available_space_block, available_space_layout);

    // ----------------------------------------
    //
    //     FOR DISK FILE SYSTEM LAYOUT
    //
    // ----------------------------------------
    let file_system_label = if file_system_layout.width < SMALL_WIDTH + 10 {
        Line::from("F/S").style(app_color_info.base_app_text_color)
    } else {
        Line::from("File System:").style(app_color_info.base_app_text_color)
    };

    let mut file_system = disk_data.file_system.clone();
    if area.width <= SMALL_WIDTH + 5 {
        let extension = if file_system.len() > 8 { ".." } else { "" };
        let new_file_system = file_system.get(..8).unwrap_or(file_system.as_str());
        let new_file_system_with_ext = new_file_system.to_string() + extension;
        file_system = new_file_system_with_ext;
    } else if area.width <= SMALL_WIDTH + 20 {
        let extension = if file_system.len() > 25 { ".." } else { "" };
        let new_file_system = file_system.get(..25).unwrap_or(file_system.as_str());
        let new_file_system_with_ext = new_file_system.to_string() + extension;
        file_system = new_file_system_with_ext;
    }

    let file_system_usage =
        Line::from(format!("{}", file_system)).style(app_color_info.disk_text_color);
    let file_system_block = Block::bordered()
        .title(file_system_label.left_aligned())
        .title(file_system_usage.right_aligned())
        .style(app_color_info.disk_main_block_color)
        .borders(border_type);

    frame.render_widget(file_system_block, file_system_layout);

    // ----------------------------------------
    //
    //     FOR DISK MOUNT POINT LAYOUT
    //
    // ----------------------------------------
    let mount_point_label = if mount_point_layout.width < SMALL_WIDTH + 10 {
        Line::from("M/P").style(app_color_info.base_app_text_color)
    } else {
        Line::from("Mount Point:").style(app_color_info.base_app_text_color)
    };

    let mut mount_point = disk_data.mount_point.clone();
    if area.width <= SMALL_WIDTH + 5 {
        let extension = if mount_point.len() > 8 { ".." } else { "" };
        let new_mount_point = mount_point.get(..8).unwrap_or(mount_point.as_str());
        let new_mount_point_with_ext = new_mount_point.to_string() + extension;
        mount_point = new_mount_point_with_ext;
    } else if area.width <= SMALL_WIDTH + 20 || !is_full_screen {
        let extension = if mount_point.len() > 25 { ".." } else { "" };
        let new_mount_point = mount_point.get(..25).unwrap_or(mount_point.as_str());
        let new_mount_point_with_ext = new_mount_point.to_string() + extension;
        mount_point = new_mount_point_with_ext;
    }

    let mount_point_usage =
        Line::from(format!("{}", mount_point)).style(app_color_info.memory_text_color);
    let mount_point_block = Block::bordered()
        .title(mount_point_label.left_aligned())
        .title(mount_point_usage.right_aligned())
        .style(app_color_info.disk_main_block_color)
        .borders(border_type);

    frame.render_widget(mount_point_block, mount_point_layout);

    // ----------------------------------------
    //
    //     FOR DISK KIND LAYOUT
    //
    // ----------------------------------------
    let disk_kind_label = if disk_kind_layout.width < SMALL_WIDTH {
        Line::from("K").style(app_color_info.base_app_text_color)
    } else {
        Line::from("Disk Kind:").style(app_color_info.base_app_text_color)
    };

    let disk_kind_usage =
        Line::from(format!("{}", disk_data.disk_kind)).style(app_color_info.disk_text_color);
    let disk_kind_block = Block::bordered()
        .title(disk_kind_label.left_aligned())
        .title(disk_kind_usage.right_aligned())
        .style(app_color_info.disk_main_block_color)
        .borders(border_type);

    frame.render_widget(disk_kind_block, disk_kind_layout);

    // ----------------------------------------
    //
    //          FOR BYTES WRITTEN LAYOUT
    //
    // ----------------------------------------
    let [_, bytes_written_graph] = Layout::vertical([
        Constraint::Percentage(100 - DISK_GRAPH_HEIGHT_PRCENTAGE),
        Constraint::Percentage(DISK_GRAPH_HEIGHT_PRCENTAGE),
    ])
    .areas(current_bytes_written_layout);
    let bytes_written_label = if current_bytes_written_layout.width < SMALL_WIDTH {
        Line::from("W").style(app_color_info.base_app_text_color)
    } else {
        Line::from("WRITE:").style(app_color_info.base_app_text_color)
    };

    let mut actual_bytes = disk_data.bytes_written_vec[disk_data.bytes_written_vec.len() - 1];
    let mut bytes_format = "KiB";

    if actual_bytes > 1024.0 {
        actual_bytes /= 1024.0;
        actual_bytes = (actual_bytes * 1000.0).round() / 1000.0;
        bytes_format = "MiB";

        if actual_bytes > 1024.0 {
            actual_bytes /= 1024.0;
            actual_bytes = (actual_bytes * 1000.0).round() / 1000.0;
            bytes_format = "GiB";
        }
    }

    let bytes_written_usage = Line::from(format!(
        "{} {} {}",
        if actual_bytes > 0.0 { "▲" } else { "" },
        actual_bytes,
        bytes_format
    ))
    .style(app_color_info.memory_text_color);

    let bytes_written_block = Block::new()
        .title(bytes_written_label.left_aligned())
        .title(bytes_written_usage.right_aligned())
        .style(app_color_info.memory_main_block_color)
        .borders(border_type);

    let bytes_written_history = disk_data.bytes_written_vec.clone();
    let num_points_to_display = graph_show_range.min(bytes_written_history.len());
    let start_idx = bytes_written_history
        .len()
        .saturating_sub(num_points_to_display);

    let mut current_max_written_bytes: f64 = 0.0;
    bytes_written_history[start_idx..].iter().for_each(|usage| {
        current_max_written_bytes = current_max_written_bytes.max(*usage);
    });

    let bytes_written_data_points: Vec<(f64, f64)> = bytes_written_history[start_idx..]
        .iter()
        .enumerate()
        .map(|(i, &usage)| {
            let x = i as f64;
            let y = (usage / current_max_written_bytes) * current_graph_percentage as f64;
            (x, y)
        })
        .collect();

    let dataset = Dataset::default()
        .data(&bytes_written_data_points)
        .graph_type(GraphType::Bar)
        .marker(Marker::Braille)
        .style(Style::default().fg(app_color_info.disk_bytes_written_base_graph_color));

    let x_axis = Axis::default().bounds([0.0, num_points_to_display as f64]);

    let y_axis = Axis::default().bounds([0.0, current_graph_percentage]);

    let bytes_written_chart = Chart::new(vec![dataset])
        .x_axis(x_axis)
        .y_axis(y_axis)
        .bg(app_color_info.background_color);

    frame.render_widget(bytes_written_block, current_bytes_written_layout);
    frame.render_widget(bytes_written_chart, bytes_written_graph);

    drop(bytes_written_history);
    drop(bytes_written_data_points);

    // ----------------------------------------
    //
    //          FOR BYTES READ LAYOUT
    //
    // ----------------------------------------
    let [_, bytes_read_graph] = Layout::vertical([
        Constraint::Percentage(100 - DISK_GRAPH_HEIGHT_PRCENTAGE),
        Constraint::Percentage(DISK_GRAPH_HEIGHT_PRCENTAGE),
    ])
    .areas(current_bytes_read_layout);
    let bytes_read_label = if current_bytes_read_layout.width < SMALL_WIDTH {
        Line::from("R").style(app_color_info.base_app_text_color)
    } else {
        Line::from("READ:").style(app_color_info.base_app_text_color)
    };

    let mut actual_bytes = disk_data.bytes_read_vec[disk_data.bytes_read_vec.len() - 1];
    let mut bytes_format = "KiB";

    if actual_bytes > 1024.0 {
        actual_bytes /= 1024.0;
        actual_bytes = (actual_bytes * 1000.0).round() / 1000.0;
        bytes_format = "MiB";

        if actual_bytes > 1024.0 {
            actual_bytes /= 1024.0;
            actual_bytes = (actual_bytes * 1000.0).round() / 1000.0;
            bytes_format = "GiB";
        }
    }

    let bytes_read_usage = Line::from(format!(
        "{} {} {}",
        if actual_bytes > 0.0 { "▲" } else { "" },
        actual_bytes,
        bytes_format
    ))
    .style(app_color_info.memory_text_color);

    let bytes_read_block = Block::new()
        .title(bytes_read_label.left_aligned())
        .title(bytes_read_usage.right_aligned())
        .style(app_color_info.memory_main_block_color)
        .borders(border_type);

    let bytes_read_history = disk_data.bytes_read_vec.clone();
    let num_points_to_display = graph_show_range.min(bytes_read_history.len());
    let start_idx = bytes_read_history
        .len()
        .saturating_sub(num_points_to_display);

    let mut current_max_read_bytes: f64 = 0.0;
    bytes_read_history[start_idx..].iter().for_each(|usage| {
        current_max_read_bytes = current_max_read_bytes.max(*usage);
    });

    let bytes_read_data_points: Vec<(f64, f64)> = bytes_read_history[start_idx..]
        .iter()
        .enumerate()
        .map(|(i, &usage)| {
            let x = i as f64;
            let y = (usage / current_max_read_bytes) * current_graph_percentage as f64;
            (x, y)
        })
        .collect();

    let dataset = Dataset::default()
        .data(&bytes_read_data_points)
        .graph_type(GraphType::Bar)
        .marker(Marker::Braille)
        .style(Style::default().fg(app_color_info.disk_bytes_read_base_graph_color));

    let x_axis = Axis::default().bounds([0.0, num_points_to_display as f64]);

    let y_axis = Axis::default().bounds([0.0, current_graph_percentage]);

    let bytes_read_chart = Chart::new(vec![dataset])
        .x_axis(x_axis)
        .y_axis(y_axis)
        .bg(app_color_info.background_color);

    frame.render_widget(bytes_read_block, current_bytes_read_layout);
    frame.render_widget(bytes_read_chart, bytes_read_graph);

    drop(bytes_read_history);
    drop(bytes_read_data_points);
}
