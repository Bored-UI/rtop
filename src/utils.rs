use std::{cmp::Ordering, collections::HashMap};

use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    symbols::border,
    text::{Line, Span},
    widgets::Block,
    Frame,
};

use crate::{
    app::AppColorInfo,
    types::{
        AppPopUpType, CProcessesInfo, CSysInfo, CpuData, CurrentProcessSignalStateData, DiskData,
        MemoryData, NetworkData, ProcessData, ProcessSortType, ProcessesInfo, SysInfo,
    },
};

pub fn process_sys_info(current_sys_info: &mut SysInfo, collected_sys_info: CSysInfo) {
    // -------------------------------------------
    //
    //             CPU INFO UPDATE
    //
    // -------------------------------------------
    if current_sys_info.cpus.len() == 0 {
        for cpu in collected_sys_info.cpus.iter() {
            let cpu = CpuData::new(cpu.id as i8, cpu.brand.clone(), cpu.usage);
            current_sys_info.cpus.push(cpu);
        }
    } else {
        for cpu in collected_sys_info.cpus.iter() {
            current_sys_info.cpus[cpu.id as usize + 1].update(cpu.id as i8, cpu.usage);
        }
    }

    // -------------------------------------------
    //
    //           RAM MEMORY INFO UPDATE
    //
    // -------------------------------------------
    if current_sys_info.memory.total_memory == -0.1 {
        current_sys_info.memory = MemoryData::new(
            collected_sys_info.memory.total_memory,
            collected_sys_info.memory.available_memory,
            collected_sys_info.memory.used_memory,
            collected_sys_info.memory.used_swap,
            collected_sys_info.memory.free_memory,
            collected_sys_info.memory.cached_memory,
        );
    } else {
        current_sys_info.memory.update(
            collected_sys_info.memory.total_memory,
            collected_sys_info.memory.available_memory,
            collected_sys_info.memory.used_memory,
            collected_sys_info.memory.used_swap,
            collected_sys_info.memory.free_memory,
            collected_sys_info.memory.cached_memory,
        );
    }

    // -------------------------------------------
    //
    //            DISK INFO UPDATE
    //
    // -------------------------------------------
    if current_sys_info.disks.len() == 0 {
        for disk in collected_sys_info.disks.iter() {
            let disk = DiskData::new(
                disk.name.clone(),
                disk.total_space,
                disk.available_space,
                disk.used_space,
                disk.bytes_written,
                disk.bytes_written,
                disk.file_system.clone(),
                disk.mount_point.clone(),
                disk.kind.clone(),
            );
            current_sys_info
                .disks
                .insert(disk.mount_point.clone(), disk);
        }
    } else {
        // need slightly more processing to address the following
        // 1. Update existing disk data with new information
        // 2. Handle disk removals and additions

        // update all existing disk data is_updated field to false
        for disk in current_sys_info.disks.values_mut() {
            disk.is_updated = false;
        }

        // loop through all collected disk data and update existing disk data or create new one
        for disk in collected_sys_info.disks.iter() {
            let existing_disk = current_sys_info.disks.get_mut(&disk.mount_point);
            match existing_disk {
                Some(e_d) => {
                    e_d.update(
                        disk.name.clone(),
                        disk.total_space,
                        disk.available_space,
                        disk.used_space,
                        disk.bytes_written,
                        disk.bytes_read,
                        disk.file_system.clone(),
                        disk.mount_point.clone(),
                        disk.kind.clone(),
                    );
                }
                None => {
                    let disk = DiskData::new(
                        disk.name.clone(),
                        disk.total_space,
                        disk.available_space,
                        disk.used_space,
                        disk.bytes_written,
                        disk.bytes_read,
                        disk.file_system.clone(),
                        disk.mount_point.clone(),
                        disk.kind.clone(),
                    );
                    current_sys_info
                        .disks
                        .insert(disk.mount_point.clone(), disk);
                }
            }
        }

        // now remove those that is_updated field is false as it was indicated they were no longer connected
        let keys_to_remove: Vec<String> = current_sys_info
            .disks
            .iter()
            .filter(|(_, disk)| !disk.is_updated)
            .map(|(key, _)| key.clone())
            .collect();

        for key in keys_to_remove {
            current_sys_info.disks.remove(&key);
        }
    }

    // -------------------------------------------
    //
    //          NETWORKS INFO UPDATE
    //
    // -------------------------------------------
    if current_sys_info.networks.len() == 0 {
        for network in collected_sys_info.networks.iter() {
            let network = NetworkData::new(
                network.interface_name.clone(),
                network.ip_network.clone(),
                network.current_received,
                network.current_transmitted,
                network.total_received,
                network.total_transmitted,
            );
            current_sys_info
                .networks
                .insert(network.interface_name.clone(), network);
        }
    } else {
        // need slightly more processing to address the following
        // 1. Update existing network data with new information
        // 2. Handle network removals and additions due to disconnection or new connection

        // update all existing network data is_updated field to false
        for network in current_sys_info.networks.values_mut() {
            network.is_updated = false;
        }

        // loop through all collected network data and update existing network data or create new one
        for network in collected_sys_info.networks.iter() {
            let existing_network = current_sys_info.networks.get_mut(&network.interface_name);
            match existing_network {
                Some(e_n) => {
                    e_n.update(
                        network.interface_name.clone(),
                        network.ip_network.clone(),
                        network.current_received,
                        network.current_transmitted,
                        network.total_received,
                        network.total_transmitted,
                    );
                }
                None => {
                    let network = NetworkData::new(
                        network.interface_name.clone(),
                        network.ip_network.clone(),
                        network.current_received,
                        network.current_transmitted,
                        network.total_received,
                        network.total_transmitted,
                    );
                    current_sys_info
                        .networks
                        .insert(network.interface_name.clone(), network);
                }
            }
        }

        // now remove those that is_updated field is false as it was indicated they were no longer connected
        let keys_to_remove: Vec<String> = current_sys_info
            .networks
            .iter()
            .filter(|(_, network)| !network.is_updated)
            .map(|(key, _)| key.clone())
            .collect();

        for key in keys_to_remove {
            current_sys_info.networks.remove(&key);
        }
    }

    // drop the collected system info that we got from a seperated thread
    drop(collected_sys_info);
}

