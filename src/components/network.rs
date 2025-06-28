use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    symbols::{border, Marker},
    text::{Line, Span},
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType},
    Frame,
};

use crate::{
    components::theme::types::AppColorInfo,
    types::NetworkData,
    utils::{get_tick_line_ui, process_to_kib_mib_gib},
};

// width smaller than this will be consider small width for the network container
const SMALL_WIDTH: u16 = 40;
const GRAPH_PERCENTAGE: f64 = 100.0;

pub fn draw_network_info(
    tick: u64,
    network_data: &NetworkData,
    area: Rect,
    frame: &mut Frame,
    graph_show_range: usize,
    is_selected: bool,
    app_color_info: &AppColorInfo,
    is_full_screen: bool,
) {
    let mut network_name = network_data.interface_name.clone();
    if area.width <= SMALL_WIDTH + 5 {
        let extension = if network_name.len() > 16 { ".." } else { "" };
        let new_network_name = network_name.get(..16).unwrap_or(network_name.as_str());
        let new_network_name_with_ext = new_network_name.to_string() + extension;
        network_name = new_network_name_with_ext;
    } else if area.width <= SMALL_WIDTH + 20 {
        let extension = if network_name.len() > 50 { ".." } else { "" };
        let new_network_name = network_name.get(..50).unwrap_or(network_name.as_str());
        let new_network_name_with_ext = new_network_name.to_string() + extension;
        network_name = new_network_name_with_ext;
    }

    let select_instruction = Line::from(vec![
        Span::styled(" ", Style::default().fg(app_color_info.app_title_color)).bold(),
        Span::styled("N", Style::default().fg(app_color_info.key_text_color))
            .bold()
            .underlined(),
        Span::styled(
            "etwork ",
            Style::default().fg(app_color_info.app_title_color).bold(),
        ),
    ]);

    let network_switch_instruction = Line::from(vec![
        Span::styled("  ", Style::default().fg(app_color_info.app_title_color)),
        Span::styled("<", Style::default().fg(app_color_info.key_text_color)).bold(),
        Span::styled(
            format!(" {} ", network_name),
            Style::default().fg(app_color_info.app_title_color).bold(),
        ),
        Span::styled(">", Style::default().fg(app_color_info.key_text_color)).bold(),
        Span::styled("  ", Style::default().fg(app_color_info.app_title_color)),
    ]);

    let mut main_block = Block::bordered()
        .title(select_instruction.left_aligned())
        .title_bottom(network_switch_instruction.centered())
        .style(app_color_info.network_main_block_color)
        .border_set(border::ROUNDED);

    if network_data.ip_network.is_some() {
        main_block = main_block.title(
            Line::from(format!(" {} ", network_data.ip_network.as_ref().unwrap()))
                .fg(app_color_info.network_text_color)
                .bold()
                .centered(),
        )
    }

    if is_selected {
        main_block = main_block
            .style(app_color_info.network_container_selected_color)
            .border_set(border::DOUBLE);
    }
    if is_full_screen {
        let refresh_tick = get_tick_line_ui(tick, app_color_info);

        main_block = main_block.title(refresh_tick.right_aligned())
    }

    frame.render_widget(main_block, area);

    // this will be the layout for the network block for graph and info
    let [_, network_block, _] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Fill(1),
        Constraint::Length(1),
    ])
    .areas(area);

    // padded the layout for the network graph to have some space on the left and right
    let [_, padded_network_block, _] = Layout::horizontal([
        Constraint::Length(2),
        Constraint::Fill(1),
        Constraint::Length(2),
    ])
    .areas(network_block);

    let [network_received_layout, network_transmitted_layout] =
        Layout::vertical([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
            .areas(padded_network_block);

    // render the network received graph
    // ----------------------------------------
    //
    //       FOR NETWORK RECEIVED LAYOUT
    //
    // ----------------------------------------

    let current_network_received_bytes_info = Line::from(format!(
        "Download: {} {}",
        "▼",
        process_to_kib_mib_gib(
            network_data.current_received_vec[network_data.current_received_vec.len() - 1]
        )
    ))
    .style(app_color_info.network_text_color)
    .bold();

    let total_network_received_bytes_info = Line::from(format!(
        "{} Total: {} ",
        "▼",
        process_to_kib_mib_gib(network_data.total_received)
    ))
    .style(app_color_info.network_text_color)
    .bold();

    let [network_received_padded_info_layout, network_received_padded_graph_layout] =
        Layout::vertical([Constraint::Length(1), Constraint::Fill(1)])
            .areas(network_received_layout);

    // network received info
    let network_received_info_block = Block::bordered()
        .title(current_network_received_bytes_info.left_aligned())
        .title(total_network_received_bytes_info.right_aligned())
        .borders(Borders::NONE);

    // network received graph
    let network_received_history = network_data.current_received_vec.clone();
    let num_points_to_display = graph_show_range.min(network_received_history.len());
    let start_idx = network_received_history
        .len()
        .saturating_sub(num_points_to_display);

    let mut current_max_network_received: f64 = 0.0;
    network_received_history[start_idx..]
        .iter()
        .for_each(|usage| {
            current_max_network_received = current_max_network_received.max(*usage);
        });

    let mut network_received_points: Vec<(f64, f64)> = network_received_history[start_idx..]
        .iter()
        .enumerate()
        .map(|(i, &usage)| {
            let x = i as f64;
            let y = if usage > 0.0 {
                (usage / current_max_network_received) * GRAPH_PERCENTAGE as f64
            } else {
                0.0
            };
            (x, y)
        })
        .collect();

    network_received_points = network_received_points
        .iter()
        .map(|(x, y)| {
            (
                graph_show_range as f64 - (network_received_points.len() as f64 - x),
                *y,
            )
        })
        .collect();

    let dataset = Dataset::default()
        .data(&network_received_points)
        .graph_type(GraphType::Bar)
        .marker(Marker::Braille)
        .style(Style::default().fg(app_color_info.network_received_base_graph_color));

    let x_axis = Axis::default().bounds([0.0, graph_show_range as f64]);

    let y_axis = Axis::default().bounds([0.0, GRAPH_PERCENTAGE]);

    let network_received_chart = Chart::new(vec![dataset])
        .x_axis(x_axis)
        .y_axis(y_axis)
        .bg(app_color_info.background_color);

    frame.render_widget(
        network_received_info_block,
        network_received_padded_info_layout,
    );
    frame.render_widget(network_received_chart, network_received_padded_graph_layout);

    // render the network transmitted graph
    // ----------------------------------------
    //
    //       FOR NETWORK TRANSMITTED LAYOUT
    //
    // ----------------------------------------

    let current_network_transmitted_bytes_info = Line::from(format!(
        "Upload: {} {}",
        "▲",
        process_to_kib_mib_gib(
            network_data.current_transmitted_vec[network_data.current_transmitted_vec.len() - 1]
        )
    ))
    .style(app_color_info.network_text_color)
    .bold();

    let total_network_transmitted_bytes_info = Line::from(format!(
        "{} Total: {}",
        "▲",
        process_to_kib_mib_gib(network_data.total_transmitted)
    ))
    .style(app_color_info.network_text_color)
    .bold();

    let [network_transmitted_padded_info_layout, network_transmitted_padded_graph_layout] =
        Layout::vertical([Constraint::Length(1), Constraint::Fill(1)])
            .areas(network_transmitted_layout);

    // network transmitted info
    let network_transmitted_info_block = Block::bordered()
        .title(current_network_transmitted_bytes_info.left_aligned())
        .title(total_network_transmitted_bytes_info.right_aligned())
        .borders(Borders::NONE);

    // network received graph
    let network_transmitted_history = network_data.current_transmitted_vec.clone();
    let num_points_to_display = graph_show_range.min(network_transmitted_history.len());
    let start_idx = network_transmitted_history
        .len()
        .saturating_sub(num_points_to_display);

    let mut current_max_network_transmitted: f64 = 0.0;
    network_transmitted_history[start_idx..]
        .iter()
        .for_each(|usage| {
            current_max_network_transmitted = current_max_network_transmitted.max(*usage);
        });

    let mut network_transmitted_points: Vec<(f64, f64)> = network_transmitted_history[start_idx..]
        .iter()
        .enumerate()
        .map(|(i, &usage)| {
            let x = i as f64;
            let y = if usage > 0.0 {
                (usage / current_max_network_transmitted) * GRAPH_PERCENTAGE as f64
            } else {
                0.0
            };
            (x, y)
        })
        .collect();

    network_transmitted_points = network_transmitted_points
        .iter()
        .map(|(x, y)| {
            (
                graph_show_range as f64 - (network_transmitted_points.len() as f64 - x),
                *y,
            )
        })
        .collect();

    let dataset = Dataset::default()
        .data(&network_transmitted_points)
        .graph_type(GraphType::Bar)
        .marker(Marker::Braille)
        .style(Style::default().fg(app_color_info.network_transmitted_base_graph_color));

    let x_axis = Axis::default().bounds([0.0, graph_show_range as f64]);

    let y_axis = Axis::default().bounds([0.0, GRAPH_PERCENTAGE]);

    let network_transmitted_chart = Chart::new(vec![dataset])
        .x_axis(x_axis)
        .y_axis(y_axis)
        .bg(app_color_info.background_color);

    frame.render_widget(
        network_transmitted_info_block,
        network_transmitted_padded_info_layout,
    );
    frame.render_widget(
        network_transmitted_chart,
        network_transmitted_padded_graph_layout,
    );
}
