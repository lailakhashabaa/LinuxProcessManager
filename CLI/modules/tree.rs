use crate::modules::structs::*;

pub fn build_tree(procdata: &[Proc], procdata2: &[Proc]) -> TreeProc {
    let mut parent = TreeProc{pid: 1, name: "systemd".to_string(), children: Vec::new()};
    set_children(procdata, procdata2, &mut parent);
    return parent;
}
pub fn print_tree(parent: &TreeProc, level: u32) {
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
pub fn set_children(procdata: &[Proc], procdata2: &[Proc], parent: &mut TreeProc) {
    for proc in procdata {
        if proc.pid == parent.pid {
            let mut children = Vec::new(); // create a new vector of children processes
            for proc2 in procdata2 {
                if proc2.pid != parent.pid && proc.pid == proc2.ppid {
                    let mut child = TreeProc{pid: proc2.pid, name: proc2.name.clone(), children: Vec::new()};
                    set_children(procdata, procdata2, &mut child);
                    children.push(child); // add the child process to the children vector
                }
            }
            parent.children = children; // set the children vector to the parent node

        }
    }
}
pub fn print_children(parent: &TreeProc, pid: u32) {
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
pub fn build_baby_tree(procdata: &[Proc], procdata2: &[Proc], pid: u32, name: String) -> TreeProc
{
    let mut parent = TreeProc{pid: pid, name: name, children: Vec::new()};
    set_children(procdata, procdata2, &mut parent);
    return parent;
}
