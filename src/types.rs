// the main type structture for the application
pub struct SysInfo {
    pub cpus: Vec<CpuData>,
    pub memory: MemoryData,
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

// the structure of info collected from a seperated thread
// a C infront mean Collected
pub struct CSysInfo {
    pub cpus: Vec<CCpuData>,
    pub memory: CMemoryData,
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
    pub free_memory: f64,   // free means memory that is not used at all
    pub cached_memory: f64, 
}
