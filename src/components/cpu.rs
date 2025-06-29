use chrono::Local;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    symbols::{border, Marker},
    text::{Line, Span},
    widgets::{Axis, Block, Chart, Dataset, GraphType, List, ListItem, ListState},
    Frame,
};

use crate::{
    types::{AppColorInfo, CpuData},
    utils::get_tick_line_ui,
};

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
        Span::styled(
            format!(" {} ", local_time.format("%H:%M:%S")),
            Style::default().fg(app_color_info.app_title_color),
        )
        .bold(),
    );
    let refresh_tick = get_tick_line_ui(tick, app_color_info);
    let select_instruction = Line::from(vec![
        Span::styled(" ", Style::default().fg(app_color_info.app_title_color)),
        Span::styled("C", Style::default().fg(app_color_info.key_text_color))
            .bold()
            .underlined(),
        Span::styled("pu ", Style::default().fg(app_color_info.app_title_color)).bold(),
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
    let [_, cpu_block, _] = Layout::horizontal([
        Constraint::Length(2),
        Constraint::Fill(1),
        Constraint::Length(2),
    ])
    .areas(size);

    // Split into cpu_graph_layout and cpu_info_layout (cpu name and usage info)
    let [cpu_graph_layout, cpu_info_layout] =
        Layout::horizontal([Constraint::Fill(7), Constraint::Fill(3)]).areas(cpu_block);

    // Constrain the block to have space at the top and bottom for cpu graph
    let [_, constraint_inner_cpu_graph_layout, _] = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Fill(8),
        Constraint::Fill(1),
    ])
    .areas(cpu_graph_layout);

    // Constrain the block to have space at the top and bottom for cpu name and usage info
    let [_, constraint_inner_cpu_info_layout, _] = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Fill(8),
        Constraint::Fill(1),
    ])
    .areas(cpu_info_layout);

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
    let mut data_points: Vec<(f64, f64)> = cpu_usage_history[start_idx..]
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

    data_points = data_points
        .iter()
        .map(|(x, y)| (graph_show_range as f64 - (data_points.len() as f64 - x), *y))
        .collect();

    // Create the dataset for the chart
    let dataset = Dataset::default()
        .name("")
        .data(&data_points)
        .graph_type(GraphType::Bar)
        .marker(Marker::Braille)
        .style(Style::default().fg(app_color_info.cpu_base_graph_color));

    let x_axis = Axis::default().bounds([0.0, graph_show_range as f64]);

    // Define the x-axis (CPU Usage) and y-axis (Time)
    let y_axis = Axis::default().bounds([0.0, 100.0]);

    // Create the chart widget
    let chart = Chart::new(vec![dataset])
        .x_axis(x_axis)
        .y_axis(y_axis)
        .bg(app_color_info.background_color);

    // --------------------------------------------------
    //    Rendering for CPU info on the right
    // --------------------------------------------------

    // Create the inner_right block [for cpu info]
    let cpu_brand = Line::from(format!(" {} ", cpus[0].brand))
        .style(app_color_info.app_title_color)
        .bold();
    let inner_right_block = Block::bordered()
        .title(cpu_brand.left_aligned())
        .style(app_color_info.cpu_info_block_color)
        .border_set(border::ROUNDED);

    // split the cpu name and usage info into two parts
    let [_, cpu_info_inner_container, _] = Layout::horizontal([
        Constraint::Length(1),
        Constraint::Fill(1),
        Constraint::Length(1),
    ])
    .areas(constraint_inner_cpu_info_layout);

    // Approximate 48% of the container width for each section (name and usage)
    let name_width = cpu_info_inner_container.width as usize / 2;
    let usage_width = cpu_info_inner_container.width as usize / 2;

    // Prepare the combined CPU info list
    let cpu_info_items: Vec<ListItem> = cpus
        .iter()
        .map(|cpu| {
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

            ListItem::new(Line::from(vec![
                Span::styled(
                    padded_name,
                    Style::default().fg(app_color_info.base_app_text_color),
                ),
                Span::styled(
                    padded_usage,
                    Style::default().fg(app_color_info.cpu_text_color),
                ),
            ]))
        })
        .collect();

    // Create the combined list
    let cpu_info_list = List::new(cpu_info_items)
        .block(inner_right_block)
        .style(Style::default().fg(app_color_info.cpu_selected_color))
        .highlight_style(
            Style::default()
                .fg(app_color_info.cpu_selected_color)
                .bold(),
        )
        .highlight_symbol(">> ");

    // Render the main cpu block container
    frame.render_widget(main_block, size);
    // Render the chart in the left area
    frame.render_widget(chart, constraint_inner_cpu_graph_layout);
    // Render the combined list with state
    frame.render_stateful_widget(cpu_info_list, cpu_info_inner_container, cpu_selected_state);

    drop(data_points);
    drop(cpu_usage_history);
}