pub fn process_processes_info(
    current_process_info: &mut ProcessesInfo,
    collected_process_info: CProcessesInfo,
    process_detail_info: &mut Option<HashMap<String, ProcessData>>,
) {
    if current_process_info.processes.len() == 0 {
        for process in collected_process_info.processes.iter() {
            let process_data = ProcessData::new(
                process.pid,
                process.name.clone(),
                process.exe_path.clone(),
                process.cmd.clone(),
                process.user.clone(),
                process.cpu_usage,
                process.thread_count,
                process.memory,
                process.status.clone(),
                process.elapsed,
                process.parent.clone(),
                process.current_read_disk_usage,
                process.total_read_disk_usage,
                process.current_write_disk_usage,
                process.total_write_disk_usage,
            );
            let pid_string = format!("{}", process.pid);
            current_process_info
                .processes
                .insert(pid_string, process_data.clone());
        }
    } else {
        for process in current_process_info.processes.values_mut() {
            process.is_updated = false;
        }

        for process in collected_process_info.processes.iter() {
            let current_process = current_process_info
                .processes
                .get_mut(&process.pid.to_string());
            match current_process {
                Some(p) => {
                    p.update(
                        process.pid,
                        process.name.clone(),
                        process.exe_path.clone(),
                        process.cmd.clone(),
                        process.user.clone(),
                        process.cpu_usage,
                        process.thread_count,
                        process.memory,
                        process.status.clone(),
                        process.elapsed,
                        process.parent.clone(),
                        process.current_read_disk_usage,
                        process.total_read_disk_usage,
                        process.current_write_disk_usage,
                        process.total_write_disk_usage,
                    );

                    // if there process detail info showing, update the process detail info
                    if let Some(hashmap) = process_detail_info.as_mut() {
                        let key = process.pid.to_string();
                        if hashmap.contains_key(&key) {
                            hashmap.entry(key).and_modify(|value| *value = p.to_owned());
                        }
                    }
                }
                None => {
                    let p = ProcessData::new(
                        process.pid,
                        process.name.clone(),
                        process.exe_path.clone(),
                        process.cmd.clone(),
                        process.user.clone(),
                        process.cpu_usage,
                        process.thread_count,
                        process.memory,
                        process.status.clone(),
                        process.elapsed,
                        process.parent.clone(),
                        process.current_read_disk_usage,
                        process.total_read_disk_usage,
                        process.current_write_disk_usage,
                        process.total_write_disk_usage,
                    );
                    let pid_string = format!("{}", process.pid);
                    current_process_info.processes.insert(pid_string, p);
                }
            }
        }

        let keys_to_remove: Vec<String> = current_process_info
            .processes
            .iter()
            .filter(|(_, process)| !process.is_updated)
            .map(|(key, _)| key.clone())
            .collect();

        for key in keys_to_remove {
            current_process_info.processes.remove(&key);
        }
    }

    drop(collected_process_info);
}

