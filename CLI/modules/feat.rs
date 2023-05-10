
use prettytable::{Table, row};
use libc::getpriority;
use libc::setpriority;
use sysinfo::{ProcessExt, System, SystemExt};
use std::process::Command;
use crate:: modules:: structs::*;


use super::initial::initialise_vec1;

pub fn get_processes_by_uid(procdata: &mut Vec<Proc>, uid: u32) -> ()
{
    let mut table = Table::new();
    table.add_row(row!["PID", "Name", "Status", "CPU Usage", "Memory", "Run Time", "User or System", "UID", "Disk Read Bytes", "Disk Written Bytes"]);
    for proc in procdata
    {
        if proc.uid == uid
        {
            table.add_row(row![proc.pid, proc.name, proc.status, proc.cpu_usage, proc.memory, proc.run_time, proc.sys_or_user, proc.uid, proc.disk_read_bytes, proc.disk_write_bytes]);
        }
    }
    table.printstd();
}
//get disk usage of each process in procdata
pub fn disk_usage(_procdata: &mut Vec<Proc>) -> ()
{
    let  mut sys = System::new_all();
    sys.refresh_all();
    let _table = Table::new();
    for (_pid,proc) in sys.processes()
    {
       println!("{}\n", proc.disk_usage().total_read_bytes);
    }
}
//set priority of a process using setpriority
pub unsafe fn set_priority(pid: u32, priority: i32) -> ()
{
    let prioity_before = getpriority(libc::PRIO_PROCESS, pid as libc::id_t);
    setpriority(libc::PRIO_PROCESS, pid as libc::id_t, priority);
    let prioity_after = getpriority(libc::PRIO_PROCESS, pid as libc::id_t);
    println!("Priority before: {}\n", prioity_before);
    println!("Priority after: {}\n", prioity_after);
}
//save process data in a csv file
pub fn save_csv(procdata: &mut Vec<Proc>) -> ()
{
    let mut wtr = csv::Writer::from_path("processes.csv").unwrap();
    wtr.write_record(&["PID", "Name", "Status", "CPU Usage", "Memory", "Run Time", "System or User Process", "UID", "Disk Read Bytes", "Disk Written Bytes"]).unwrap();
    for proc in procdata
    {
        wtr.write_record(&[proc.pid.to_string(), proc.name.to_string(), proc.status.to_string(), proc.cpu_usage.to_string(), proc.memory.to_string(), proc.run_time.to_string(), proc.sys_or_user.to_string(), proc.uid.to_string(), proc.disk_read_bytes.to_string(), proc.disk_write_bytes.to_string()]).unwrap();
    }
    wtr.flush().unwrap();
}
 // search function to search the process by name
pub fn search_name(procdata: &mut Vec<Proc>, search: &str) -> ()
{
    let mut found = false;
    let mut table = Table::new();
    table.add_row(row!["PID", "Name", "Status", "CPU Usage", "Memory", "Run Time", "User or System", "UID", "Disk Read Bytes", "Disk Written Bytes", "Priority"]);
    for proc in procdata
    {
        if proc.name == search
        {
            found = true;
            table.add_row( row![ proc.pid, proc.name, proc.status, proc.cpu_usage, proc.memory, proc.run_time, proc.sys_or_user, proc.uid, proc.disk_read_bytes, proc.disk_write_bytes, proc.priority]); 
            
        }
    }
    if found == false
    {
        println!("Process not found");
    }
    else {
        table.printstd();
    }
    
          

}

pub fn kill_process(pid: u32, procdata: &mut Vec<Proc>) -> ()
{
    
    for proc in procdata
    {
        if proc.pid == pid
        {
            println!("{} {} {} {} {} {} {} {} {} {}", proc.pid, proc.name, proc.status, proc.cpu_usage, proc.memory, proc.run_time, proc.sys_or_user, proc.uid, proc.disk_read_bytes, proc.disk_write_bytes);
            let mut command = Command::new("kill");
            command.arg("-9");
            command.arg(pid.to_string());
            let _output = command.output().expect("failed to execute process");
           
        }
    }
   
}
// function to kill root and children processes given root pid as an input
pub fn recursive_kill(parent:TreeProc, procdata: &mut Vec<Proc>) -> ()
{
    let mut found = false;

    for proc in procdata
    {
        if proc.pid == parent.pid
        {
            found = true;
        }
    }
    if found == false
    {
        return;
    }
    else
    {
        if parent.children.len() == 0
        {
           
            let mut command = Command::new("kill");
            command.arg("-9");
            command.arg(parent.pid.to_string());
            let _output = command.output().expect("failed to execute process");
          
        }
        else {
            for child in parent.children
        {
            
            let mut data= initialise_vec1();
            recursive_kill(child, &mut data);
            let mut command = Command::new("kill");
            command.arg("-9");
            command.arg(parent.pid.to_string());
            let _output = command.output().expect("failed to execute process");
            

        }
    }
        
}

}
