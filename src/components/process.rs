use std::collections::HashMap;

use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style, Stylize},
    symbols::{border, Marker},
    text::{Line, Span},
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType, List, ListItem, ListState},
    Frame,
};

use crate::{
    types::{AppColorInfo, ProcessData, ProcessSortType},
    utils::{
        break_line_into_vectors_of_string, format_seconds, get_tick_line_ui,
        process_to_kib_mib_gib, round_to_2_decimal, sort_process,
    },
};

const GRAPH_PERCENTAGE: f64 = 100.0;

const MEDIUM_WIDTH: u16 = 60;
const LARGE_WIDTH: u16 = 80;
const X_LARGE_WIDTH: u16 = 100;
const XX_LARGE_WIDTH: u16 = 120;

// following is the process detail container required space percentage in different window height and also the height definetion
const MEDIUM_HEIGHT: u16 = 15;
const LARGE_HEIGHT: u16 = 20;
const X_LARGE_HEIGHT: u16 = 30;
const XX_LARGE_HEIGHT: u16 = 40;

const MEDIUM_HEIGHT_FILL: u16 = 5;
const LARGE_HEIGHT_FILL: u16 = 4;
const X_LARGE_HEIGHT_FILL: u16 = 3;
const XX_LARGE_HEIGHT_FILL: u16 = 3;

