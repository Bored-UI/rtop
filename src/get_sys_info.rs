use std::{
    process::Command,
    sync::mpsc::{Receiver, RecvTimeoutError, Sender},
    thread,
    time::{Duration, Instant},
};

use crate::types::{
    CCpuData, CDiskData, CMemoryData, CNetworkData, CProcessData, CProcessesInfo, CSysInfo,
};
use sysinfo::{Disks, Networks, Process, ProcessesToUpdate, System, Users};

const TO_GB: f64 = 1_073_741_824.0;
const TO_KB: f64 = 1024.0;

pub fn spawn_system_info_collector(
    tick_receiver: Receiver<u32>,
    tx: Sender<CSysInfo>,
    default_tick: u32,
) {
    // Spawn a worker thread to gather CPU info
    thread::spawn(move || {
        let mut sys = System::new_all();
        let mut disks = Disks::new();
        let mut networks = Networks::new();
        let mut last_refresh = Instant::now();
        let mut tick_value = default_tick; // Current tick in ms

        sys.refresh_all();
        disks.refresh(true);
        networks.refresh(true);

        loop {
            let elapsed = last_refresh.elapsed();
            let sleep_duration = if tick_value > elapsed.as_millis() as u32 {
                Duration::from_millis((tick_value - elapsed.as_millis() as u32).into())
            } else {
                Duration::from_millis(0)
            };

            match tick_receiver.recv_timeout(sleep_duration) {
                Ok(new_tick) => {
                    tick_value = new_tick;
                    continue; // don't collect this cycle, just updated tick
                }
                Err(RecvTimeoutError::Timeout) => {
                    // -------------------------------------------
                    //
                    //             CPU DATA COLLECTION
                    //
                    // -------------------------------------------

                    // Refresh CPU data
                    sys.refresh_cpu_all();
                    let cpus = sys.cpus();

                    // Gather CPU data
                    let mut cpu_data: Vec<CCpuData> = cpus
                        .iter()
                        .enumerate()
                        .map(|(index, cpu)| CCpuData {
                            id: index as i8,
                            brand: cpu.brand().to_string(),
                            usage: cpu.cpu_usage(),
                        })
                        .collect();

                    // we later add cpu avg info as the first entry of the collected cpu info vector
                    let avg_cpu_data = CCpuData {
                        id: -1 as i8,
                        brand: cpu_data[0].brand.clone(),
                        usage: sys.global_cpu_usage(),
                    };
                    cpu_data.insert(0, avg_cpu_data);

                    // -------------------------------------------
                    //
                    //          RAM MEMORY DATA COLLECTION
                    //
                    // -------------------------------------------

                    sys.refresh_memory();
                    let total_memory =
                        ((sys.total_memory() as f64 / TO_GB) * 100.0).round() / 100.0;
                    let available_memory =
                        ((sys.available_memory() as f64 / TO_GB) * 100.0).round() / 100.0;
                    let used_memory = ((sys.used_memory() as f64 / TO_GB) * 100.0).round() / 100.0;
                    let used_swap = ((sys.used_swap() as f64 / TO_GB) * 100.0).round() / 100.0;
                    let free_memory = ((sys.free_memory() as f64 / TO_GB) * 100.0).round() / 100.0;
                    let cached_memory = get_cached_memory();

                    let memory_data = CMemoryData {
                        total_memory,
                        available_memory,
                        used_memory,
                        used_swap,
                        free_memory,
                        cached_memory,
                    };

                    // -------------------------------------------
                    //
                    //            DISK DATA COLLECTION
                    //
                    // -------------------------------------------
                    disks.refresh(true);
                    let mut disk_data = Vec::new();
                    for disk in &disks {
                        let total_space =
                            ((disk.total_space() as f64 / TO_GB) * 100.0).round() / 100.0;
                        let available_space =
                            ((disk.available_space() as f64 / TO_GB) * 100.0).round() / 100.0;
                        let data = CDiskData {
                            name: disk.name().to_string_lossy().to_string(),
                            total_space,
                            available_space,
                            used_space: ((total_space - available_space) * 100.0).round() / 100.0,
                            bytes_written: ((disk.usage().written_bytes as f64 / TO_KB) * 1000.0)
                                .round()
                                / 1000.0,
                            bytes_read: ((disk.usage().read_bytes as f64 / TO_KB) * 1000.0).round()
                                / 1000.0,
                            file_system: disk.file_system().to_string_lossy().to_string(),
                            mount_point: disk.mount_point().to_string_lossy().to_string(),
                            kind: disk.kind().to_string(),
                        };

                        disk_data.push(data);
                    }

                    // -------------------------------------------
                    //
                    //          NETWORK DATA COLLECTION
                    //
                    // -------------------------------------------
                    networks.refresh(true);
                    let mut networks_data = Vec::new();
                    for (interface_name, network_data) in &networks {
                        let data = CNetworkData {
                            interface_name: interface_name.to_string(),
                            ip_network: if network_data.ip_networks().len() > 0 {
                                let mut ipv4_networks = Vec::new();
                                for ip in network_data.ip_networks() {
                                    if ip.addr.is_ipv4() {
                                        ipv4_networks.push(ip.addr.to_string());
                                    }
                                }
                                if ipv4_networks.is_empty() {
                                    None
                                } else {
                                    Some(ipv4_networks[0].clone())
                                }
                            } else {
                                None
                            },
                            current_received: ((network_data.received() as f64 / TO_KB) * 1000.0)
                                .round()
                                / 1000.0,
                            current_transmitted: ((network_data.transmitted() as f64 / TO_KB)
                                * 1000.0)
                                .round()
                                / 1000.0,
                            total_received: ((network_data.total_received() as f64 / TO_KB)
                                * 1000.0)
                                .round()
                                / 1000.0,
                            total_transmitted: ((network_data.total_transmitted() as f64 / TO_KB)
                                * 1000.0)
                                .round()
                                / 1000.0,
                        };
                        networks_data.push(data);
                    }

                    // -------------------------------------------
                    //
                    //    SEND COLLECTION DATA TO MAIN THREAD
                    //
                    // -------------------------------------------
                    let sys_info = CSysInfo {
                        cpus: cpu_data,
                        memory: memory_data,
                        disks: disk_data,
                        networks: networks_data,
                    };

                    // Send the data to the main thread
                    if let Err(e) = tx.send(sys_info) {
                        eprintln!("Failed to send System Info: {}", e);
                        break; // Exit loop if channel is disconnected
                    }

                    // Reset the last refresh time
                    last_refresh = Instant::now();
                }
                Err(RecvTimeoutError::Disconnected) => {
                    break;
                }
            }
        }
    });
}

