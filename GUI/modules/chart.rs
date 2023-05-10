use rand::Rng;
use fltk::misc::Chart;
use fltk::{prelude::*,*};



use crate::modules::structs::*;
use crate::modules::sorting::*;
pub fn chart_create( procdata:&mut Vec<Proc>, flag: i32, chart: &mut Chart) -> (){
    //for creating random colors
    let mut r = rand::thread_rng();
    let mut g = rand::thread_rng();
    let mut b = rand::thread_rng();
    let mut proc_chart = Vec::new();
    if flag == 0{
        proc_chart = sort_cpu_usage(procdata);
        chart.set_type(misc::ChartType::Pie);
    }
    else if flag == 1{
        proc_chart = sort_memory_desc(procdata);
        chart.set_type(misc::ChartType::Bar);
    }
    else if flag == 2{
        proc_chart = sort_runtime_desc(procdata);
        chart.set_type(misc::ChartType::Pie);
    }
    for (i, proc) in proc_chart.iter().enumerate()
    {
    if i < 7
    {
    let red: u8 = r.gen_range(0..255);
    let green = g.gen_range(0..255);
    let blue = b.gen_range(0..255);
    let color =  enums::Color::from_rgb(red, green, blue);

    if flag == 0{
        chart.add(proc.cpu_usage as f64, &proc.pid.to_string(),  color);
    }
    else if flag == 1{
        chart.add(proc.memory as f64, &proc.pid.to_string(),  color);
    }
    else if flag == 2{
        chart.add(proc.run_time as f64, &proc.pid.to_string(),  color);
    }
    }
    
    }
}