// the line to show the current tick
pub fn get_tick_line_ui(tick: u64, app_color_info: &AppColorInfo) -> Line {
    let refresh_tick = Line::from(vec![
        Span::styled("  ", Style::default().fg(app_color_info.app_title_color)),
        Span::styled("-", Style::default().fg(app_color_info.key_text_color)).bold(),
        Span::styled(
            format!(" {}ms ", tick),
            Style::default().fg(app_color_info.app_title_color).bold(),
        ),
        Span::styled("+", Style::default().fg(app_color_info.key_text_color)).bold(),
        Span::styled("  ", Style::default().fg(app_color_info.app_title_color)),
    ]);

    return refresh_tick;
}

// break line into multiple line into a vector based on desire len of string (String -> Vec<String>)
pub fn break_line_into_vectors_of_string(
    line: String,
    max_length_per_string: usize,
    vector_size: usize,
) -> Vec<String> {
    if vector_size < 1 {
        return vec![];
    }
    if line.len() <= max_length_per_string {
        return vec![line];
    } else {
        let chars_vec: Vec<char> = line.chars().collect();
        let mut line_vec = vec![];
        for i in 0..vector_size {
            let start = i * max_length_per_string;
            let end = (i + 1) * max_length_per_string;

            if end >= line.len() {
                line_vec.push(chars_vec[start..line.len()].iter().collect());
                break;
            }

            line_vec.push(chars_vec[start..end].iter().collect());
        }

        return line_vec;
    }
}

pub fn round_to_2_decimal(value: f32) -> f32 {
    (value * 100.0).round() / 100.0
}

pub fn process_to_kib_mib_gib(value: f64) -> String {
    let mut value = value;
    let mut unit = "B";

    if value >= 1024.0 {
        value /= 1024.0;
        unit = "KiB";
    }

    if value >= 1024.0 {
        value /= 1024.0;
        unit = "MiB";
    }

    if value >= 1024.0 {
        value /= 1024.0;
        unit = "GiB";
    }

    return format!("{:.2} {}", ((value * 1000.0).round() / 1000.0), unit);
}

pub fn format_seconds(value: u64) -> String {
    let days = value / (24 * 60 * 60);
    let hours = value % (24 * 60 * 60) / (60 * 60);
    let minutes = value % (60 * 60) / 60;
    let seconds = value % 60;
    if days > 0 {
        return format!("{}:{}:{}:{}", days, hours, minutes, seconds);
    } else {
        return format!("{}:{}:{}", hours, minutes, seconds);
    }
}

