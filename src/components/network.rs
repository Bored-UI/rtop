use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    symbols::border,
    text::{Line, Span},
    widgets::Block,
    Frame,
};

use crate::{tui::AppColorInfo, types::NetworkData, utils::get_tick_line_ui};

// width smaller than this will be consider small width for the memory container
const SMALL_WIDTH: u16 = 40;
const MEDIUM_HEIGHT: u16 = 16;
const LARGE_HEIGHT: u16 = 21;
// const MEMORY_GRAPH_HEIGHT_PRCENTAGE: u16 = 70;

// this was to indicate that the memory graph y axis will be either shown as 25% or 100% (based on the widget size)
const SMALL_WIDGET_PERCENTAGE: f64 = 25.0;
const BIG_WIDGET_PERCENTAGE: f64 = 100.0;

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
    let current_graph_percentage = if is_full_screen {
        BIG_WIDGET_PERCENTAGE
    } else {
        SMALL_WIDGET_PERCENTAGE
    };

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
        Span::styled(" ", Style::default().fg(app_color_info.app_title_color)),
        Span::styled("N", Style::default().fg(app_color_info.key_text_color))
            .bold()
            .underlined(),
        Span::styled(
            "etwork ",
            Style::default().fg(app_color_info.app_title_color),
        ),
    ]);

    let network_switch_instruction = Line::from(vec![
        Span::styled("  ", Style::default().fg(app_color_info.app_title_color)),
        Span::styled("<", Style::default().fg(app_color_info.key_text_color)).bold(),
        Span::styled(
            format!(" {} ", network_name),
            Style::default().fg(app_color_info.app_title_color),
        ),
        Span::styled(">", Style::default().fg(app_color_info.key_text_color)).bold(),
        Span::styled("  ", Style::default().fg(app_color_info.app_title_color)),
    ]);

    let mut main_block = Block::bordered()
        .title(select_instruction.left_aligned())
        .title_bottom(network_switch_instruction.centered())
        .style(app_color_info.network_main_block_color)
        .border_set(border::ROUNDED);
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

    // this will be the layout for the memory usage graph
    let [_, network_block, _] = Layout::vertical([
        Constraint::Percentage(2),
        Constraint::Percentage(96),
        Constraint::Percentage(2),
    ])
    .areas(area);

    // padded the layout for the network graph to have some space on the left and right
    let [_, padded_network_block, _] = Layout::horizontal([
        Constraint::Percentage(3),
        Constraint::Percentage(94),
        Constraint::Percentage(3),
    ])
    .areas(network_block);
}
