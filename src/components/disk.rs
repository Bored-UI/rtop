use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    symbols::border,
    text::{Line, Span},
    widgets::{Block, Borders},
    Frame,
};

use crate::{tui::AppColorInfo, types::DiskData, utils::get_tick_line_ui};

// width smaller than this will be consider small width for the disk container
const SMALL_WIDTH: u16 = 20;
const MEDIUM_HEIGHT: u16 = 16;
const LARGE_HEIGHT: u16 = 21;
const DISK_GRAPH_HEIGHT_PRCENTAGE: u16 = 70;

// this was to indicate that the disk graph y axis will be either shown as 25% or 100% (based on the widget size)
const SMALL_WIDGET_PERCENTAGE: f64 = 25.0;
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
        Span::styled(" ", Style::default().fg(app_color_info.text_color)),
        Span::styled("D", Style::default().fg(app_color_info.key_text_color))
            .bold()
            .underlined(),
        Span::styled("isk ", Style::default().fg(app_color_info.text_color)),
    ]);

    let disk_switch_instruction = Line::from(vec![
        Span::styled("| ", Style::default().fg(app_color_info.text_color)),
        Span::styled("<", Style::default().fg(app_color_info.key_text_color)).bold(),
        Span::styled(
            format!(" {} ", disk_name),
            Style::default().fg(app_color_info.text_color),
        ),
        Span::styled(">", Style::default().fg(app_color_info.key_text_color)).bold(),
        Span::styled(" |", Style::default().fg(app_color_info.text_color)),
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

    let [_, bottom_border, _] = Layout::vertical([
        Constraint::Percentage(5),
        Constraint::Percentage(90),
        Constraint::Percentage(5),
    ])
    .areas(area);
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

    let total_disk_space_label = Line::from("Total:").style(app_color_info.text_color);
    let total_disk_space =
        Line::from(format!("{} GiB", disk_data.total_space)).style(app_color_info.text_color);
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
    // total bytes written
    // total bytes read
    // file system
    // mount point
    // current written bytes [graph]
    // current read bytes [graph]

    let [used_space_layout, available_space_layout, total_bytes_written_layout, total_bytes_read_layout, file_system_layout, mount_point_layout, current_written_bytes_layout, current_read_bytes_layout] =
        Layout::vertical([
            Constraint::Percentage(5),
            Constraint::Percentage(5),
            Constraint::Percentage(5),
            Constraint::Percentage(5),
            Constraint::Percentage(5),
            Constraint::Percentage(5),
            Constraint::Percentage(35),
            Constraint::Percentage(35),
        ])
        .areas(bottom_blocks);

    let used_space_label = if used_space_layout.width < SMALL_WIDTH {
        Line::from("U").style(app_color_info.text_color)
    } else {
        Line::from("Used:").style(app_color_info.text_color)
    };

    let used_space_usage =
        Line::from(format!("{} GiB", disk_data.used_space)).style(app_color_info.text_color);
    let used_space_block = Block::bordered()
        .title(used_space_label.left_aligned())
        .title(used_space_usage.right_aligned())
        .style(app_color_info.disk_main_block_color)
        .borders(Borders::NONE);

    frame.render_widget(used_space_block, used_space_layout);
}
