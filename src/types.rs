use std::collections::HashMap;

// the main type structture for the application
pub struct SysInfo {
    pub cpus: Vec<CpuData>,
    pub memory: MemoryData,
    pub disks: HashMap<String, DiskData>,
}

const MAXIMUM_DATA_COLLECTION: usize = 10000;

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
    pub total_written_bytes: f64,    // Total number of written bytes.
    pub written_bytes_vec: Vec<f64>, // Number of written bytes since the last refresh. in MB with 3 decimal places
    pub total_read_bytes: f64,       // Total number of read bytes.
    pub read_bytes_vec: Vec<f64>, // Number of read bytes since the last refresh. in MB with 3 decimal places
    pub file_system: String, // file system used on this disk (so for example: EXT4, NTFS, etc…).
    pub mount_point: String, // mount point of the disk (/ for example). And mount point will also served as the unique identifier for the disk
    pub kind: String,        // kind of disk.( SSD for example )

    // following info will not be shown in ui
    pub last_written_bytes: f64, // in MB with 3 decimal places
    pub last_read_bytes: f64,    // in MB with 3 decimal places
    pub is_updated: bool, // this was to keep tracked of exsiting disk data we collected was still connected to the system
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
        total_written_bytes: f64,
        written_bytes: f64,
        total_read_bytes: f64,
        read_bytes: f64,
        file_system: String,
        mount_point: String,
        kind: String,
    ) -> DiskData {
        DiskData {
            name,
            total_space,
            available_space,
            used_space,
            total_written_bytes,
            written_bytes_vec: vec![],
            total_read_bytes,
            read_bytes_vec: vec![],
            file_system,
            mount_point,
            kind,
            last_written_bytes: written_bytes,
            last_read_bytes: read_bytes,
            is_updated: true,
        }
    }

    pub fn update(
        &mut self,
        name: String,
        total_space: f64,
        available_space: f64,
        used_space: f64,
        total_written_bytes: f64,
        written_bytes: f64,
        total_read_bytes: f64,
        read_bytes: f64,
        file_system: String,
        mount_point: String,
        kind: String,
    ) {
        if mount_point == self.mount_point {
            self.name = name;
            self.total_space = total_space;
            self.available_space = available_space;
            self.used_space = used_space;
            self.total_written_bytes = total_written_bytes;
            self.total_read_bytes = total_read_bytes;
            self.file_system = file_system;
            self.kind = kind;
            let actual_written_byte = written_bytes - self.last_written_bytes;
            let actual_read_byte = read_bytes - self.last_read_bytes;
            self.written_bytes_vec.push(actual_written_byte.min(0.0));
            self.read_bytes_vec.push(actual_read_byte.min(0.0));
            if self.written_bytes_vec.len() > MAXIMUM_DATA_COLLECTION {
                self.written_bytes_vec.remove(0);
            }
            if self.read_bytes_vec.len() > MAXIMUM_DATA_COLLECTION {
                self.read_bytes_vec.remove(0);
            }
            self.last_written_bytes = written_bytes;
            self.last_read_bytes = read_bytes;
            self.is_updated = true;
        }
    }
}

// the structure of info collected from a seperated thread
// a C infront mean Collected
pub struct CSysInfo {
    pub cpus: Vec<CCpuData>,
    pub memory: CMemoryData,
    pub disks: Vec<CDiskData>,
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
    pub total_written_bytes: f64, // Total number of written bytes.
    pub written_bytes: f64, // Number of written bytes since the last refresh. Will be return in MB with 3 decimal places
    pub total_read_bytes: f64, // Total number of read bytes.
    pub read_bytes: f64, // Number of read bytes since the last refresh. Will be return in MB with 3 decimal places
    pub file_system: String, // file system used on this disk (so for example: EXT4, NTFS, etc…).
    pub mount_point: String, // mount point of the disk (/ for example).
    pub kind: String,    // kind of disk.( SSD for example )
}
