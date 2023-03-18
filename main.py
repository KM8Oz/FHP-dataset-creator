import time
import psutil
import csv

class FHPDataset():
    app_path = ''
    def __init__(self, name) -> None:
        self.app_path =  name
    def getProcess(self):
        for proc in psutil.process_iter(['cmdline']):
            try:
                for s in proc.cmdline():
                    if s.__contains__(self.app_path):
                        return proc
            except (psutil.NoSuchProcess, psutil.AccessDenied):
                pass
        return None
    def start_monitor(self, filename:str,process:psutil.Process, maxrows=500):
        with open(f'{filename}.csv', 'w', newline='') as csvfile:
            rownumber = 0
            # Continuously monitor and log the process usage
            writer = csv.writer(csvfile)
            # Write the header row
            writer.writerow(['cpu_percent', 'threads_number', 'memory_usage_vms', 'memory_usage_rss'])
            while True:
                try:
                    # Get the process object by PID
                    # process = psutil.Process(pid)
                    # Get the process CPU usage as a percentage
                    cpu_percent = process.cpu_percent()

                    # Get the process disk usage as a tuple of (total, used, free) bytes
                    number_threads = process.num_threads()

                    # vms: aka “Virtual Memory Size”, this is the total amount of virtual memory used by the process. On UNIX it matches “top“‘s VIRT column. On Windows this is an alias for pagefile field and it matches “Mem Usage” “VM Size” column of taskmgr.exe.
                    memory_usage_vms:tuple = process.memory_info().vms, 
                    
                    # rss: aka “Resident Set Size”, this is the non-swapped physical memory a process has used. On UNIX it matches “top“‘s RES column). On Windows this is an alias for wset field and it matches “Mem Usage” column of taskmgr.exe.
                    memory_usage_rss = process.memory_info().rss

                    # Write the data row to the CSV file
                    writer.writerow([cpu_percent, number_threads, memory_usage_vms[0], memory_usage_rss])

                    # Wait for a few seconds before the next iteration
                    time.sleep(1)
                    if rownumber >= maxrows:
                        return None
                    else:
                        rownumber = rownumber + 1
                    
                except psutil.NoSuchProcess:
                    # If the process is not found, break the loop
                    pass
if __name__ == "__main__":
    fhp = FHPDataset("/Applications/Visual Studio Code.app/Contents/MacOS/Electron")
    process  = fhp.getProcess()
    if process:
        print(f"PID: {process.pid}")
        fhp.start_monitor("data", process, 600)