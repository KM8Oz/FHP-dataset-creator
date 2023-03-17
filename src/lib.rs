use std::io::prelude::*;
use std::sync::Mutex;
use std::{fs::File, os::unix::prelude::PermissionsExt};
use sysinfo::{Pid, ProcessExt, System, SystemExt};

pub async fn get_top_level_parent(pid: Pid) -> Option<Pid> {
    // Create a System object
    let mut s = System::new_all();

    // Refresh system information
    s.refresh_all();

    // Find the process with the given PID
    let process = s.process(pid).unwrap();

    // Traverse the process tree one level higher
    let mut parent = process.parent()?;
    while let Some(grandparent) = s.process(parent) {
        parent = grandparent.parent()?;
        s.refresh_all();
        println!("pid found : {}", parent);
    }
    // Return the top-level parent process
    Some(parent)
}
pub async fn start_monitor(pid: Pid) {
    // Specify the name of the application you want to track
    // Initialize the system object
    let toplevelpid = match get_top_level_parent(pid).await {
        Some(p) => p,
        None => pid
    };
    let mut system = System::new_all();
    let file_path = "data.csv";

    // Check if file exists
    // let file_exists = std::path::Path::new(file_path).exists();
    println!("process pid: {}", toplevelpid);
    // Create or open file
    let mut file = File::create(file_path).expect("Failed to create file");
    let mut perms = file.metadata().unwrap().permissions();
    perms.set_mode(0o777);
    file.set_permissions(perms).unwrap();
    // If file is empty, insert column names
    if file.metadata().unwrap().len() == 0 {
        writeln!(file, "total_read_bytes,total_written_bytes,virtual_memory,cpu_usage").unwrap();
    }

    let file_mutex = Mutex::new(file);

    loop {
        system.refresh_all();
        // Refresh system information

        // Get network activity for the process
        // let network_usage = match mainprocess {
        //     Some(process) => {
        //         let data = _process.parent().;
        //     }
        //     None => {
        //         println!("Process not found");
        //         continue;
        //     }
        // };
        let mainprocess = system.process(toplevelpid);
        // Get disk usage for the process
        let disk_usage = match mainprocess {
            Some(process) => process.disk_usage(),
            None => {
                println!("Process not found");
                continue;
            }
        };

        // Get CPU usage for the process
        let cpu_usage = match mainprocess {
            Some(process) => process.cpu_usage(),
            None => {
                println!("Process not found");
                continue;
            }
        };

        // Get virtual memory usage for the process
        let virtual_memory = match mainprocess {
            Some(process) => process.virtual_memory(),
            None => {
                println!("Process not found");
                continue;
            }
        };
        let mut file_lock = file_mutex.lock().unwrap();
        let data = format!(
            "{},{},{},{}\n",
            disk_usage.total_read_bytes, disk_usage.total_written_bytes, virtual_memory, cpu_usage
        );
        file_lock.write_all(data.as_bytes()).unwrap();
        // Wait for 1 second before logging data again
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
