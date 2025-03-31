use std::collections::HashMap;

// the main type structture for the application
pub struct SysInfo {
    pub cpus: Vec<CpuData>,
    pub memory: MemoryData,
    pub disks: HashMap<String, DiskData>,
    pub networks: HashMap<String, NetworkData>,
}

const MAXIMUM_DATA_COLLECTION: usize = 5000;

pub struct CpuData {
    pub info_type: String,
    pub id: String,
    pub brand: String,
    pub usage: f32,
    pub usage_history_vec: Vec<f32>,
}

pub struct MemoryData {
    pub total_memory: f64,
    pub available_memory_vec: Vec<f64>, // available is the combination of free memory, cachedmemory and ready to be reused memory
    pub used_memory_vec: Vec<f64>,
    pub used_swap_vec: Vec<f64>,
    pub free_memory_vec: Vec<f64>, // free means memory that is not used at all
    pub cached_memory_vec: Vec<f64>,
}

pub struct DiskData {
    pub name: String,
    pub total_space: f64,
    pub available_space: f64,
    pub used_space: f64,
    pub bytes_written_vec: Vec<f64>, // Number of written bytes since the last refresh. in KiB
    pub bytes_read_vec: Vec<f64>,    // Number of read bytes since the last refresh. in KiB
    pub file_system: String, // file system used on this disk (so for example: EXT4, NTFS, etc…).
    pub mount_point: String, // mount point of the disk (/ for example). And mount point will also served as the unique identifier for the disk
    pub disk_kind: String,   // kind of disk.( SSD for example )
    pub is_updated: bool, // this was to keep tracked of exsiting disk data we collected was still connected to the system
}

pub struct NetworkData {
    pub interface_name: String,
    pub ip_network: Option<String>,
    pub current_received_vec: Vec<f64>,
    pub current_transmitted_vec: Vec<f64>,
    pub total_received: f64,
    pub total_transmitted: f64,
    pub is_updated: bool,
}

impl CpuData {
    pub fn new(id: i8, brand: String, usage: f32) -> CpuData {
        let id = if id == -1 {
            "CPU-AVG".to_string()
        } else {
            format!("CPU{}", id)
        };
        CpuData {
            info_type: "CPU".to_string(),
            id,
            brand,
            usage,
            usage_history_vec: vec![],
        }
    }

    pub fn update(&mut self, id: i8, usage: f32) {
        let id = if id == -1 {
            "CPU-AVG".to_string()
        } else {
            format!("CPU{}", id)
        };
        if id == self.id {
            self.usage = usage;
            if self.usage_history_vec.len() >= MAXIMUM_DATA_COLLECTION {
                self.usage_history_vec.remove(0);
            }
            self.usage_history_vec.push(usage);
        }
    }
}

impl MemoryData {
    pub fn default() -> MemoryData {
        MemoryData {
            total_memory: -1.0,
            available_memory_vec: vec![0.0],
            used_memory_vec: vec![0.0],
            used_swap_vec: vec![0.0],
            free_memory_vec: vec![0.0],
            cached_memory_vec: vec![0.0],
        }
    }

    pub fn new(
        total: f64,
        available: f64,
        used: f64,
        used_swap: f64,
        free: f64,
        cached: f64,
    ) -> MemoryData {
        return MemoryData {
            total_memory: total,
            available_memory_vec: vec![available],
            used_memory_vec: vec![used],
            used_swap_vec: vec![used_swap],
            free_memory_vec: vec![free],
            cached_memory_vec: vec![cached],
        };
    }

    pub fn update(
        &mut self,
        total: f64,
        available: f64,
        used: f64,
        used_swap: f64,
        free: f64,
        cached: f64,
    ) {
        self.total_memory = total;
        self.available_memory_vec.push(available);
        self.used_memory_vec.push(used);
        self.used_swap_vec.push(used_swap);
        self.free_memory_vec.push(free);
        self.cached_memory_vec.push(cached);

        if self.available_memory_vec.len() > MAXIMUM_DATA_COLLECTION {
            self.available_memory_vec.remove(0);
        }
        if self.used_memory_vec.len() > MAXIMUM_DATA_COLLECTION {
            self.used_memory_vec.remove(0);
        }
        if self.used_swap_vec.len() > MAXIMUM_DATA_COLLECTION {
            self.used_swap_vec.remove(0);
        }
        if self.free_memory_vec.len() > MAXIMUM_DATA_COLLECTION {
            self.free_memory_vec.remove(0);
        }
        if self.cached_memory_vec.len() > MAXIMUM_DATA_COLLECTION {
            self.cached_memory_vec.remove(0);
        }
    }
}

