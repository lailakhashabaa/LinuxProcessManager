
use fltk::{prelude::*};
use fltk::{enums::FrameType};
use fltk_table::{SmartTable,TableOpts};
use crate::modules::initial::*;
use crate::modules::table::*;
use crate::modules::tree::*;
use crate::modules::sorting::*;

pub fn kill_tree_trig(table: SmartTable)->()
{

        let pid = table.cell_value(table.callback_row() , table.callback_col()).parse::<u32>().unwrap(); 
        let name = table.cell_value(table.callback_row() , 1);
        println!("{}", name);
         let procdata2 = initialise_vec1();
        for process in procdata2 
                   {
                       if process.pid == pid
                       {
                           let mut procdata3 = initialise_vec1();
                           let procdata4=initialise_vec1();
                           let root =build_baby_tree(&mut procdata3, &procdata4, process.pid, process.name);
                           let proc_kill= &mut procdata3;
                           recursive_kill(root, proc_kill);
                           break;
                       }
       
                   }

}
pub fn kill_btn_trig(table: SmartTable)->()
{
    let pid = table.cell_value(table.callback_row() , table.callback_col()).parse::<u32>().unwrap(); 
    let mut proc = initialise_vec1();
    kill_process(pid, &mut proc);
}
pub fn search_btn_trig(mut table: SmartTable,info:String, pid_s:bool,mem_s:bool,cpu_s:bool, run_s:bool,disk_read_s:bool,disk_write_s:bool,priority_s:bool)->bool
{
    let mut counter = 0;
    
    if info.is_empty() {
        
       
    let mut proc2 = initialise_vec1();
    table = table.clone()
    .with_opts(TableOpts { rows: (proc2.len() as i32), cols: (11), editable: (false), cell_font_size:(8), header_frame: FrameType::FlatBox, ..Default::default() });
    if pid_s == true
    {
        let mut sortedProc = sort_pid(&mut proc2);
        create_table(proc2.len() as i32,&mut sortedProc, &mut table);
    }
    if mem_s == true
    {
        let mut sortedProc = sort_memory_desc(&mut proc2);
        create_table(proc2.len() as i32,&mut sortedProc, &mut table);
    }
    if cpu_s == true
    {
        let mut sortedProc = sort_cpu_usage(&mut proc2);
        create_table(proc2.len() as i32,&mut sortedProc, &mut table);
    }
    if run_s == true
    {
        let mut sortedProc = sort_runtime_desc(&mut proc2);
        create_table(proc2.len() as i32,&mut sortedProc, &mut table);
    }
    if disk_read_s == true
    {
        let mut sortedProc = sort_disk_read(&mut proc2);
        create_table(proc2.len() as i32,&mut sortedProc, &mut table);
    }
    if disk_write_s == true
    {
        let mut sortedProc = sort_disk_write(&mut proc2);
        create_table(proc2.len() as i32,&mut sortedProc, &mut table);
    }
    if priority_s == true
    {
        let mut sortedProc = sort_priority(&mut proc2);
        create_table(proc2.len() as i32,&mut sortedProc, &mut table);
    }
    else
    {
        create_table(proc2.len() as i32,&mut proc2, &mut table);
    }

    
    return false;
    }
    else{
    let proc2 = initialise_vec1();
    for proc in proc2
    {
    if proc.name == info || proc.pid.to_string() == info
    {
    counter = counter + 1;
    } 
    }
    table = table.clone()
    .with_opts(TableOpts { rows: ((counter) as i32), cols: (11), editable: (false), cell_font_size:(8), header_frame: FrameType::FlatBox, ..Default::default() });
    //fill table with data
    table.set_col_header_value(0, "PID");
    table.set_col_header_value(1, "Name");
    table.set_col_header_value(2, "Status");
    table.set_col_header_value(3, "CPU Usage");
    table.set_col_header_value(4, "Memory");
    table.set_col_header_value(5, "run Time");
    table.set_col_header_value(6, "User or System");
    table.set_col_header_value(7, "UID");
    table.set_col_header_value(8, "bytes read");
    table.set_col_header_value(9, "bytes Written");
    table.set_col_header_value(10, "Priority");
    let proc3=initialise_vec1();
    for proc in proc3
    {
    if proc.name == info || proc.pid.to_string() == info
    {
    
    table.set_cell_value((counter - 1).try_into().unwrap(), 0, &proc.pid.to_string());
    table.set_cell_value((counter - 1).try_into().unwrap(), 1, &proc.name);
    table.set_cell_value((counter - 1).try_into().unwrap(), 2, &proc.status);
    table.set_cell_value((counter - 1).try_into().unwrap(), 3, &proc.cpu_usage.to_string());
    table.set_cell_value((counter - 1).try_into().unwrap(), 4, &proc.memory.to_string());
    table.set_cell_value((counter - 1).try_into().unwrap(), 5, &proc.run_time.to_string());
    table.set_cell_value((counter - 1).try_into().unwrap(), 6, &proc.sys_or_user);
    table.set_cell_value((counter - 1).try_into().unwrap(), 7, &proc.uid.to_string());
    table.set_cell_value((counter - 1).try_into().unwrap(), 8, &proc.disk_read_bytes.to_string());
    table.set_cell_value((counter - 1).try_into().unwrap(), 9, &proc.disk_write_bytes.to_string());
    table.set_cell_value((counter - 1).try_into().unwrap(), 10, &proc.priority.to_string());
    counter = counter - 1;
    }
    }
    return true;
    }
    
}


