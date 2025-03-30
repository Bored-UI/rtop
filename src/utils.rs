use ratatui::{
    style::{Style, Stylize},
    text::{Line, Span},
};

use crate::{
    tui::AppColorInfo,
    types::{CSysInfo, CpuData, DiskData, MemoryData, NetworkData, SysInfo},
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
                        network.current_received,
                        network.current_transmitted,
                        network.total_received,
                        network.total_transmitted,
                    );
                }
                None => {
                    let network = NetworkData::new(
                        network.interface_name.clone(),
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
