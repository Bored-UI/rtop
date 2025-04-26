use std::collections::HashMap;

use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    symbols::border,
    text::{Line, Span},
    widgets::{Block, List, ListItem, ListState},
    Frame,
};

use crate::{
    tui::AppColorInfo,
    types::{ProcessData, ProcessSortType},
    utils::{get_tick_line_ui, round_to_2_decimal, sort_process},
};

const MEDIUM_WIDTH: u16 = 60;
const LARGE_WIDTH: u16 = 75;
const X_LARGE_WIDTH: u16 = 95;
const XX_LARGE_WIDTH: u16 = 120;

pub fn draw_process_info(
    tick: u64,
    process_data: &HashMap<String, ProcessData>,
    process_selectable_entries: &mut usize,
    process_selected_state: &mut ListState,
    process_sort_type: &ProcessSortType,
    process_sort_is_reversed: bool,
    process_filter: String,
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
            Style::default().fg(app_color_info.app_title_color),
        ),
    ]);

    let process_sort_is_reversed_intruction = Line::from(vec![
        Span::styled(" ", Style::default().fg(app_color_info.app_title_color)),
        Span::styled("R", Style::default().fg(app_color_info.key_text_color))
            .bold()
            .underlined(),
        Span::styled(
            "everse ",
            Style::default().fg(app_color_info.app_title_color),
        ),
    ]);

    let process_sort_slect_instruction = Line::from(vec![
        Span::styled("　< ", Style::default().fg(app_color_info.key_text_color)).bold(),
        Span::styled(
            ProcessSortType::get_sort_string_name(process_sort_type),
            Style::default().fg(app_color_info.app_title_color),
        ),
        Span::styled(" >　", Style::default().fg(app_color_info.key_text_color)).bold(),
    ]);
    let mut process_filter_without_underscore_extension: String = process_filter
        .chars()
        .take(process_filter.len() - 1)
        .collect();

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
                Style::default().fg(app_color_info.app_title_color),
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
                    Style::default().fg(app_color_info.app_title_color),
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
                    Style::default().fg(app_color_info.app_title_color),
                ),
                Span::styled("← ", Style::default().fg(app_color_info.key_text_color)).bold(),
            ])
        }
    };

    let mut main_block = Block::bordered()
        .title(select_instruction.left_aligned())
        .title_bottom(process_filter_instruction.left_aligned())
        .title_bottom(process_sort_is_reversed_intruction.right_aligned())
        .title_bottom(process_sort_slect_instruction.right_aligned())
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
        Constraint::Percentage(3),
        Constraint::Percentage(94),
        Constraint::Percentage(3),
    ])
    .areas(area);

    let [_, process_block, _] = Layout::horizontal([
        Constraint::Percentage(2),
        Constraint::Percentage(96),
        Constraint::Percentage(2),
    ])
    .areas(padded_vertical_inner);

    let [title_layout, _, process_list_layout] = Layout::vertical([
        Constraint::Percentage(3),
        Constraint::Percentage(2),
        Constraint::Percentage(95),
    ])
    .areas(process_block);

    let [pid, program, user, memory, cpu_usage] = Layout::horizontal([
        Constraint::Percentage(15),
        Constraint::Percentage(40),
        Constraint::Percentage(15),
        Constraint::Percentage(15),
        Constraint::Percentage(15),
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
            Constraint::Percentage(10),
            Constraint::Percentage(20),
            Constraint::Percentage(40),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
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
            Constraint::Percentage(10),
            Constraint::Percentage(17),
            Constraint::Percentage(35),
            Constraint::Percentage(13),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
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
            Style::default().fg(app_color_info.process_title_color),
        ),
        Span::styled(
            padded_program_title,
            Style::default().fg(app_color_info.process_title_color),
        ),
        Span::styled(
            padded_command_title,
            Style::default().fg(app_color_info.process_title_color),
        ),
        Span::styled(
            padded_thread_title,
            Style::default().fg(app_color_info.process_title_color),
        ),
        Span::styled(
            padded_user_title,
            Style::default().fg(app_color_info.process_title_color),
        ),
        Span::styled(
            padded_memory_title,
            Style::default().fg(app_color_info.process_title_color),
        ),
        Span::styled(
            padded_cpu_usage_title,
            Style::default().fg(app_color_info.process_title_color),
        ),
    ]);

    frame.render_widget(process_title, title_layout);

    let sorted_process = sort_process(
        process_sort_type.clone(),
        process_sort_is_reversed,
        process_filter_without_underscore_extension,
        process_data,
    );

    let process_list: Vec<ListItem> = sorted_process
        .iter()
        .map(|value| {
            let processed_memory = if value.memory[value.memory.len() - 1] > 1024.0 {
                let new_memory =
                    ((value.memory[value.memory.len() - 1] / 1024.0) * 1000.0).round() / 1000.0;
                if new_memory > 1024.0 {
                    format!(
                        "{:.1} GB",
                        (((new_memory / 1024.0) * 1000.0).round() / 1000.0) as f32
                    )
                } else {
                    format!("{} MB", new_memory as usize)
                }
            } else {
                format!("{} KB", value.memory[value.memory.len() - 1] as usize)
            };

            // Pad the string to take up respective width
            let pid = format!("{}", value.pid);
            let program = value.name.clone();
            let command = if value.cmd.len() > 0 {
                value.cmd.join(" ")
            } else {
                value.name.clone()
            };
            let thread = value.thread_count.to_string();
            let user = value.user.clone();
            let memory = processed_memory;
            let cpu_usage = format!("{:.2}%", round_to_2_decimal(value.cpu_usage));

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
                user.chars().take(user_width).collect::<String>()
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

    if let Some(selected) = process_selected_state.selected() {
        if selected > process_list.len() {
            process_selected_state.select(Some(0));
        }
    }

    // Create the combined list
    let process_info_list = List::new(process_list).highlight_style(
        Style::default()
            .bg(app_color_info.process_selected_color_bg)
            .fg(app_color_info.process_selected_color_fg),
    );
    // Render the combined list with state
    frame.render_stateful_widget(
        process_info_list,
        process_list_layout,
        process_selected_state,
    );
}
