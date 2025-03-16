use chrono::Local;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    symbols::{border, Marker},
    text::{Line, Span},
    widgets::{Axis, Block, Chart, Dataset, GraphType, List, ListItem, ListState},
    Frame,
};

use crate::{tui::AppColorInfo, types::CpuData};

pub fn draw_cpu_info(
    tick: u64,
    cpus: &Vec<CpuData>,
    size: Rect,
    frame: &mut Frame,
    cpu_selected_state: &mut ListState,
    graph_show_range: usize,
    is_selected: bool,
    app_color_info: &AppColorInfo,
) {
    let local_time = Local::now();

    let title = Line::from(
        format!(" {} ", local_time.format("%H:%M:%S"))
            .bold()
            .style(app_color_info.text_color),
    );
    let refresh_tick = Line::from(vec![
        Span::styled("| ", Style::default().fg(app_color_info.text_color)),
        Span::styled("-", Style::default().fg(app_color_info.key_text_color)).bold(),
        Span::styled(
            format!(" {}ms ", tick),
            Style::default().fg(app_color_info.text_color),
        ),
        Span::styled("+", Style::default().fg(app_color_info.key_text_color)).bold(),
        Span::styled(" |", Style::default().fg(app_color_info.text_color)),
    ]);
    let select_instruction = Line::from(vec![
        Span::styled(" ", Style::default().fg(app_color_info.text_color)),
        Span::styled("C", Style::default().fg(app_color_info.key_text_color))
            .bold()
            .underlined(),
        Span::styled("pu ", Style::default().fg(app_color_info.text_color)),
    ]);

    // The main block for CPU info
    let mut main_block = Block::bordered()
        .title(title.centered())
        .title(select_instruction.left_aligned())
        .title(refresh_tick.right_aligned())
        .style(app_color_info.cpu_main_block_color)
        .border_set(border::ROUNDED);
    if is_selected {
        main_block = main_block
            .style(app_color_info.cpu_container_selected_color)
            .border_set(border::DOUBLE);
    }

    // Constrain the block to have space at the right and left
    let [_, constraint_block, _] = Layout::horizontal([
        Constraint::Percentage(1),
        Constraint::Percentage(98),
        Constraint::Percentage(1),
    ])
    .areas(size);

    // Split into left (cpu graph) and right (cpu name and usage info)
    let [left, right] =
        Layout::horizontal([Constraint::Percentage(70), Constraint::Percentage(30)])
            .areas(constraint_block);

    // Constrain the block to have space at the top and bottom for cpu graph
    let [_, constraint_inner_left, _] = Layout::vertical([
        Constraint::Percentage(10),
        Constraint::Percentage(80),
        Constraint::Percentage(10),
    ])
    .areas(left);

    // Constrain the block to have space at the top and bottom for cpu name and usage info
    let [_, constraint_inner_right, _] = Layout::vertical([
        Constraint::Percentage(10),
        Constraint::Percentage(80),
        Constraint::Percentage(10),
    ])
    .areas(right);

    // --------------------------------------------------
    // Rendering for CPU usage history graph on the left
    // --------------------------------------------------

    // first get the current selected cpu usage history
    let cpu_usage_history = cpus[cpu_selected_state.selected().unwrap()]
        .usage_history_vec
        .clone();

    // Determine the number of points to display based on zoom level
    let num_points_to_display = graph_show_range.min(cpu_usage_history.len());
    let start_idx = cpu_usage_history
        .len()
        .saturating_sub(num_points_to_display);
    let data_points: Vec<(f64, f64)> = cpu_usage_history[start_idx..]
        .iter()
        .enumerate()
        .map(|(i, &usage)| {
            // X-axis: Usage (0.0 to 100.0)
            // Y-axis: Time (most recent at the bottom)
            // Map the index to a y-value from 0.0 (oldest) to num_points_to_display (newest)
            let x = i as f64;
            let y = usage as f64;
            (x, y)
        })
        .collect();

    // Create the dataset for the chart
    let dataset = Dataset::default()
        .name("")
        .data(&data_points)
        .graph_type(GraphType::Bar)
        .marker(Marker::Braille)
        .style(Style::default().fg(app_color_info.cpu_graph_color));

    let x_axis = Axis::default()
        .style(Style::default().fg(app_color_info.text_color))
        .bounds([0.0, num_points_to_display as f64]);

    // Define the x-axis (CPU Usage) and y-axis (Time)
    let y_axis = Axis::default()
        .title(Line::from("Usage (%)").style(app_color_info.text_color))
        .style(Style::default().fg(app_color_info.text_color))
        .bounds([0.0, 100.0])
        .labels(vec![
            Line::from(vec![Span::styled(
                "0",
                Style::default().fg(app_color_info.text_color),
            )]),
            Line::from(vec![Span::styled(
                "50",
                Style::default().fg(app_color_info.text_color),
            )]),
            Line::from(vec![Span::styled(
                "100",
                Style::default().fg(app_color_info.text_color),
            )]),
        ]);

    // Create the chart widget
    let chart = Chart::new(vec![dataset])
        .x_axis(x_axis)
        .y_axis(y_axis)
        .bg(app_color_info.background_color);

    // --------------------------------------------------
    //    Rendering for CPU info on the right
    // --------------------------------------------------

    // Create the inner_right block [for cpu info]
    let cpu_brand = Line::from(format!(" {} ", cpus[0].brand)).style(app_color_info.text_color);
    let inner_right_block = Block::bordered()
        .title(cpu_brand.left_aligned())
        .style(app_color_info.cpu_info_border_color)
        .border_set(border::ROUNDED);

    // split the cpu name and usage info into two parts
    let [_, cpu_info_inner_container, _] = Layout::horizontal([
        Constraint::Percentage(5),
        Constraint::Percentage(90),
        Constraint::Percentage(5),
    ])
    .areas(constraint_inner_right);

    // Approximate 48% of the container width for each section (name and usage)
    let name_width = cpu_info_inner_container.width as usize / 2;
    let usage_width = cpu_info_inner_container.width as usize / 2;

    // Prepare the combined CPU info list
    let cpu_info_items: Vec<ListItem> = cpus
        .iter()
        .map(|cpu| {
            let mut color = app_color_info.cpu_low_usage_color;
            let name = format!("{}", cpu.id);
            let usage = format!("{:.2}%", cpu.usage);

            // Pad the name to take up 48% of the width
            let padded_name = if name.len() < name_width {
                format!("{:width$}", name, width = name_width)
            } else {
                name.chars().take(name_width).collect::<String>()
            };

            // Pad the usage to take up 48% of the width
            let padded_usage = if usage.len() < usage_width {
                format!("{:width$}", usage, width = usage_width)
            } else {
                usage.chars().take(usage_width).collect::<String>()
            };

            if cpu.usage > 30.0 && cpu.usage < 70.0 {
                color = app_color_info.cpu_medium_usage_color;
            } else if cpu.usage >= 70.0 {
                color = app_color_info.cpu_high_usage_color;
            }

            ListItem::new(Line::from(vec![
                Span::styled(padded_name, Style::default().fg(color)),
                Span::styled(padded_usage, Style::default().fg(color)),
            ]))
        })
        .collect();

    // Create the combined list
    let cpu_info_list = List::new(cpu_info_items)
        .block(inner_right_block)
        .style(Style::default().fg(app_color_info.cpu_selected_color))
        .highlight_style(Style::default().fg(app_color_info.cpu_selected_color))
        .highlight_symbol(">> ");

    // Render the main cpu block container
    frame.render_widget(main_block, size);
    // Render the chart in the left area
    frame.render_widget(chart, constraint_inner_left);
    // Render the combined list with state
    frame.render_stateful_widget(cpu_info_list, cpu_info_inner_container, cpu_selected_state);

    drop(data_points);
    drop(cpu_usage_history);
}