// function to sort and filter the process list based on user selected sort type, sorting order and filtering input
pub fn sort_process(
    sort_type: ProcessSortType,
    is_reversed: bool,
    filter: String,
    process_data: &HashMap<String, ProcessData>,
) -> Vec<ProcessData> {
    // we first map the hashmap into a vec for easy processing
    let mut processes: Vec<ProcessData> = process_data
        .iter()
        .map(|(_, value)| value)
        .cloned()
        .collect();

    // if user input for filter is not empty, we will retrieve those that name/cmd/user is matching the user inpu
    if !filter.is_empty() {
        processes.retain(|process| {
            process.name.to_lowercase().contains(&filter.to_lowercase())
                || process
                    .cmd
                    .join(" ")
                    .to_lowercase()
                    .contains(&filter.to_lowercase())
                || process.user.to_lowercase().contains(&filter.to_lowercase())
        });
    }

    if sort_type == ProcessSortType::Thread {
        processes.sort_by(|a, b| {
            let ordering = a
                .thread_count
                .partial_cmp(&b.thread_count)
                .unwrap_or(Ordering::Equal);
            if is_reversed {
                ordering.reverse()
            } else {
                ordering
            }
        });
    } else if sort_type == ProcessSortType::Memory {
        processes.sort_by(|a, b| {
            let ordering = a.memory[a.memory.len() - 1]
                .partial_cmp(&b.memory[b.memory.len() - 1])
                .unwrap_or(Ordering::Equal);
            if is_reversed {
                ordering.reverse()
            } else {
                ordering
            }
        });
    } else if sort_type == ProcessSortType::Cpu {
        processes.sort_by(|a, b| {
            let ordering = a
                .cpu_usage
                .partial_cmp(&b.cpu_usage)
                .unwrap_or(Ordering::Equal);
            if is_reversed {
                ordering.reverse()
            } else {
                ordering
            }
        });
    } else if sort_type == ProcessSortType::Pid {
        processes.sort_by(|a, b| {
            let ordering = a.pid.partial_cmp(&b.pid).unwrap_or(Ordering::Equal);
            if is_reversed {
                ordering.reverse()
            } else {
                ordering
            }
        });
    } else if sort_type == ProcessSortType::Name {
        processes.sort_by(|a, b| {
            let ordering = a.name.to_lowercase().cmp(&b.name.to_lowercase());
            if is_reversed {
                ordering.reverse()
            } else {
                ordering
            }
        });
    } else if sort_type == ProcessSortType::Command {
        processes.sort_by(|a, b| {
            // there is cases where command is empty vector, in this case it will be replace by the process name
            let a_command = if a.cmd.is_empty() {
                a.name.clone()
            } else {
                a.cmd.join(" ")
            };
            let b_command = if b.cmd.is_empty() {
                b.name.clone()
            } else {
                b.cmd.join(" ")
            };
            let ordering = a_command.to_lowercase().cmp(&b_command.to_lowercase());
            if is_reversed {
                ordering.reverse()
            } else {
                ordering
            }
        })
    } else if sort_type == ProcessSortType::User {
        processes.sort_by(|a, b| {
            let ordering = a.user.to_lowercase().cmp(&b.user.to_lowercase());
            if is_reversed {
                ordering.reverse()
            } else {
                ordering
            }
        })
    }
    return processes;
}

