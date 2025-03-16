use std::{
    process::Command, sync::mpsc::{Receiver, Sender}, thread, time::{Duration, Instant}
};

use crate::types::{CCpuData, CMemoryData, CSysInfo};
use sysinfo::System;

pub fn spawn_system_info_collector(
    tick_receiver: Receiver<u32>,
    tx: Sender<CSysInfo>,
    default_tick: u32,
) {
    // Spawn a worker thread to gather CPU info
    thread::spawn(move || {
        let mut sys = System::new_all();
        let mut last_refresh = Instant::now();
        let mut tick_value = default_tick; // Current tick in ms

        sys.refresh_all();

        loop {
            let elapsed = last_refresh.elapsed().as_millis() as u32;
            if let Ok(new_tick) = tick_receiver.try_recv() {
                tick_value = new_tick;
            }

            if elapsed >= (tick_value - 2) {
                
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
                let total_memory = ((sys.total_memory() as f64 / 1_073_741_824.0)* 100.0).round() / 100.0;
                let available_memory = ((sys.available_memory() as f64 / 1_073_741_824.0)* 100.0).round() / 100.0;
                let used_memory = ((sys.used_memory() as f64 / 1_073_741_824.0)* 100.0).round() / 100.0;
                let used_swap = ((sys.used_swap() as f64 / 1_073_741_824.0)* 100.0).round() / 100.0;
                let free_memory = ((sys.free_memory() as f64 / 1_073_741_824.0)* 100.0).round() / 100.0;
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
                //    SEND COLLECTION DATA TO MAIN THREAD
                // 
                // -------------------------------------------
                let sys_info = CSysInfo { cpus: cpu_data, memory: memory_data };

                // Send the data to the main thread
                if let Err(e) = tx.send(sys_info) {
                    eprintln!("Failed to send System Info: {}", e);
                    break; // Exit loop if channel is disconnected
                }

                // Reset the last refresh time
                last_refresh = Instant::now();
            }

            // Sleep for a short interval to prevent busy-waiting
            thread::sleep(Duration::from_millis(1));
        }
    });
}

fn get_cached_memory() -> f64 {
    let mut cached_memory = 0.0;
    
    #[cfg(target_os = "macos")]
    {
        let macos_cache = get_macos_cache_memory();
        if let Some(cache) = macos_cache {
            cached_memory = ((cache as f64 / 1_073_741_824.0)* 100.0).round() / 100.0;
        }
    }
    
    return cached_memory
}

fn get_macos_cache_memory() -> Option<u64> {
    let output = Command::new("sh")
        .arg("-c")
        .arg("vm_stat | awk '/File-backed pages/ {print $3 * 16384}'")
        .output()
        .ok()?;

    let cache_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
    cache_str.parse::<u64>().ok()
}