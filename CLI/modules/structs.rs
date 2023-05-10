#[derive(Clone)]
pub struct Proc{
    pub pid: u32,
    pub ppid: u32,
    pub name: String,
    pub status: String,
    pub cpu_usage: f32,
    pub memory: u64,
    pub run_time: u64,
    pub sys_or_user: String,
    pub uid: u32,
    pub disk_read_bytes: u64,
    pub disk_write_bytes: u64,
    pub priority: i32,
}

pub struct Proc2{
    pub pid: u32,
    pub ppid: u32,
    pub name: String,
}

pub struct TreeProc{
    pub pid: u32,
    pub name: String,
    pub children: Vec<TreeProc>,
}