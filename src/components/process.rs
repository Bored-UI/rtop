use std::collections::HashMap;

use ratatui::{
    layout::Rect,
    style::{Style, Stylize},
    symbols::border,
    text::{Line, Span},
    widgets::Block,
    Frame,
};

use crate::{tui::AppColorInfo, types::ProcessData, utils::get_tick_line_ui};

pub fn draw_process_info(
    tick: u64,
    process_data: &HashMap<String, ProcessData>,
    area: Rect,
    frame: &mut Frame,
    graph_show_range: usize,
    is_selected: bool,
    app_color_info: &AppColorInfo,
    is_full_screen: bool,
) {
    let select_instruction = Line::from(vec![
        Span::styled(" ", Style::default().fg(app_color_info.app_title_color)),
        Span::styled("P", Style::default().fg(app_color_info.key_text_color))
            .bold()
            .underlined(),
        Span::styled(
            "rocess ",
            Style::default().fg(app_color_info.app_title_color),
        ),
    ]);

    let mut main_block = Block::bordered()
        .title(select_instruction.left_aligned())
        .style(app_color_info.process_main_block_color)
        .border_set(border::ROUNDED);

    if is_selected {
        main_block = main_block
            .style(app_color_info.process_container_selected_color)
            .border_set(border::DOUBLE);
    }
    if is_full_screen {
        let refresh_tick = get_tick_line_ui(tick, app_color_info);

        main_block = main_block.title(refresh_tick.right_aligned())
    }

    frame.render_widget(main_block, area);
}