pub fn draw_process_info(
    tick: u64,
    process_data: &HashMap<String, ProcessData>,
    process_current_list: &mut Vec<ProcessData>,
    process_selectable_entries: &mut usize,
    process_selected_state: &mut ListState,
    process_sort_type: &ProcessSortType,
    process_sort_is_reversed: bool,
    process_filter: String,
    process_show_detail: bool,
    current_showing_process_detail: &Option<HashMap<String, ProcessData>>,
    total_memory: f64,
    is_filtering: bool, // to indicate if the app enter typing state for process filtering
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
            Style::default().fg(app_color_info.app_title_color).bold(),
        ),
    ]);

    let process_sort_is_reversed_intruction = Line::from(vec![
        Span::styled(" ", Style::default().fg(app_color_info.app_title_color)),
        Span::styled("R", Style::default().fg(app_color_info.key_text_color))
            .bold()
            .underlined(),
        Span::styled(
            "everse ",
            Style::default().fg(app_color_info.app_title_color).bold(),
        ),
    ]);

    // for selecting based sorting type, example based on thread count, memory etc
    let process_sort_select_instruction = Line::from(vec![
        Span::styled("　< ", Style::default().fg(app_color_info.key_text_color)).bold(),
        Span::styled(
            ProcessSortType::get_sort_string_name(process_sort_type),
            Style::default().fg(app_color_info.app_title_color).bold(),
        ),
        Span::styled(" >　", Style::default().fg(app_color_info.key_text_color)).bold(),
    ]);

    let mut process_filter_without_underscore_extension: String = process_filter
        .chars()
        .take(process_filter.len() - 1)
        .collect();

    // for process filtering input width takeup space
    process_filter_without_underscore_extension =
        if area.width > MEDIUM_WIDTH && area.width <= LARGE_WIDTH {
            if process_filter_without_underscore_extension.len() > 20 {
                process_filter_without_underscore_extension
                    .chars()
                    .skip(process_filter_without_underscore_extension.len() - 20)
                    .collect()
            } else {
                process_filter_without_underscore_extension
            }
        } else if area.width > LARGE_WIDTH && area.width <= X_LARGE_WIDTH {
            if process_filter_without_underscore_extension.len() > 30 {
                process_filter_without_underscore_extension
                    .chars()
                    .skip(process_filter_without_underscore_extension.len() - 30)
                    .collect()
            } else {
                process_filter_without_underscore_extension
            }
        } else if area.width > X_LARGE_WIDTH && area.width <= XX_LARGE_WIDTH {
            if process_filter_without_underscore_extension.len() > 45 {
                process_filter_without_underscore_extension
                    .chars()
                    .skip(process_filter_without_underscore_extension.len() - 45)
                    .collect()
            } else {
                process_filter_without_underscore_extension
            }
        } else if area.width > XX_LARGE_WIDTH {
            if process_filter_without_underscore_extension.len() > 60 {
                process_filter_without_underscore_extension
                    .chars()
                    .skip(process_filter_without_underscore_extension.len() - 60)
                    .collect()
            } else {
                process_filter_without_underscore_extension
            }
        } else {
            if process_filter_without_underscore_extension.len() > 10 {
                process_filter_without_underscore_extension
                    .chars()
                    .skip(process_filter_without_underscore_extension.len() - 10)
                    .collect()
            } else {
                process_filter_without_underscore_extension
            }
        };

    let process_filter_instruction = if is_filtering {
        Line::from(vec![
            Span::styled(" ", Style::default().fg(app_color_info.app_title_color)),
            Span::styled("F", Style::default().fg(app_color_info.key_text_color))
                .bold()
                .underlined(),
            Span::styled(
                format!(" {}_ ", process_filter_without_underscore_extension),
                Style::default().fg(app_color_info.app_title_color).bold(),
            ),
            Span::styled("↵ ", Style::default().fg(app_color_info.key_text_color)).bold(),
        ])
    } else {
        if process_filter.is_empty() || process_filter == "_".to_string() {
            Line::from(vec![
                Span::styled(" ", Style::default().fg(app_color_info.app_title_color)),
                Span::styled("F", Style::default().fg(app_color_info.key_text_color))
                    .bold()
                    .underlined(),
                Span::styled(
                    "ilter ",
                    Style::default().fg(app_color_info.app_title_color).bold(),
                ),
            ])
        } else {
            Line::from(vec![
                Span::styled(" ", Style::default().fg(app_color_info.app_title_color)),
                Span::styled("F", Style::default().fg(app_color_info.key_text_color))
                    .bold()
                    .underlined(),
                Span::styled(
                    format!(" {} ", process_filter_without_underscore_extension),
                    Style::default().fg(app_color_info.app_title_color).bold(),
                ),
                Span::styled("← ", Style::default().fg(app_color_info.key_text_color)).bold(),
            ])
        }
    };

    // to indicate that user is currently navigating in the process list items or not
    let is_process_selected = if let Some(_) = process_selected_state.selected() {
        true
    } else {
        false
    };

    // to check if user have already reached the end of the list
    let is_selected_process_eol = if let Some(selected) = process_selected_state.selected() {
        let is_eol = if selected + 1 == *process_selectable_entries {
            true
        } else {
            false
        };

        is_eol
    } else {
        false
    };

    let navigating_up_arrow = if is_process_selected {
        Span::styled("↑ ", Style::default().fg(app_color_info.key_text_color)).bold()
    } else {
        // dim out the up arrow key as user didn't select any process thus navigating up the list is impossible
        Span::styled("↑ ", Style::default().fg(app_color_info.key_text_color))
            .bold()
            .add_modifier(Modifier::DIM)
    };

    let navigating_down_arrow = if is_selected_process_eol {
        // dim out the down arrow key as user has reached the end of list thus navigating down the list is impossible
        Span::styled(" ↓", Style::default().fg(app_color_info.key_text_color))
            .bold()
            .add_modifier(Modifier::DIM)
    } else {
        Span::styled(" ↓", Style::default().fg(app_color_info.key_text_color)).bold()
    };

    let process_list_selection_instruction = Line::from(vec![
        navigating_up_arrow,
        Span::styled(
            "select",
            Style::default().fg(app_color_info.app_title_color),
        )
        .bold(),
        navigating_down_arrow,
    ]);

    let able_show_info = if is_process_selected {
        Line::from(vec![
            Span::styled(
                " info ",
                Style::default().fg(app_color_info.app_title_color),
            )
            .bold(),
            Span::styled("↵ ", Style::default().fg(app_color_info.key_text_color)).bold(),
        ])
    } else {
        // dim out the up info key as user didn't select any process thus showing detail info is not possible
        Line::from(vec![
            Span::styled(
                " info ",
                Style::default().fg(app_color_info.app_title_color),
            )
            .bold()
            .add_modifier(Modifier::DIM),
            Span::styled("↵ ", Style::default().fg(app_color_info.key_text_color))
                .bold()
                .add_modifier(Modifier::DIM),
        ])
    };

    let mut main_block = Block::bordered()
        .title(select_instruction.left_aligned())
        .title(process_filter_instruction.left_aligned())
        .title(process_sort_is_reversed_intruction.right_aligned())
        .title(process_sort_select_instruction.right_aligned())
        .title_bottom(process_list_selection_instruction.left_aligned())
        .title_bottom(able_show_info.left_aligned())
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

    // padded the inner container
    let [_, padded_vertical_inner, _] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Fill(1),
        Constraint::Length(1),
    ])
    .areas(area);

    let [_, process_block, _] = Layout::horizontal([
        Constraint::Length(2),
        Constraint::Fill(1),
        Constraint::Length(2),
    ])
    .areas(padded_vertical_inner);

    let [mut title_layout, mut process_list_layout] =
        Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).areas(process_block);

    // layout for process detail
    if process_show_detail {
        // to determine how much space should the process detail layout takes
        let percentage_of_process_detail_container_space =
            if area.height >= MEDIUM_HEIGHT && area.height < LARGE_HEIGHT {
                MEDIUM_HEIGHT_FILL
            } else if area.height >= LARGE_HEIGHT && area.height < X_LARGE_HEIGHT {
                LARGE_HEIGHT_FILL
            } else if area.height >= X_LARGE_HEIGHT && area.height < XX_LARGE_HEIGHT {
                X_LARGE_HEIGHT_FILL
            } else {
                XX_LARGE_HEIGHT_FILL
            };

        let [process_detail_layout, new_title_layout, new_process_list_layout] =
            Layout::vertical([
                Constraint::Fill(percentage_of_process_detail_container_space),
                Constraint::Length(1),
                Constraint::Fill(10 - percentage_of_process_detail_container_space),
            ])
            .areas(process_block);

        title_layout = new_title_layout;
        process_list_layout = new_process_list_layout;

        match current_showing_process_detail.as_ref() {
            Some(hashmap) => {
                if let Some((_, value)) = hashmap.iter().next() {
                    let process_detail = value;

                    let [process_detail_graph_layout, process_detail_info_layout] =
                        Layout::horizontal([Constraint::Fill(3), Constraint::Fill(7)])
                            .areas(process_detail_layout);
                    // ------------------------------------------------
                    // block for the cpu usage graph for the process
                    // ------------------------------------------------

                    // pid of the process detail
                    let pid = Line::from(vec![Span::styled(
                        process_detail.pid.to_string(),
                        Style::default().fg(app_color_info.app_title_color),
                    )
                    .bold()]);

                    // name of the process detail
                    let name = Line::from(vec![Span::styled(
                        process_detail.name.to_string(),
                        Style::default().fg(app_color_info.app_title_color),
                    )
                    .bold()]);
                    let process_detail_graph_block = Block::bordered()
                        .borders(Borders::RIGHT)
                        .title(pid.left_aligned())
                        .title(name.left_aligned())
                        .style(app_color_info.process_main_block_color);

                    // ------------------------------------------------
                    // block for process detail info
                    // ------------------------------------------------
                    let is_user_navigating_process_list =
                        if let Some(_) = process_selected_state.selected() {
                            true
                        } else {
                            false
                        };

                    let is_process_killed_or_terminated =
                        if let Some(hashmap) = current_showing_process_detail.as_ref() {
                            if let Some((_, value)) = hashmap.iter().next() {
                                if value.status == "killed".to_string() {
                                    true
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        } else {
                            false
                        };

                    // if user is currently navigating in the process list or the process is killed/terminated,
                    // dim the termination trigger for process detail container to act as like it was disabled
                    let termination_instruction =
                        if is_user_navigating_process_list || is_process_killed_or_terminated {
                            Line::from(vec![
                                Span::styled(
                                    "T".to_string(),
                                    Style::default().fg(app_color_info.key_text_color),
                                )
                                .bold()
                                .underlined()
                                .add_modifier(Modifier::DIM),
                                Span::styled(
                                    "erminate".to_string(),
                                    Style::default().fg(app_color_info.app_title_color),
                                )
                                .bold()
                                .add_modifier(Modifier::DIM),
                            ])
                        } else {
                            Line::from(vec![
                                Span::styled(
                                    "T".to_string(),
                                    Style::default().fg(app_color_info.key_text_color),
                                )
                                .bold()
                                .underlined(),
                                Span::styled(
                                    "erminate".to_string(),
                                    Style::default().fg(app_color_info.app_title_color),
                                )
                                .bold(),
                            ])
                        };

                    // if user is currently navigating in the process list or the process is killed/terminated,
                    // dim the kill trigger for process detail container to act as like it was disabled
                    let kill_instruction =
                        if is_user_navigating_process_list || is_process_killed_or_terminated {
                            Line::from(vec![
                                Span::styled(
                                    "K".to_string(),
                                    Style::default().fg(app_color_info.key_text_color),
                                )
                                .bold()
                                .underlined()
                                .add_modifier(Modifier::DIM),
                                Span::styled(
                                    "ill".to_string(),
                                    Style::default().fg(app_color_info.app_title_color),
                                )
                                .bold()
                                .add_modifier(Modifier::DIM),
                            ])
                        } else {
                            Line::from(vec![
                                Span::styled(
                                    "K".to_string(),
                                    Style::default().fg(app_color_info.key_text_color),
                                )
                                .bold()
                                .underlined(),
                                Span::styled(
                                    "ill".to_string(),
                                    Style::default().fg(app_color_info.app_title_color),
                                )
                                .bold(),
                            ])
                        };

                    // if user is currently navigating in the process list or the process is killed/terminated,
                    // dim the signal trigger for process detail container to act as like it was disabled
                    let signal_instruction =
                        if is_user_navigating_process_list || is_process_killed_or_terminated {
                            Line::from(vec![
                                Span::styled(
                                    "S".to_string(),
                                    Style::default().fg(app_color_info.key_text_color),
                                )
                                .bold()
                                .underlined()
                                .add_modifier(Modifier::DIM),
                                Span::styled(
                                    "ignal".to_string(),
                                    Style::default().fg(app_color_info.app_title_color),
                                )
                                .bold()
                                .add_modifier(Modifier::DIM),
                            ])
                        } else {
                            Line::from(vec![
                                Span::styled(
                                    "S".to_string(),
                                    Style::default().fg(app_color_info.key_text_color),
                                )
                                .bold()
                                .underlined(),
                                Span::styled(
                                    "ignal".to_string(),
                                    Style::default().fg(app_color_info.app_title_color),
                                )
                                .bold(),
                            ])
                        };

                    // if user is currently navigating in the process list, dim the hide trigger for process detail container to act as like it was disabled
                    let hide_instruction = if is_user_navigating_process_list {
                        Line::from(vec![
                            Span::styled(
                                "Hide ".to_string(),
                                Style::default().fg(app_color_info.app_title_color),
                            )
                            .bold()
                            .add_modifier(Modifier::DIM),
                            Span::styled("↵", Style::default().fg(app_color_info.key_text_color))
                                .bold()
                                .add_modifier(Modifier::DIM),
                        ])
                    } else {
                        Line::from(vec![
                            Span::styled(
                                "Hide ".to_string(),
                                Style::default().fg(app_color_info.app_title_color),
                            )
                            .bold(),
                            Span::styled("↵", Style::default().fg(app_color_info.key_text_color))
                                .bold(),
                        ])
                    };

                    let process_detail_info_block = if area.width < MEDIUM_WIDTH {
                        Block::bordered()
                            .borders(Borders::NONE)
                            .title(termination_instruction.left_aligned())
                            .title(signal_instruction.left_aligned())
                            .title(hide_instruction.right_aligned())
                    } else {
                        Block::bordered()
                            .borders(Borders::NONE)
                            .title(termination_instruction.left_aligned())
                            .title(kill_instruction.left_aligned())
                            .title(signal_instruction.left_aligned())
                            .title(hide_instruction.right_aligned())
                    };

                    // render both block
                    frame.render_widget(process_detail_graph_block, process_detail_graph_layout);
                    frame.render_widget(process_detail_info_block, process_detail_info_layout);

                    // ------------------------------------------------------------
                    // Render process CPU usage history graph on the left
                    // ------------------------------------------------------------
                    let [_, padded_detail_graph_horizontal, _] = Layout::horizontal([
                        Constraint::Length(1),
                        Constraint::Fill(1),
                        Constraint::Length(1),
                    ])
                    .areas(process_detail_graph_layout);

                    let [_, padded_detail_graph_layout, detail_graph_naming_layout] =
                        Layout::vertical([
                            Constraint::Length(1),
                            Constraint::Fill(1),
                            Constraint::Length(1),
                        ])
                        .areas(padded_detail_graph_horizontal);

                    let [_, padded_detail_graph_naming_layout, _] = Layout::horizontal([
                        Constraint::Fill(1),
                        Constraint::Length(3),
                        Constraint::Fill(1),
                    ])
                    .areas(detail_graph_naming_layout);

                    // get the process cpu usage history
                    let process_cpu_usage_history = process_detail.cpu_usage.clone();

                    // Determine the number of points to display based on zoom level
                    let num_points_to_display =
                        graph_show_range.min(process_cpu_usage_history.len());
                    let start_idx = process_cpu_usage_history
                        .len()
                        .saturating_sub(num_points_to_display);
                    let mut process_cpu_usage_points: Vec<(f64, f64)> = process_cpu_usage_history
                        [start_idx..]
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

                    process_cpu_usage_points = process_cpu_usage_points
                        .iter()
                        .map(|(x, y)| {
                            (
                                graph_show_range as f64
                                    - (process_cpu_usage_points.len() as f64 - x),
                                *y,
                            )
                        })
                        .collect();

                    // Create the dataset for the chart
                    let dataset = Dataset::default()
                        .name("")
                        .data(&process_cpu_usage_points)
                        .graph_type(GraphType::Bar)
                        .marker(Marker::Braille)
                        .style(Style::default().fg(app_color_info.cpu_base_graph_color));

                    let x_axis = Axis::default().bounds([0.0, graph_show_range as f64]);

                    // Define the x-axis (CPU Usage) and y-axis (Time)
                    let y_axis = Axis::default().bounds([0.0, 100.0]);

                    // Create the chart widget
                    let process_cpu_usage_chart = Chart::new(vec![dataset])
                        .x_axis(x_axis)
                        .y_axis(y_axis)
                        .bg(app_color_info.background_color);

                    // --------------------------------------------------------------------------------
                    // Render process CPU usage history graph naming at the bottom of the graph
                    // --------------------------------------------------------------------------------
                    let process_cpu_usage_graph_naming = Line::from(vec![Span::styled(
                        "CPU".to_string(),
                        Style::default().fg(app_color_info.app_title_color),
                    )
                    .bold()]);

                    frame.render_widget(process_cpu_usage_chart, padded_detail_graph_layout);
                    frame.render_widget(
                        process_cpu_usage_graph_naming,
                        padded_detail_graph_naming_layout,
                    );

                    // ------------------------------------------------------------
                    // Render process detail info on the right
                    // ------------------------------------------------------------
                    let [_, padded_detail_info_layout] =
                        Layout::vertical([Constraint::Length(1), Constraint::Fill(1)])
                            .areas(process_detail_info_layout);

                    let [process_info_layout, process_memory_usage_layout, process_cmd_layout] =
                        Layout::vertical(vec![
                            Constraint::Length(3),
                            Constraint::Fill(1),
                            Constraint::Length(3),
                        ])
                        .areas(padded_detail_info_layout);

                    // ------------------------------------------------------------
                    // Various process metrics on the top
                    // ------------------------------------------------------------
                    let [process_info_title_layout, process_info_detail_layout, extra_detail_layout] =
                        Layout::vertical(vec![
                            Constraint::Length(1),
                            Constraint::Length(1),
                            Constraint::Length(1),
                        ])
                        .areas(process_info_layout);

                    let mut status_width = 0;
                    let mut elapsed_width = 0;
                    let mut io_read_width = 0;
                    let mut io_write_width = 0;
                    let mut parent_width = 0;
                    let mut user_width = 0;
                    let mut thread_width = 0;

                    if area.width <= MEDIUM_WIDTH {
                        let [new_status, new_elapsed, new_thread] = Layout::horizontal(vec![
                            Constraint::Fill(1),
                            Constraint::Fill(1),
                            Constraint::Fill(1),
                        ])
                        .areas(process_info_title_layout);
                        status_width = new_status.width as usize;
                        elapsed_width = new_elapsed.width as usize;
                        thread_width = new_thread.width as usize;
                    } else if area.width > MEDIUM_WIDTH && area.width <= LARGE_WIDTH {
                        let [new_status, new_elapsed, new_io_read, new_thread] =
                            Layout::horizontal(vec![
                                Constraint::Fill(2),
                                Constraint::Fill(2),
                                Constraint::Fill(3),
                                Constraint::Fill(2),
                            ])
                            .areas(process_info_title_layout);
                        status_width = new_status.width as usize;
                        elapsed_width = new_elapsed.width as usize;
                        io_read_width = new_io_read.width as usize;
                        thread_width = new_thread.width as usize;
                    } else if area.width > LARGE_WIDTH && area.width <= X_LARGE_WIDTH {
                        let [new_status, new_elapsed, new_io_read, new_io_write, new_thread] =
                            Layout::horizontal(vec![
                                Constraint::Fill(2),
                                Constraint::Fill(2),
                                Constraint::Fill(3),
                                Constraint::Fill(3),
                                Constraint::Fill(2),
                            ])
                            .areas(process_info_title_layout);
                        status_width = new_status.width as usize;
                        elapsed_width = new_elapsed.width as usize;
                        io_read_width = new_io_read.width as usize;
                        io_write_width = new_io_write.width as usize;
                        thread_width = new_thread.width as usize;
                    } else if area.width > X_LARGE_WIDTH && area.width <= XX_LARGE_WIDTH {
                        let [new_status, new_elapsed, new_io_read, new_io_write, new_parent, new_thread] =
                            Layout::horizontal(vec![
                                Constraint::Fill(2),
                                Constraint::Fill(2),
                                Constraint::Fill(3),
                                Constraint::Fill(3),
                                Constraint::Fill(2),
                                Constraint::Fill(2),
                            ])
                            .areas(process_info_title_layout);
                        status_width = new_status.width as usize;
                        elapsed_width = new_elapsed.width as usize;
                        io_read_width = new_io_read.width as usize;
                        io_write_width = new_io_write.width as usize;
                        parent_width = new_parent.width as usize;
                        thread_width = new_thread.width as usize;
                    } else if area.width > XX_LARGE_WIDTH {
                        let [new_status, new_elapsed, new_io_read, new_io_write, new_parent, new_user, new_thread] =
                            Layout::horizontal(vec![
                                Constraint::Fill(2),
                                Constraint::Fill(2),
                                Constraint::Fill(3),
                                Constraint::Fill(3),
                                Constraint::Fill(2),
                                Constraint::Fill(2),
                                Constraint::Fill(2),
                            ])
                            .areas(process_info_title_layout);

                        status_width = new_status.width as usize;
                        elapsed_width = new_elapsed.width as usize;
                        io_read_width = new_io_read.width as usize;
                        io_write_width = new_io_write.width as usize;
                        parent_width = new_parent.width as usize;
                        user_width = new_user.width as usize;
                        thread_width = new_thread.width as usize;
                    }

                    let status_title = String::from("Status:");
                    let elapsed_title = String::from("Elapsed:");
                    let io_read_title = String::from("IO/R (C/T):");
                    let io_write_title = String::from("IO/W (C/T):");
                    let user_title = String::from("User:");
                    let parent_title = String::from("Parent:");
                    let thread_title = String::from("Threads:");

                    let padded_status_title = if status_title.len() < status_width {
                        format!("{:^width$}", status_title, width = status_width)
                    } else {
                        status_title.chars().take(status_width).collect::<String>()
                    };

                    let padded_elapsed_title = if elapsed_title.len() < elapsed_width {
                        format!("{:^width$}", elapsed_title, width = elapsed_width)
                    } else {
                        elapsed_title
                            .chars()
                            .take(elapsed_width)
                            .collect::<String>()
                    };

                    let padded_io_read_title = if io_read_title.len() < io_read_width {
                        format!("{:^width$}", io_read_title, width = io_read_width)
                    } else {
                        io_read_title
                            .chars()
                            .take(io_read_width)
                            .collect::<String>()
                    };

                    let padded_io_write_title = if io_write_title.len() < io_write_width {
                        format!("{:^width$}", io_write_title, width = io_write_width)
                    } else {
                        io_write_title
                            .chars()
                            .take(io_write_width)
                            .collect::<String>()
                    };

                    let padded_user_title = if user_title.len() < user_width {
                        format!("{:^width$}", user_title, width = user_width)
                    } else {
                        user_title.chars().take(user_width).collect::<String>()
                    };

                    let padded_parent_title = if parent_title.len() < parent_width {
                        format!("{:^width$}", parent_title, width = parent_width)
                    } else {
                        parent_title.chars().take(parent_width).collect::<String>()
                    };

                    let padded_thread_title = if thread_title.len() < thread_width {
                        format!("{:^width$}", thread_title, width = thread_width)
                    } else {
                        thread_title.chars().take(thread_width).collect::<String>()
                    };

                    let process_info_title = Line::from(vec![
                        Span::styled(
                            padded_status_title,
                            Style::default()
                                .fg(app_color_info.process_title_color)
                                .bold(),
                        ),
                        Span::styled(
                            padded_elapsed_title,
                            Style::default()
                                .fg(app_color_info.process_title_color)
                                .bold(),
                        ),
                        Span::styled(
                            padded_io_read_title,
                            Style::default()
                                .fg(app_color_info.process_title_color)
                                .bold(),
                        ),
                        Span::styled(
                            padded_io_write_title,
                            Style::default()
                                .fg(app_color_info.process_title_color)
                                .bold(),
                        ),
                        Span::styled(
                            padded_user_title,
                            Style::default()
                                .fg(app_color_info.process_title_color)
                                .bold(),
                        ),
                        Span::styled(
                            padded_parent_title,
                            Style::default()
                                .fg(app_color_info.process_title_color)
                                .bold(),
                        ),
                        Span::styled(
                            padded_thread_title,
                            Style::default()
                                .fg(app_color_info.process_title_color)
                                .bold(),
                        ),
                    ]);

                    frame.render_widget(process_info_title, process_info_title_layout);

                    let status_detail = value.status.clone();
                    let elapsed_detail = format_seconds(value.elapsed);
                    let current_io_read_detail = format!(
                        "{} /",
                        process_to_kib_mib_gib(value.current_read_disk_usage as f64)
                    );
                    let total_io_read_detail = format!(
                        "{}",
                        process_to_kib_mib_gib(value.total_read_disk_usage as f64)
                    ); // this will be render at the extra detail row
                    let current_io_write_detail = format!(
                        "{} /",
                        process_to_kib_mib_gib(value.current_write_disk_usage as f64)
                    );
                    let total_io_write_detail = format!(
                        "{}",
                        process_to_kib_mib_gib(value.total_write_disk_usage as f64)
                    ); // this will be render at the extra detail row
                    let user_detail = value.user.clone();
                    let parent_detail = match process_data.get(&value.parent) {
                        Some(p_d) => p_d.name.clone(),
                        None => "-".to_string(),
                    };
                    let thread_detail = value.thread_count.to_string();

                    let padded_status_detail = if status_detail.len() < status_width {
                        format!("{:^width$}", status_detail, width = status_width)
                    } else {
                        status_detail.chars().take(status_width).collect::<String>()
                    };

                    let padded_elapsed_detail = if elapsed_detail.len() < elapsed_width {
                        format!("{:^width$}", elapsed_detail, width = elapsed_width)
                    } else {
                        elapsed_detail
                            .chars()
                            .take(elapsed_width)
                            .collect::<String>()
                    };

                    let padded_current_io_read_detail =
                        if current_io_read_detail.len() < io_read_width {
                            format!("{:^width$}", current_io_read_detail, width = io_read_width)
                        } else {
                            current_io_read_detail
                                .chars()
                                .take(io_read_width)
                                .collect::<String>()
                        };

                    let padded_current_io_write_detail =
                        if current_io_write_detail.len() < io_write_width {
                            format!(
                                "{:^width$}",
                                current_io_write_detail,
                                width = io_write_width
                            )
                        } else {
                            current_io_write_detail
                                .chars()
                                .take(io_write_width)
                                .collect::<String>()
                        };

                    let padded_total_io_read_detail = if total_io_read_detail.len() < io_read_width
                    {
                        format!("{:^width$}", total_io_read_detail, width = io_read_width)
                    } else {
                        total_io_read_detail
                            .chars()
                            .take(io_read_width)
                            .collect::<String>()
                    };

                    let padded_total_io_write_detail =
                        if total_io_write_detail.len() < io_write_width {
                            format!("{:^width$}", total_io_write_detail, width = io_write_width)
                        } else {
                            total_io_write_detail
                                .chars()
                                .take(io_write_width)
                                .collect::<String>()
                        };

                    let padded_user_detail = if user_detail.len() < user_width {
                        format!("{:^width$}", user_detail, width = user_width)
                    } else {
                        user_detail.chars().take(user_width).collect::<String>()
                    };

                    let padded_parent_detail = if parent_detail.len() < parent_width {
                        format!("{:^width$}", parent_detail, width = parent_width)
                    } else {
                        parent_detail.chars().take(parent_width).collect::<String>()
                    };

                    let padded_thread_detail = if thread_detail.len() < thread_width {
                        format!("{:^width$}", thread_detail, width = thread_width)
                    } else {
                        thread_detail.chars().take(thread_width).collect::<String>()
                    };

                    let process_info_detail = Line::from(vec![
                        Span::styled(
                            padded_status_detail,
                            Style::default().fg(app_color_info.process_text_color),
                        ),
                        Span::styled(
                            padded_elapsed_detail,
                            Style::default().fg(app_color_info.base_app_text_color),
                        ),
                        Span::styled(
                            padded_current_io_read_detail,
                            Style::default().fg(app_color_info.base_app_text_color),
                        ),
                        Span::styled(
                            padded_current_io_write_detail,
                            Style::default().fg(app_color_info.base_app_text_color),
                        ),
                        Span::styled(
                            padded_user_detail,
                            Style::default().fg(app_color_info.base_app_text_color),
                        ),
                        Span::styled(
                            padded_parent_detail,
                            Style::default().fg(app_color_info.base_app_text_color),
                        ),
                        Span::styled(
                            padded_thread_detail,
                            Style::default().fg(app_color_info.base_app_text_color),
                        ),
                    ]);

                    let process_info_detail_extra = Line::from(vec![
                        Span::styled(
                            format!("{:^width$}", "", width = status_width),
                            Style::default().fg(app_color_info.process_text_color),
                        ),
                        Span::styled(
                            format!("{:^width$}", "", width = elapsed_width),
                            Style::default().fg(app_color_info.base_app_text_color),
                        ),
                        Span::styled(
                            padded_total_io_read_detail,
                            Style::default().fg(app_color_info.base_app_text_color),
                        ),
                        Span::styled(
                            padded_total_io_write_detail,
                            Style::default().fg(app_color_info.base_app_text_color),
                        ),
                        Span::styled(
                            format!("{:^width$}", "", width = user_width),
                            Style::default().fg(app_color_info.base_app_text_color),
                        ),
                        Span::styled(
                            format!("{:^width$}", "", width = parent_width),
                            Style::default().fg(app_color_info.base_app_text_color),
                        ),
                        Span::styled(
                            format!("{:^width$}", "", width = thread_width),
                            Style::default().fg(app_color_info.base_app_text_color),
                        ),
                    ]);

                    frame.render_widget(process_info_detail, process_info_detail_layout);
                    frame.render_widget(process_info_detail_extra, extra_detail_layout);

                    // ------------------------------------------------------------
                    // Memory Usage Metrics and graph on the middle
                    // ------------------------------------------------------------

                    let [process_memory_usage_percentage_layout, process_memory_usage_graph_layout, process_memory_usage_bytes_layout] =
                        Layout::horizontal(vec![
                            Constraint::Fill(3),
                            Constraint::Fill(4),
                            Constraint::Fill(3),
                        ])
                        .areas(process_memory_usage_layout);

                    // ------------------------------------------------------------
                    // Memory Usage Percentage on the right side of the memory usage graph
                    // ------------------------------------------------------------

                    let process_memory_usage_percentage: f64 =
                        ((process_detail.memory[process_detail.memory.len() - 1]) / total_memory)
                            * 100.0;
                    let process_memory_usage_percentage_formatting = if area.width < LARGE_WIDTH {
                        format!("M: {:.2}%", process_memory_usage_percentage)
                    } else {
                        format!("MEMORY: {:.2}%", process_memory_usage_percentage)
                    };

                    let [_, process_memory_usage_percentage_layout, _] = Layout::horizontal(vec![
                        Constraint::Fill(1),
                        Constraint::Length(process_memory_usage_percentage_formatting.len() as u16),
                        Constraint::Fill(1),
                    ])
                    .areas(process_memory_usage_percentage_layout);
                    let [_, padded_process_memory_usage_percentage_layout] =
                        Layout::vertical(vec![Constraint::Fill(1), Constraint::Length(1)])
                            .areas(process_memory_usage_percentage_layout);

                    let process_memory_usage_percentage_line = Line::from(vec![Span::styled(
                        process_memory_usage_percentage_formatting,
                        Style::default().fg(app_color_info.process_title_color),
                    )
                    .bold()]);

                    frame.render_widget(
                        process_memory_usage_percentage_line,
                        padded_process_memory_usage_percentage_layout,
                    );

                    // get the process memory history
                    let process_memory = process_detail.memory.clone();
                    let num_points_to_display = graph_show_range.min(process_memory.len());
                    let start_idx = process_memory.len().saturating_sub(num_points_to_display);

                    let mut process_memory_points: Vec<(f64, f64)> = process_memory[start_idx..]
                        .iter()
                        .enumerate()
                        .map(|(i, &usage)| {
                            let x = i as f64;
                            let y = if usage > 0.0 {
                                (usage / total_memory) * GRAPH_PERCENTAGE as f64
                            } else {
                                0.0
                            };
                            (x, y)
                        })
                        .collect();

                    process_memory_points = process_memory_points
                        .iter()
                        .map(|(x, y)| {
                            (
                                graph_show_range as f64 - (process_memory_points.len() as f64 - x),
                                *y,
                            )
                        })
                        .collect();

                    let dataset = Dataset::default()
                        .data(&process_memory_points)
                        .graph_type(GraphType::Bar)
                        .marker(Marker::Braille)
                        .style(Style::default().fg(app_color_info.used_memory_base_graph_color));

                    let x_axis = Axis::default().bounds([0.0, graph_show_range as f64]);

                    let y_axis = Axis::default().bounds([0.0, GRAPH_PERCENTAGE]);

                    let process_memory_chart = Chart::new(vec![dataset])
                        .x_axis(x_axis)
                        .y_axis(y_axis)
                        .bg(app_color_info.background_color);

                    frame.render_widget(process_memory_chart, process_memory_usage_graph_layout);

                    // ------------------------------------------------------------
                    // Memory Usage Bytes on the left side of the memory usage graph
                    // ------------------------------------------------------------
                    let process_memory_usage_bytes_formatting = process_to_kib_mib_gib(
                        process_detail.memory[process_detail.memory.len() - 1],
                    );

                    let [_, process_memory_usage_bytes_layout, _] = Layout::horizontal(vec![
                        Constraint::Fill(1),
                        Constraint::Length(process_memory_usage_bytes_formatting.len() as u16),
                        Constraint::Fill(1),
                    ])
                    .areas(process_memory_usage_bytes_layout);

                    let [_, padded_process_memory_usage_bytes_layout] =
                        Layout::vertical(vec![Constraint::Fill(1), Constraint::Length(1)])
                            .areas(process_memory_usage_bytes_layout);
                    let process_memory_usage_bytes_line = Line::from(vec![Span::styled(
                        process_memory_usage_bytes_formatting,
                        Style::default().fg(app_color_info.process_title_color),
                    )
                    .bold()]);

                    frame.render_widget(
                        process_memory_usage_bytes_line,
                        padded_process_memory_usage_bytes_layout,
                    );

                    // ------------------------------------------------------------
                    // CMD command on the bottom
                    // ------------------------------------------------------------
                    let [process_cmd_title_layout, process_cmd_info_layout] =
                        Layout::horizontal(vec![Constraint::Fill(2), Constraint::Fill(8)])
                            .areas(process_cmd_layout);

                    let [_, process_cmd_title_layout, _] = Layout::vertical(vec![
                        Constraint::Length(1),
                        Constraint::Length(1),
                        Constraint::Length(1),
                    ])
                    .areas(process_cmd_title_layout);
                    let [upper_process_cmd_layout, mid_process_cmd_layout, bottom_process_cmd_layout] =
                        Layout::vertical(vec![
                            Constraint::Length(1),
                            Constraint::Length(1),
                            Constraint::Length(1),
                        ])
                        .areas(process_cmd_info_layout);

                    let process_cmd_title_line = Line::from(vec![Span::styled(
                        format!(
                            "{:^width$}",
                            "CMD:",
                            width = process_cmd_title_layout.width as usize
                        ),
                        Style::default().fg(app_color_info.process_title_color),
                    )
                    .bold()]);

                    let seperated_cmd_line_vec = break_line_into_vectors_of_string(
                        process_detail.cmd.join(""),
                        upper_process_cmd_layout.width as usize,
                        3,
                    );
                    if seperated_cmd_line_vec.len() == 1 {
                        let first_line = Line::from(vec![Span::styled(
                            format!(
                                "{:^width$}",
                                seperated_cmd_line_vec[0],
                                width = mid_process_cmd_layout.width as usize
                            ),
                            Style::default().fg(app_color_info.base_app_text_color),
                        )
                        .bold()]);

                        frame.render_widget(first_line, mid_process_cmd_layout);
                    } else if seperated_cmd_line_vec.len() == 2 {
                        let first_line = Line::from(vec![Span::styled(
                            format!(
                                "{:^width$}",
                                seperated_cmd_line_vec[0],
                                width = upper_process_cmd_layout.width as usize
                            ),
                            Style::default().fg(app_color_info.base_app_text_color),
                        )
                        .bold()]);
                        let second_line = Line::from(vec![Span::styled(
                            format!(
                                "{:^width$}",
                                seperated_cmd_line_vec[1],
                                width = mid_process_cmd_layout.width as usize
                            ),
                            Style::default().fg(app_color_info.base_app_text_color),
                        )
                        .bold()]);

                        frame.render_widget(first_line, upper_process_cmd_layout);
                        frame.render_widget(second_line, mid_process_cmd_layout);
                    }
                    if seperated_cmd_line_vec.len() == 3 {
                        let first_line = Line::from(vec![Span::styled(
                            format!(
                                "{:^width$}",
                                seperated_cmd_line_vec[0],
                                width = upper_process_cmd_layout.width as usize
                            ),
                            Style::default().fg(app_color_info.base_app_text_color),
                        )
                        .bold()]);
                        let second_line = Line::from(vec![Span::styled(
                            format!(
                                "{:^width$}",
                                seperated_cmd_line_vec[1],
                                width = mid_process_cmd_layout.width as usize
                            ),
                            Style::default().fg(app_color_info.base_app_text_color),
                        )
                        .bold()]);
                        let third_line = Line::from(vec![Span::styled(
                            format!(
                                "{:^width$}",
                                seperated_cmd_line_vec[2],
                                width = bottom_process_cmd_layout.width as usize
                            ),
                            Style::default().fg(app_color_info.base_app_text_color),
                        )
                        .bold()]);

                        frame.render_widget(first_line, upper_process_cmd_layout);
                        frame.render_widget(second_line, mid_process_cmd_layout);
                        frame.render_widget(third_line, bottom_process_cmd_layout);
                    }

                    frame.render_widget(process_cmd_title_line, process_cmd_title_layout);
                } else {
                    return;
                }
            }
            None => {
                return;
            }
        };
    }

    // for each column of different info of process
    let [pid, program, user, memory, cpu_usage] = Layout::horizontal([
        // Constraint::Ratio(15, 100),
        // Constraint::Ratio(40, 100),
        // Constraint::Ratio(15, 100),
        // Constraint::Ratio(20, 100),
        // Constraint::Ratio(10, 100),
        Constraint::Fill(1),
        Constraint::Fill(4),
        Constraint::Fill(1),
        Constraint::Fill(2),
        Constraint::Fill(1),
    ])
    .areas(title_layout);

    let mut pid_width = pid.width as usize;
    let mut program_width = program.width as usize;
    let mut command_width = 0;
    let mut thread_width = 0;
    let mut user_width = user.width as usize;
    let mut memory_width = memory.width as usize;
    let mut cpu_usage_width = cpu_usage.width as usize;

    if area.width > MEDIUM_WIDTH && area.width <= LARGE_WIDTH {
        let [pid, program, command, user, memory, cpu_usage] = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Fill(2),
            Constraint::Fill(3),
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
        ])
        .areas(title_layout);
        pid_width = pid.width as usize;
        program_width = program.width as usize;
        command_width = command.width as usize;
        user_width = user.width as usize;
        memory_width = memory.width as usize;
        cpu_usage_width = cpu_usage.width as usize;
    } else if area.width > LARGE_WIDTH {
        let [pid, program, command, thread, user, memory, cpu_usage] = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Fill(3),
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
        ])
        .areas(title_layout);
        pid_width = pid.width as usize;
        program_width = program.width as usize;
        command_width = command.width as usize;
        thread_width = thread.width as usize;
        user_width = user.width as usize;
        memory_width = memory.width as usize;
        cpu_usage_width = cpu_usage.width as usize;
    }

    // Pad the string to take up respective width
    let pid_title = String::from("Pid: ");
    let program_title = String::from("Program: ");
    let command_title = String::from("Command: ");
    let thread_title = String::from("Threads: ");
    let user_title = String::from("User: ");
    let memory_title = String::from("Mem: ");
    let cpu_usage_title = String::from("Cpu%: ");

    let padded_pid_title = if pid_title.len() < pid_width {
        format!("{:width$}", pid_title, width = pid_width)
    } else {
        pid_title.chars().take(pid_width).collect::<String>()
    };

    let padded_program_title = if program_title.len() < program_width {
        format!("{:width$}", program_title, width = program_width)
    } else {
        program_title
            .chars()
            .take(program_width)
            .collect::<String>()
    };

    let padded_command_title = if command_title.len() < command_width {
        format!("{:width$}", command_title, width = command_width)
    } else {
        command_title
            .chars()
            .take(command_width)
            .collect::<String>()
    };

    let padded_thread_title = if thread_title.len() < thread_width {
        format!("{:width$}", thread_title, width = thread_width)
    } else {
        thread_title.chars().take(thread_width).collect::<String>()
    };

    let padded_user_title = if user_title.len() < user_width {
        format!("{:width$}", user_title, width = user_width)
    } else {
        user_title.chars().take(user_width).collect::<String>()
    };

    let padded_memory_title = if memory_title.len() < memory_width {
        format!("{:width$}", memory_title, width = memory_width)
    } else {
        memory_title.chars().take(memory_width).collect::<String>()
    };

    let padded_cpu_usage_title = if cpu_usage_title.len() < cpu_usage_width {
        format!("{:width$}", cpu_usage_title, width = cpu_usage_width)
    } else {
        cpu_usage_title
            .chars()
            .take(cpu_usage_width)
            .collect::<String>()
    };

    let process_title = Line::from(vec![
        Span::styled(
            padded_pid_title,
            Style::default()
                .fg(app_color_info.process_title_color)
                .bold(),
        ),
        Span::styled(
            padded_program_title,
            Style::default()
                .fg(app_color_info.process_title_color)
                .bold(),
        ),
        Span::styled(
            padded_command_title,
            Style::default()
                .fg(app_color_info.process_title_color)
                .bold(),
        ),
        Span::styled(
            padded_thread_title,
            Style::default()
                .fg(app_color_info.process_title_color)
                .bold(),
        ),
        Span::styled(
            padded_user_title,
            Style::default()
                .fg(app_color_info.process_title_color)
                .bold(),
        ),
        Span::styled(
            padded_memory_title,
            Style::default()
                .fg(app_color_info.process_title_color)
                .bold(),
        ),
        Span::styled(
            padded_cpu_usage_title,
            Style::default()
                .fg(app_color_info.process_title_color)
                .bold(),
        ),
    ]);

    frame.render_widget(process_title, title_layout);

    let sorted_process = sort_process(
        process_sort_type.clone(),
        process_sort_is_reversed,
        process_filter_without_underscore_extension,
        process_data,
    );

    *process_current_list = sorted_process.clone();

    let process_list: Vec<ListItem> = sorted_process
        .iter()
        .map(|value| {
            // Pad the string to take up respective width
            let pid = format!("{}", value.pid);
            let program = value.name.clone();
            let command = if value.cmd.len() > 0 {
                value.cmd.join(" ")
            } else {
                value.name.clone()
            };
            #[cfg(target_os = "windows")]
            // due to unoptimized way of getting thread count on window platform which hurt performence,
            // will not support this till a solution is found
            let thread = "?".to_string();

            #[cfg(any(target_os = "macos", target_os = "linux"))]
            let thread = value.thread_count.to_string();

            let user = value.user.clone();
            let memory = process_to_kib_mib_gib(value.memory[value.memory.len() - 1]);
            let cpu_usage = format!(
                "{:.2}%",
                round_to_2_decimal(value.cpu_usage[value.cpu_usage.len() - 1])
            );

            let padded_pid = if pid.len() < pid_width {
                format!("{:width$}", pid, width = pid_width)
            } else {
                pid.chars().take(pid_width).collect::<String>()
            };

            let padded_program = if program.len() < program_width {
                format!("{:width$}", program, width = program_width)
            } else {
                let mut pgm = program.chars().take(program_width - 2).collect::<String>();
                pgm.push_str("  ");
                pgm
            };

            let padded_command = if command.len() < command_width {
                format!("{:width$}", command, width = command_width)
            } else {
                let mut cmd = command.chars().take(command_width - 2).collect::<String>();
                cmd.push_str("  ");
                cmd
            };

            let padded_thread = if thread.len() < thread_width {
                format!("{:width$}", thread, width = thread_width)
            } else {
                thread.chars().take(thread_width).collect::<String>()
            };

            let padded_user = if user.len() < user_width {
                format!("{:width$}", user, width = user_width)
            } else {
                let mut user = user.chars().take(user_width - 2).collect::<String>();
                user.push_str("  ");
                user
            };

            let padded_memory = if memory.len() < memory_width {
                format!("{:width$}", memory, width = memory_width)
            } else {
                memory.chars().take(memory_width).collect::<String>()
            };

            let padded_cpu_usage = if cpu_usage.len() < cpu_usage_width {
                format!("{:width$}", cpu_usage, width = cpu_usage_width)
            } else {
                cpu_usage.chars().take(cpu_usage_width).collect::<String>()
            };

            let mut process_inline_content_vec = vec![
                Span::styled(
                    padded_pid,
                    Style::default().fg(app_color_info.base_app_text_color),
                ),
                Span::styled(
                    padded_program,
                    Style::default().fg(app_color_info.process_text_color),
                ),
                Span::styled(
                    padded_user,
                    Style::default().fg(app_color_info.base_app_text_color),
                ),
                Span::styled(
                    padded_memory,
                    Style::default().fg(app_color_info.process_text_color),
                ),
                Span::styled(
                    padded_cpu_usage,
                    Style::default().fg(app_color_info.base_app_text_color),
                ),
            ];
            if area.width > MEDIUM_WIDTH && area.width <= LARGE_WIDTH {
                process_inline_content_vec.insert(
                    2,
                    Span::styled(
                        padded_command,
                        Style::default().fg(app_color_info.base_app_text_color),
                    ),
                );
            } else if area.width > LARGE_WIDTH {
                process_inline_content_vec.insert(
                    2,
                    Span::styled(
                        padded_command,
                        Style::default().fg(app_color_info.base_app_text_color),
                    ),
                );
                process_inline_content_vec.insert(
                    3,
                    Span::styled(
                        padded_thread,
                        Style::default().fg(app_color_info.process_text_color),
                    ),
                );
            }

            let process = Line::from(process_inline_content_vec);

            ListItem::new(process)
        })
        .collect();

    *process_selectable_entries = process_list.len() as usize;

    // Create the combined list
    let process_info_list = List::new(process_list).highlight_style(
        Style::default()
            .bg(app_color_info.process_selected_color_bg)
            .fg(app_color_info.process_selected_color_fg)
            .bold(),
    );
    // Render the combined list with state
    frame.render_stateful_widget(
        process_info_list,
        process_list_layout,
        process_selected_state,
    );
}
