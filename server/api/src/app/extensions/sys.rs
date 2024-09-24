use chrono::{Duration, NaiveDateTime, Utc};
use sysinfo::{Disks, System};
use tokio::sync::{Mutex, OnceCell};

static SYS: OnceCell<Mutex<System>> = OnceCell::const_new();
static NEXT: OnceCell<Mutex<NaiveDateTime>> = OnceCell::const_new();

async fn init_sys() -> Mutex<System> {
    Mutex::new(System::new())
}

async fn init_time() -> Mutex<NaiveDateTime> {
    Mutex::new(Utc::now().naive_utc())
}

/// get the cpu%, memory_total, memory%, disk_total, disk%
pub async fn fetch_sysinfo(init: bool) -> (u64, f64, u64, f64, u64) {
    let now = Utc::now().naive_utc();
    let mut time = NEXT.get_or_init(init_time).await.lock().await;
    let mut sys = SYS.get_or_init(init_sys).await.lock().await;

    if init || *time + Duration::seconds(10) < now {
        sys.refresh_cpu_all();
        sys.refresh_memory();
        *time = now;
    }

    let cpu_num = sys.cpus().len() as f32;
    let cpu_total: f32 = sys.cpus().iter().map(|c| c.cpu_usage()).sum();
    let p_cpu = (cpu_total / cpu_num) as u64;

    let mem_total = sys.total_memory();
    let mem_used = sys.used_memory();
    let p_mem = mem_used * 100 / mem_total;
    let mem_gb = (mem_total * 100 / 1073741824) as f64 / 100f64; // GB

    let mut disk_total = 0u64;
    let mut disk_used = 0u64;
    let disks = Disks::new_with_refreshed_list();
    for disk in disks.list() {
        disk_total += disk.total_space();
        disk_used += disk.available_space();
    }
    let p_disk = disk_used * 100 / disk_total;
    let disk_gb = (disk_total * 100 / 1073741824) as f64 / 100f64; // GB

    (p_cpu, mem_gb, p_mem, disk_gb, p_disk)
}