// dedicate thread to collect process info only
pub fn spawn_process_info_collector(
    tick_receiver: Receiver<u32>,
    tx: Sender<CProcessesInfo>,
    default_tick: u32,
) {
    // Spawn a worker thread to gather CPU info
    thread::spawn(move || {
        let mut sys = System::new_all();
        let mut last_refresh = Instant::now();
        let mut tick_value = default_tick; // Current tick in ms

        sys.refresh_all();

        loop {
            let elapsed = last_refresh.elapsed();
            let sleep_duration = if tick_value > elapsed.as_millis() as u32 {
                Duration::from_millis((tick_value - elapsed.as_millis() as u32).into())
            } else {
                Duration::from_millis(0)
            };

            match tick_receiver.recv_timeout(sleep_duration) {
                Ok(new_tick) => {
                    tick_value = new_tick;
                    continue; // don't collect this cycle, just updated tick
                }
                Err(RecvTimeoutError::Timeout) => {
                    sys.refresh_processes(ProcessesToUpdate::All, true);
                    let users = Users::new_with_refreshed_list();
                    let mut processes = vec![];
                    // -------------------------------------------
                    //
                    //          PROCESS INFO COLLECTION
                    //
                    // -------------------------------------------
                    for (pid, process) in sys.processes() {
                        let mut user = "root";
                        let thread_count = get_thread_count(pid.as_u32() as i32, &process);

                        if process.user_id().is_some() {
                            let u = users.get_user_by_id(process.user_id().unwrap());
                            if u.is_some() {
                                user = u.unwrap().name();
                            }
                        }
                        let process_info = CProcessData {
                            pid: pid.as_u32(),
                            name: process.name().to_string_lossy().to_string(),
                            exe_path: if process.exe().is_some() {
                                Some(process.exe().unwrap().to_string_lossy().to_string())
                            } else {
                                None
                            },
                            cmd: process
                                .cmd()
                                .into_iter()
                                .map(|osstr| osstr.to_string_lossy().to_string())
                                .collect(),
                            user: user.to_string(),
                            cpu_usage: process.cpu_usage(),
                            thread_count,
                            memory: ((process.memory() as f64 / TO_KB) * 1000.0).round() / 1000.0,
                            status: process.status().to_string(),
                            elapsed: process.run_time(),
                        };

                        processes.push(process_info);
                    }

                    // -------------------------------------------
                    //
                    //  SEND COLLECTED PROCESS INFO TO MAIN THREAD
                    //
                    // -------------------------------------------
                    let process_info = CProcessesInfo { processes };

                    // Send the data to the main thread
                    if let Err(e) = tx.send(process_info) {
                        eprintln!("Failed to send Process Info: {}", e);
                        break; // Exit loop if channel is disconnected
                    }

                    // Reset the last refresh time
                    last_refresh = Instant::now();
                }
                Err(RecvTimeoutError::Disconnected) => {
                    eprintln!(
                        "Tick receiver channel disconnected. latest tick: {}",
                        tick_value
                    );
                    break;
                }
            }
        }
    });
}