impl DiskData {
    pub fn new(
        name: String,
        total_space: f64,
        available_space: f64,
        used_space: f64,
        bytes_written: f64,
        bytes_read: f64,
        file_system: String,
        mount_point: String,
        kind: String,
    ) -> DiskData {
        DiskData {
            name,
            total_space,
            available_space,
            used_space,
            bytes_written_vec: vec![bytes_written],
            bytes_read_vec: vec![bytes_read],
            file_system,
            mount_point,
            disk_kind: kind,
            is_updated: true,
        }
    }

    pub fn update(
        &mut self,
        name: String,
        total_space: f64,
        available_space: f64,
        used_space: f64,
        bytes_written: f64,
        bytes_read: f64,
        file_system: String,
        mount_point: String,
        kind: String,
    ) {
        if mount_point == self.mount_point {
            self.name = name;
            self.total_space = total_space;
            self.available_space = available_space;
            self.used_space = used_space;
            self.file_system = file_system;
            self.disk_kind = kind;
            self.bytes_written_vec.push(bytes_written);
            self.bytes_read_vec.push(bytes_read);
            if self.bytes_written_vec.len() > MAXIMUM_DATA_COLLECTION {
                self.bytes_written_vec.remove(0);
            }
            if self.bytes_read_vec.len() > MAXIMUM_DATA_COLLECTION {
                self.bytes_read_vec.remove(0);
            }
            self.is_updated = true;
        }
    }
}

impl NetworkData {
    pub fn new(
        interface_name: String,
        ip_network: Option<String>,
        current_received: f64,
        current_transmitted: f64,
        total_received: f64,
        total_transmitted: f64,
    ) -> NetworkData {
        return NetworkData {
            interface_name,
            ip_network,
            current_received_vec: vec![current_received],
            current_transmitted_vec: vec![current_transmitted],
            total_received,
            total_transmitted,
            is_updated: true,
        };
    }

    pub fn update(
        &mut self,
        interface_name: String,
        ip_network: Option<String>,
        current_received: f64,
        current_transmitted: f64,
        total_received: f64,
        total_transmitted: f64,
    ) {
        self.interface_name = interface_name;
        self.ip_network = ip_network;
        self.current_received_vec.push(current_received);
        self.current_transmitted_vec.push(current_transmitted);
        if self.current_received_vec.len() > MAXIMUM_DATA_COLLECTION {
            self.current_received_vec.remove(0);
        }
        if self.current_transmitted_vec.len() > MAXIMUM_DATA_COLLECTION {
            self.current_transmitted_vec.remove(0);
        }
        self.total_received = total_received;
        self.total_transmitted = total_transmitted;
        self.is_updated = true;
    }
}

// the structure of info collected from a seperated thread
// a C infront mean Collected
pub struct CSysInfo {
    pub cpus: Vec<CCpuData>,
    pub memory: CMemoryData,
    pub disks: Vec<CDiskData>,
    pub networks: Vec<CNetworkData>,
}

pub struct CCpuData {
    pub id: i8,
    pub brand: String,
    pub usage: f32,
}

pub struct CMemoryData {
    pub total_memory: f64,
    pub available_memory: f64, // available is the combination of free memory, cached memory and ready to be reused memory
    pub used_memory: f64,
    pub used_swap: f64,
    pub free_memory: f64, // free means memory that is not used at all
    pub cached_memory: f64,
}

pub struct CDiskData {
    pub name: String,
    pub total_space: f64,
    pub available_space: f64,
    pub used_space: f64,
    pub bytes_written: f64, // Number of written bytes since the last refresh. Will be return in KiB
    pub bytes_read: f64,    // Number of read bytes since the last refresh. Will be return in KiB
    pub file_system: String, // file system used on this disk (so for example: EXT4, NTFS, etc…).
    pub mount_point: String, // mount point of the disk (/ for example).
    pub kind: String,       // kind of disk.( SSD for example )
}

pub struct CNetworkData {
    pub interface_name: String,
    pub ip_network: Option<String>,
    pub current_received: f64,
    pub total_received: f64,
    pub current_transmitted: f64,
    pub total_transmitted: f64,
}
