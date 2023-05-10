use crate::modules::structs::*;
pub fn sort_pid(procdata: &mut Vec<Proc>) -> Vec<Proc>
{
 procdata.sort_by(|a, b| a.pid.cmp(&b.pid));
 return procdata.to_vec();
}
//sort function to sort procdata according to cpu usage and retrun the sorted data
pub fn sort_cpu(procdata: &mut Vec<Proc>) -> Vec<Proc>
{
 procdata.sort_by(|a, b| a.cpu_usage.partial_cmp(&b.cpu_usage).unwrap());
 return procdata.to_vec();
}
//sort function to sort procdata according to memory usage and retrun the sorted data
pub fn sort_memory(procdata: &mut Vec<Proc>) -> Vec<Proc>
{
 procdata.sort_by(|a, b| a.memory.partial_cmp(&b.memory).unwrap());
 return procdata.to_vec();
}
//sort function to sort procdata according to run time and retrun the sorted data
pub fn sort_run_time(procdata: &mut Vec<Proc>) -> Vec<Proc>
{
 procdata.sort_by(|a, b| a.run_time.partial_cmp(&b.run_time).unwrap());
 return procdata.to_vec();
}
//sort function to sort procdata according to disk read bytes and retrun the sorted data
pub fn sort_disk_read(procdata: &mut Vec<Proc>) -> Vec<Proc>
{
 procdata.sort_by(|a, b| b.disk_read_bytes.partial_cmp(&a.disk_read_bytes).unwrap());
 return procdata.to_vec();
}
//sort function to sort procdata according to disk write bytes and retrun the sorted data
pub fn sort_disk_write(procdata: &mut Vec<Proc>) -> Vec<Proc>
{
 procdata.sort_by(|a, b| b.disk_write_bytes.partial_cmp(&a.disk_write_bytes).unwrap());
 return procdata.to_vec();
}

//sort memory by desending order
pub fn sort_memory_desc(procdata: &mut Vec<Proc>) -> Vec<Proc>
{
 procdata.sort_by(|a, b| b.memory.partial_cmp(&a.memory).unwrap());
 return procdata.to_vec();
}

//sort run time by descending order
pub fn sort_runtime_desc(procdata: &mut Vec<Proc>) -> Vec<Proc>
{
 procdata.sort_by(|a, b| b.run_time.partial_cmp(&a.run_time).unwrap());
 return procdata.to_vec();
}
//sort cpu usage by descending order
pub fn sort_cpu_usage(procdata: &mut Vec<Proc>) -> Vec<Proc>
{
 procdata.sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap());
 return procdata.to_vec();
}
//sort by priority
pub fn sort_priority(procdata: &mut Vec<Proc>) -> Vec<Proc>
{
 procdata.sort_by(|a, b| a.priority.partial_cmp(&b.priority).unwrap());
 return procdata.to_vec();
}