pub fn render_pop_up_menu(
    area: Rect,
    frame: &mut Frame,
    pop_up_type: &mut AppPopUpType,
    current_process_signal_state_data: &CurrentProcessSignalStateData,
    app_color_info: &AppColorInfo,
) {
    let pop_up_dimension: (u16, u16) = if *pop_up_type == AppPopUpType::KillConfirmation
        || *pop_up_type == AppPopUpType::TerminateConfirmation
    {
        (50, 10)
    } else {
        (75.min(area.width), 25.min(area.height))
    };

    let [_, pop_up_width, _] = Layout::horizontal(vec![
        Constraint::Fill(1),
        Constraint::Length(pop_up_dimension.0),
        Constraint::Fill(1),
    ])
    .areas(area);

    let [_, pop_up, _] = Layout::vertical(vec![
        Constraint::Fill(1),
        Constraint::Length(pop_up_dimension.1),
        Constraint::Fill(1),
    ])
    .areas(pop_up_width);

    let info = Line::from(vec![Span::styled(
        pop_up_type.get_string_name(),
        Style::default().fg(app_color_info.app_title_color).bold(),
    )]);

    let pop_up_block = Block::bordered()
        .title(info.left_aligned())
        .style(app_color_info.background_color)
        .border_style(app_color_info.pop_up_color)
        .border_set(border::ROUNDED);

    // Render the pop-up block second (centered)
    frame.render_widget(pop_up_block, pop_up);

    // for kill or termination signal pop up
    if *pop_up_type == AppPopUpType::KillConfirmation
        || *pop_up_type == AppPopUpType::TerminateConfirmation
    {
        let [_, padded_pop_up, _] = Layout::horizontal(vec![
            Constraint::Fill(1),
            Constraint::Fill(8),
            Constraint::Fill(1),
        ])
        .areas(pop_up);
        let [_, info_layout, _, button_layout, _] = Layout::vertical(vec![
            Constraint::Fill(1),
            Constraint::Length(2),
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Fill(1),
        ])
        .areas(padded_pop_up);

        let [signal_info, pid_info] =
            Layout::vertical(vec![Constraint::Length(1), Constraint::Length(1)]).areas(info_layout);
        let [yes_button_layout, no_button_layout] =
            Layout::horizontal(vec![Constraint::Fill(1), Constraint::Fill(1)]).areas(button_layout);

        let signal_type = if *pop_up_type == AppPopUpType::KillConfirmation {
            Span::styled("KILL", Style::default().fg(app_color_info.key_text_color))
        } else {
            Span::styled("TERM", Style::default().fg(app_color_info.key_text_color))
        };

        // which signal information
        let signal_info_line = Line::from(vec![
            Span::styled(
                "SEND SIGNAL: ",
                Style::default().fg(app_color_info.base_app_text_color),
            )
            .bold(),
            signal_type,
        ]);
        // which PID information
        let pid_info_line = Line::from(vec![
            Span::styled(
                "TO PID: ",
                Style::default().fg(app_color_info.base_app_text_color),
            )
            .bold(),
            Span::styled(
                format!("{} ", current_process_signal_state_data.pid),
                Style::default().fg(app_color_info.key_text_color),
            ),
            Span::styled(
                format!("({})", current_process_signal_state_data.name),
                Style::default().fg(app_color_info.base_app_text_color),
            ),
        ]);

        frame.render_widget(signal_info_line, signal_info);
        frame.render_widget(pid_info_line, pid_info);

        // yes button confimation
        let [_, padded_yes_button_layout, _] = Layout::horizontal(vec![
            Constraint::Fill(1),
            Constraint::Length(15),
            Constraint::Fill(1),
        ])
        .areas(yes_button_layout);
        let [_, ppadded_yes_button_layout, _] = Layout::horizontal(vec![
            Constraint::Length(1),
            Constraint::Length(13),
            Constraint::Length(1),
        ])
        .areas(padded_yes_button_layout);
        let [_, yes_button_line_text_layout, _] = Layout::vertical(vec![
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .areas(ppadded_yes_button_layout);

        let mut yes_button_block = Block::bordered()
            .style(app_color_info.background_color)
            .border_style(app_color_info.pop_up_color)
            .border_set(border::ROUNDED);

        if current_process_signal_state_data.yes_confirmation {
            yes_button_block = yes_button_block.border_style(app_color_info.key_text_color);
        }

        let yes_button_line = Line::from(Span::styled(
            format!(
                "{:^width$}",
                "Yes (Y/y)".to_string(),
                width = yes_button_line_text_layout.width as usize
            ),
            Style::default().fg(app_color_info.base_app_text_color),
        ));

        // no button confirmation
        let [_, padded_no_button_layout, _] = Layout::horizontal(vec![
            Constraint::Fill(1),
            Constraint::Length(15),
            Constraint::Fill(1),
        ])
        .areas(no_button_layout);
        let [_, ppadded_no_button_layout, _] = Layout::horizontal(vec![
            Constraint::Length(1),
            Constraint::Length(13),
            Constraint::Length(1),
        ])
        .areas(padded_no_button_layout);
        let [_, no_button_line_text_layout, _] = Layout::vertical(vec![
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .areas(ppadded_no_button_layout);

        let mut no_button_block = Block::bordered()
            .style(app_color_info.background_color)
            .border_style(app_color_info.pop_up_color)
            .border_set(border::ROUNDED);

        if current_process_signal_state_data.no_confirmation {
            no_button_block = no_button_block.border_style(app_color_info.key_text_color);
        }

        let no_button_line = Line::from(Span::styled(
            format!(
                "{:^width$}",
                "No (N/n)".to_string(),
                width = no_button_line_text_layout.width as usize
            ),
            Style::default().fg(app_color_info.base_app_text_color),
        ));

        frame.render_widget(yes_button_block, padded_yes_button_layout);
        frame.render_widget(yes_button_line, yes_button_line_text_layout);

        frame.render_widget(no_button_block, padded_no_button_layout);
        frame.render_widget(no_button_line, no_button_line_text_layout);
    }
}
