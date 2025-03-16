use std::{
    sync::mpsc::{Receiver, Sender},
    thread,
    time::{Duration, Instant},
};

use crate::types::{CCpuData, CSysInfo};
use sysinfo::System;

pub fn spawn_cpu_info_collector(
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

            if elapsed >= tick_value {
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

                let sys_info = CSysInfo { cpus: cpu_data };

                // Send the data to the main thread
                if let Err(e) = tx.send(sys_info) {
                    eprintln!("Failed to send System Info: {}", e);
                    break; // Exit loop if channel is disconnected
                }

                // Reset the last refresh time
                last_refresh = Instant::now();
            }

            // Sleep for a short interval to prevent busy-waiting
            thread::sleep(Duration::from_millis(25));
        }
    });
}
