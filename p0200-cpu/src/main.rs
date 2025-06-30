use std::thread;
use std::time::Duration;
use sysinfo::System;

fn main() {
    let mut system = System::new_all();

    // 初回更新
    system.refresh_cpu();

    // CPUの負荷を正確に測定するため、少し待機
    thread::sleep(Duration::from_millis(200));

    // 再度更新
    system.refresh_cpu();

    let cpus = system.cpus();
    println!("CPU数: {}", cpus.len());
    println!();

    for (i, cpu) in cpus.iter().enumerate() {
        println!("CPU {}: {:.1}%", i, cpu.cpu_usage());
    }
}
