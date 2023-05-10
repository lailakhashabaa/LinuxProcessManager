
use sysinfo::{ProcessExt, System, SystemExt, PidExt};
use libc::getpriority;
use procfs::process::Process as ProcfsProcess;
use std::convert::TryInto;
use crate::modules::structs::*;



pub fn initialise_vec1()->Vec<Proc>
{

    
    let  mut sys = System::new_all();
    sys.refresh_all();
    // constrcting the struct
    let mut procdata= Vec::new();
    
    for (pid,proc) in sys.processes()
    {
        let mod_pid = pid.as_u32().try_into().unwrap();
        let process_for_uid = ProcfsProcess::new(mod_pid).unwrap();
        let uid = process_for_uid.uid();
        let _uid_copy = process_for_uid.uid();
        let uid_copy2 = process_for_uid.uid();
        let sys_or_user;
        if uid.unwrap() == 0
            {
                sys_or_user = "System Process".to_string();
            }
            else
            {
                sys_or_user =  "User Process".to_string();
            }
            let pr;
            unsafe{pr = getpriority(libc::PRIO_PROCESS, pid.as_u32() as libc::id_t);}
            procdata.push(Proc{
            pid: pid.as_u32(),
            ppid: proc.parent().map(|parent| parent.as_u32()).unwrap_or(0),
            name: proc.name().to_string(),
            status: proc.status().to_string(),
            cpu_usage: proc.cpu_usage(),
            memory: proc.memory(),
            run_time: proc.run_time(),
            sys_or_user: sys_or_user,
            uid: uid_copy2.unwrap(),
            disk_read_bytes: proc.disk_usage().total_read_bytes,
            disk_write_bytes: proc.disk_usage().total_written_bytes,
            priority: pr,

            });

           
    }
    return procdata;
}
pub fn initialise_vec2()->Vec<Proc2>
{
    let mut procdata2 =Vec::new();
    let  mut sys = System::new_all();
    sys.refresh_all();
    for (pid,proc) in sys.processes()
    {
    procdata2.push(Proc2{
        pid: pid.as_u32(),
        ppid: proc.parent().map(|parent| parent.as_u32()).unwrap_or(0),
        name: proc.name().to_string(),
            
        });
    }
    return procdata2;
}