fn get_thread_count(pid: i32, process: &Process) -> u32 {
    let mut thread_count = 0;

    #[cfg(target_os = "macos")]
    {
        use libproc::{proc_pid::pidinfo, task_info::TaskInfo};
        if let Ok(task_info) = pidinfo::<TaskInfo>(pid, 0) {
            thread_count = task_info.pti_threadnum;
        }
    }

    #[cfg(target_os = "linux")]
    {
        if let Some(tasks) = process.tasks() {
            thread_count = tasks.len() as i32;
        }
    }

    return thread_count as u32;
}

fn get_cached_memory() -> f64 {
    let mut cached_memory = 0.0;

    #[cfg(target_os = "macos")]
    {
        let macos_cache = get_macos_cache_memory();
        if let Some(cache) = macos_cache {
            cached_memory = ((cache as f64 / TO_GB) * 100.0).round() / 100.0;
        }
    }

    #[cfg(target_os = "linux")]
    {
        let linux_cache = get_linux_cached_memory();
        if let Some(cache) = linux_cache {
            cached_memory = ((cache as f64 / TO_GB) * 100.0).round() / 100.0;
        }
    }

    #[cfg(target_os = "windows")]
    {
        let windows_cache = get_window_cached_memory();
        if let Some(cache) = windows_cache {
            cached_memory = ((cache as f64 / TO_GB) * 100.0).round() / 100.0;
        }
    }

    return cached_memory;
}

// A hack, but it gets the job done
#[cfg(target_os = "macos")]
fn get_macos_cache_memory() -> Option<u64> {
    use libc::sysconf;
    use libc::_SC_PAGESIZE;

    let page_size = unsafe {
        let size = sysconf(_SC_PAGESIZE);
        if size <= 0 {
            16384 // Default page size if sysconf fails
        } else {
            size
        }
    };

    // Execute the vm_stat command to get cached memory information
    let output = Command::new("sh")
        .arg("-c")
        .arg("vm_stat | awk '/File-backed pages/ {print $3*1}'")
        .output()
        .ok();

    match output {
        Some(output) => {
            // Parse the output to get the number of cached pages
            let cache_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
            let pages = cache_str.parse::<u64>().ok()?;

            // Calculate the cached memory in bytes
            return Some(pages * page_size as u64);
        }
        None => return None,
    }
}

#[cfg(target_os = "linux")]
fn get_linux_cached_memory() -> Option<u64> {
    use std::fs;
    let data = fs::read_to_string("/proc/meminfo").ok()?;
    for line in data.lines() {
        if line.starts_with("Cached:") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if let Some(value) = parts.get(1) {
                return value.parse::<u64>().ok().map(|kb| kb * 1024); // kB to bytes
            }
        }
    }
    return None;
}

#[cfg(target_os = "windows")]
fn get_window_cached_memory() -> Option<u64> {
    use std::mem;
    use winapi::um::psapi::{GetPerformanceInfo, PERFORMANCE_INFORMATION};
    unsafe {
        let mut perf_info: PERFORMANCE_INFORMATION = mem::zeroed();
        perf_info.cb = mem::size_of::<PERFORMANCE_INFORMATION>() as u32;

        if GetPerformanceInfo(&mut perf_info as *mut PERFORMANCE_INFORMATION, perf_info.cb) != 0 {
            let page_size = perf_info.PageSize as u64;
            let cached_pages = perf_info.SystemCache as u64;
            return Some(page_size * cached_pages);
        } else {
            return None;
        }
    }
}
// Get-WmiObject -Class Win32_PerfFormattedData_PerfOS_Memory | Select-Object CacheBytes
// Get-CimInstance -ClassName Win32_OperatingSystem | Select-Object FreePhysicalMemory, TotalVisibleMemorySize, TotalVisibleMemorySize -Property *
//Get-CimInstance -ClassName Win32_PerfFormattedData_PerfOS_Memory | Select-Object CacheBytes
