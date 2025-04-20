use std::{cmp::Ordering, collections::HashMap};

use ratatui::{
    style::{Style, Stylize},
    text::{Line, Span},
};

use crate::{
    tui::AppColorInfo,
    types::{
        CProcessesInfo, CSysInfo, CpuData, DiskData, MemoryData, NetworkData, ProcessData,
        ProcessSortType, ProcessesInfo, SysInfo,
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
            );
            let pid_string = format!("{}", process.pid);
            current_process_info
                .processes
                .insert(pid_string, process_data);
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
                Some(p) => p.update(
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
                ),
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

pub fn get_tick_line_ui(tick: u64, app_color_info: &AppColorInfo) -> Line {
    let refresh_tick = Line::from(vec![
        Span::styled("  ", Style::default().fg(app_color_info.app_title_color)),
        Span::styled("-", Style::default().fg(app_color_info.key_text_color)).bold(),
        Span::styled(
            format!(" {}ms ", tick),
            Style::default().fg(app_color_info.app_title_color),
        ),
        Span::styled("+", Style::default().fg(app_color_info.key_text_color)).bold(),
        Span::styled("  ", Style::default().fg(app_color_info.app_title_color)),
    ]);

    return refresh_tick;
}

pub fn round_to_2_decimal(value: f32) -> f32 {
    (value * 100.0).round() / 100.0
}

pub fn sort_process(
    sort_type: ProcessSortType,
    is_reversed: bool,
    filter: String,
    process_data: &HashMap<String, ProcessData>,
) -> Vec<ProcessData> {
    let mut processes: Vec<ProcessData> = process_data
        .iter()
        .map(|(_, value)| value)
        .cloned()
        .collect();
    if !filter.is_empty() {
        processes.retain(|process| {
            process.name.to_lowercase().contains(&filter)
                || process.cmd.join(" ").to_lowercase().contains(&filter)
                || process.user.to_lowercase().contains(&filter)
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
