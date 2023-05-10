use fltk_table::{SmartTable};

use crate::modules::structs::*;
pub fn create_table(rowlen: i32,procdata: &mut Vec<Proc>, table: &mut SmartTable) -> () {
    //change name of columns
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
    //change size of columns
    for i in 0..11
    {
    table.set_col_width(i, 112);
    }
    //fill table with data
    for (i, proc) in procdata.iter().enumerate()
    {
    table.set_cell_value(i.try_into().unwrap(), 0, &proc.pid.to_string());
    table.set_cell_value(i.try_into().unwrap(), 1, &proc.name);
    table.set_cell_value(i.try_into().unwrap(), 2, &proc.status);
    table.set_cell_value(i.try_into().unwrap(), 3, &proc.cpu_usage.to_string());
    table.set_cell_value(i.try_into().unwrap(), 4, &proc.memory.to_string());
    table.set_cell_value(i.try_into().unwrap(), 5, &proc.run_time.to_string());
    table.set_cell_value(i.try_into().unwrap(), 6, &proc.sys_or_user);
    table.set_cell_value(i.try_into().unwrap(), 7, &proc.uid.to_string());
    table.set_cell_value(i.try_into().unwrap(), 8, &proc.disk_read_bytes.to_string());
    table.set_cell_value(i.try_into().unwrap(), 9, &proc.disk_write_bytes.to_string());
    table.set_cell_value(i.try_into().unwrap(), 10, &proc.priority.to_string());
    }
}