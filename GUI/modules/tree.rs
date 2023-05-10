use crate::modules::structs::*;
use crate::modules::initial::*;
use std::process::Command;


pub fn build_tree(procdata: &[Proc], procdata2: &[Proc]) -> Treeproc {
    let mut parent = Treeproc{pid: 1, name: "systemd".to_string(), children: Vec::new()};
    set_children(procdata, procdata2, &mut parent);
    return parent;
}
//print process tree
pub fn print_tree(parent: &Treeproc, level: u32) {
    let mut prefix = "".to_string();
    for _ in 0..level {
        prefix += "│  ";
    }

    if level > 0 {
        prefix += "├─";
    }

    println!("{}{} ({})", prefix, parent.name, parent.pid);

    for child in &parent.children {
        print_tree(child, level + 1);
    }
}
pub fn set_children(procdata: &[Proc], procdata2: &[Proc], parent: &mut Treeproc) {
    for proc in procdata {
        if proc.pid == parent.pid {
            let mut children = Vec::new();
            for proc2 in procdata2 {
                if proc2.pid != parent.pid && proc.pid == proc2.ppid {
                    let mut child = Treeproc{pid: proc2.pid, name: proc2.name.clone(), children: Vec::new()};
                    set_children(procdata, procdata2, &mut child);
                    children.push(child);
                }
            }
            parent.children = children;

        }
    }
}
pub fn print_children(parent: &Treeproc, pid: u32) {
    if parent.pid == pid {
        for child in &parent.children {
            println!("{} {}", child.pid, child.name);
            print_children(child, pid);
        }
    } else {
        for child in &parent.children {
            print_children(child, pid);
        }
    }
}
pub fn build_baby_tree(procdata: &[Proc], procdata2: &[Proc], pid: u32, name: String) -> Treeproc
{
    let mut parent = Treeproc{pid: pid, name: name, children: Vec::new()};
    set_children(procdata, procdata2, &mut parent);
    return parent;
}
pub fn recursive_kill(parent:Treeproc, procdata: &mut Vec<Proc>) -> ()
{
    let mut found = false;

    for proc in procdata
    {
        if proc.pid == parent.pid
        {
            found = true;
        }
    }
    
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
pub fn kill_process(pid: u32, procdata: &mut Vec<Proc>) -> ()
{
 let mut found = false;
 for proc in procdata
 {
    if (proc.pid == pid && proc.sys_or_user == "System Process".to_string())
    {
       println!("You can't kill system process");
        found = true;
    }
 
    else if proc.pid == pid 
    {
    println!("{} {} {} {} {} {} {} {} {} {}", proc.pid, proc.name, proc.status, proc.cpu_usage, proc.memory, proc.run_time, proc.sys_or_user, proc.uid, proc.disk_read_bytes, proc.disk_write_bytes);
    found = true;
    let mut command = Command::new("kill");
    command.arg("-9");
    command.arg(pid.to_string());
    let _output = command.output().expect("failed to execute process");
    }
    }
    if found == false
    {
    println!("Process not found");
    }
}
