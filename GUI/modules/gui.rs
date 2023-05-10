use crate::modules::sorting::*;
use crate::modules::initial::*;
use crate::modules::table::*;
use crate::modules::chart::*;
use crate::modules::buttons::*;

use sysinfo::{ System, SystemExt};
use std::convert::TryInto;
use fltk_evented::Listener;
use fltk::{prelude::*,*};
use fltk::{app, enums::FrameType};
use fltk_table::{SmartTable,TableOpts};
use std::time::{Instant};

pub fn initialize_gui()-> app::App {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
 app::background(221, 221,221);
 let mut wind = window::Window::new(100, 100, 1400, 1000, "THEone").center_screen();

 let mut sys = System::new_all();
 sys.refresh_all();
 // constrcting the struct
 //initialize two vectors 
 
 let mut procdata;
 procdata = initialise_vec1();
 let mut  procdata2;
 procdata2 =initialise_vec2();

 let mut start = Instant::now();
 //creating window inside window
 let mut choice:Listener <_> = menu::Choice::new(800, 10, 150, 50, "Sort Table by").into();
 choice.add_choice("PID | CPU Usage | Memory | run Time | Disk read bytes | Disk Written bytes | Priority");
 choice.set_value(7);
 choice.set_color(enums::Color::White);
 choice.set_text_size(12);

 let mut choice2:Listener <_> = menu::Choice::new(500, 10, 150, 50, "Show on Chart").into();
 choice2.add_choice("CPU Usage | Memory | run Time");
 choice2.set_value(3);
 choice2.set_color(enums::Color::White);
 choice2.set_text_size(12);

 let kill_btn: Listener <_> = button::Button::new(100, 10, 100, 50, "Kill Process").into();
 let kill_tree: Listener <_> = button::Button::new(250, 10, 100, 50, "Kill Branch").into();
 let search_input: Listener <_> = input::Input::new(1050, 10, 150, 50, "Search").into();
 let search_btn: Listener <_> = button::Button::new(1208, 10, 70, 50, "Search").into();



 let mut wind2 = window::Window::new(20, 70, wind.w() - 50, 475, None);
 wind2.set_color(enums::Color::White);
 wind2.set_frame(enums::FrameType::BorderBox);
 let mut table = SmartTable::default().with_size(wind2.w()-50, 425).with_pos(25, 20)
 .with_opts(TableOpts { rows: (procdata.len() as i32), cols: (11), editable: (false), cell_font_size:(8), header_frame: FrameType::FlatBox, ..Default::default() });
 create_table(procdata.len() as i32,&mut procdata, &mut table);
 wind2.end();
 
 //creating chart for cpu usage
 let mut chart = misc::Chart::default().with_size(1350, 430).with_pos(20, 550);
 chart.set_type(misc::ChartType::Pie);
 chart.set_bounds(0.0, 100.0);
 chart.set_text_size(16);
 chart.set_color(enums::Color::White);


 //fill chart with data
 let mut proc_chart = sort_cpu_usage(&mut procdata);
 chart_create(&mut proc_chart,0, &mut chart);

 wind.make_resizable(true);
 wind.end();
 wind.show();

 let mut cpu = true;
 let mut mem = false;
 let mut run = false;

 let mut pid_s = false;
 let mut cpu_s = false;
 let mut run_s = false;
 let mut mem_s = false;
 let mut disk_read_s = false;
 let mut disk_write_s = false;
 let mut priority_s = false;
 let mut searching = false;
 let mut search_info = String::new();
 
 while app.wait() 
 {
    
    if start.elapsed().as_secs() >=5
    {
      
      start = Instant::now();
      procdata.clear();
      procdata2.clear();
      sys.refresh_all();
      procdata=initialise_vec1();
      procdata2=initialise_vec2();
      table = table.clone()
      .with_opts(TableOpts { rows: (procdata.len() as i32), cols: (11), editable: (false), cell_font_size:(8), header_frame: FrameType::FlatBox, ..Default::default() });
      create_table(procdata.len() as i32,&mut procdata, &mut table);
      
      if pid_s == true{
        let mut proc_sort = &mut procdata;
        let mut sorted_proc = sort_pid(&mut proc_sort);
        table = table.clone()
        .with_opts(TableOpts { rows: (procdata.len() as i32), cols: (11), editable: (false), cell_font_size:(8), header_frame: FrameType::FlatBox, ..Default::default() });
        
        create_table(procdata.len() as i32,&mut sorted_proc,&mut table);
      }
      else if cpu_s == true{
        let mut proc_sort = &mut procdata;
        let mut sorted_proc = sort_cpu_usage(&mut proc_sort);
        table = table.clone()
        .with_opts(TableOpts { rows: (procdata.len() as i32), cols: (11), editable: (false), cell_font_size:(8), header_frame: FrameType::FlatBox, ..Default::default() });
        
        create_table(procdata.len() as i32,&mut sorted_proc, &mut table);
      }
      else if mem_s == true{
        let mut proc_sort = &mut procdata;
        let mut sorted_proc = sort_memory_desc(&mut proc_sort);
        table = table.clone()
        .with_opts(TableOpts { rows: (procdata.len() as i32), cols: (11), editable: (false), cell_font_size:(8), header_frame: FrameType::FlatBox, ..Default::default() });
        
        create_table(procdata.len() as i32,&mut sorted_proc, &mut table);
      }
      else if run_s == true{
        let mut proc_sort = &mut procdata;
        let mut sorted_proc = sort_runtime_desc(&mut proc_sort);
        table = table
        .with_opts(TableOpts { rows: (procdata.len() as i32), cols: (11), editable: (false), cell_font_size:(8), header_frame: FrameType::FlatBox, ..Default::default() });
        
        create_table(procdata.len() as i32,&mut sorted_proc, &mut table);
      }
      else if disk_read_s == true{
        let mut proc_sort = &mut procdata;
        let mut sorted_proc = sort_disk_read(&mut proc_sort);
        table = table.clone()
        .with_opts(TableOpts { rows: (procdata.len() as i32), cols: (11), editable: (false), cell_font_size:(8), header_frame: FrameType::FlatBox, ..Default::default() });
        
        create_table(procdata.len() as i32,&mut sorted_proc, &mut table);
      }
      else if disk_write_s == true {
        let mut proc_sort = &mut procdata;
        let mut sorted_proc = sort_disk_write(&mut proc_sort);
        table = table.clone()
        .with_opts(TableOpts { rows: (procdata.len() as i32), cols: (11), editable: (false), cell_font_size:(8), header_frame: FrameType::FlatBox, ..Default::default() });
      
        create_table(procdata.len() as i32,&mut sorted_proc, &mut table);
      }
      else if priority_s == true{
        let mut proc_sort = &mut procdata;
        let mut sorted_proc = sort_priority(&mut proc_sort);

        table = table.clone()
        .with_opts(TableOpts { rows: (procdata.len() as i32), cols: (11), editable: (false), cell_font_size:(8), header_frame: FrameType::FlatBox, ..Default::default() });
      
        create_table(procdata.len() as i32,&mut sorted_proc, &mut table);
      }
      else {
        table = table.clone()
        .with_opts(TableOpts { rows: (procdata.len() as i32), cols: (11), editable: (false), cell_font_size:(8), header_frame: FrameType::FlatBox, ..Default::default() });
      
        create_table(procdata.len() as i32,&mut procdata, &mut table);
      }
      if searching{
        let mut counter = 0;
        let proc2 = &mut procdata;
        for proc in proc2
        {
          if proc.name == search_info || proc.pid.to_string() == search_info
          {
            counter = counter + 1;
          } 
        }
        table = table
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
        let proc3= initialise_vec1();
        for proc in proc3
        {
          if proc.name == search_info || proc.pid.to_string() == search_info
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
      }
      //creating chart for cpu usage
      chart.clear();
      //let mut chart = misc::Chart::default().with_size(1150, 430).with_pos(20, 550);
      if cpu == true {
      chart_create(&mut procdata,0, &mut chart);
      }
      else if mem == true {
        chart_create(&mut procdata,1, &mut chart);
      }

      else if run == true {
        chart_create(&mut procdata,2, &mut chart);
      }
      } 
      if kill_btn.triggered(){
        kill_btn_trig(table.clone());
        }
      if kill_tree.triggered()
      {
        kill_tree_trig(table.clone());
      }
      if search_btn.triggered()
      {
        let info = search_input.value().parse::<String>().unwrap();
        let searchbool=search_btn_trig(table.clone(),info.clone(),pid_s,mem_s,cpu_s, run_s,disk_read_s,disk_write_s,priority_s);
        if searchbool == true
        {
            search_info = info;
            searching = true;
        }
          else
        {
          search_info = info;
          searching = false;
        }
        }
      
      if choice.triggered(){
      match choice.value()
      {
      0 => 
      {
        pid_s = true;
        cpu_s = false;
        mem_s = false;
        run_s = false;
        disk_read_s = false;
        disk_write_s = false;
        priority_s = false;
        searching = false;
        let mut proc_sort = &mut procdata;
        let mut sorted_proc = sort_pid(&mut proc_sort);
        table = table.clone()
        .with_opts(TableOpts { rows: (procdata.len() as i32), cols: (11), editable: (false), cell_font_size:(8), header_frame: FrameType::FlatBox, ..Default::default() });
        
        create_table(procdata.len() as i32,&mut sorted_proc, &mut table);
      },

      1 =>
      {
        pid_s = false;
        cpu_s = true;
        mem_s = false;
        run_s = false;
        disk_read_s = false;
        disk_write_s = false;
        priority_s = false;
        searching = false;
        let mut proc_sort = &mut procdata;
        let mut sorted_proc = sort_cpu_usage(&mut proc_sort);
        table = table.clone()
        .with_opts(TableOpts { rows: (procdata.len() as i32), cols: (11), editable: (false), cell_font_size:(8), header_frame: FrameType::FlatBox, ..Default::default() });
        
        create_table(procdata.len() as i32,&mut sorted_proc, &mut table);
      },
      2 =>
      {
        pid_s = false;
        cpu_s = false;
        mem_s = true;
        run_s = false;
        disk_read_s = false;
        disk_write_s = false;
        priority_s = false;
        searching = false;
        let mut proc_sort = &mut procdata;
        let mut sorted_proc = sort_memory_desc(&mut proc_sort);
        table = table.clone()
        .with_opts(TableOpts { rows: (procdata.len() as i32), cols: (11), editable: (false), cell_font_size:(8), header_frame: FrameType::FlatBox, ..Default::default() });
        
        create_table(procdata.len() as i32,&mut sorted_proc, &mut table);
      },
      3 => 
      {
        pid_s = false;
        cpu_s = false;
        mem_s = false;
        run_s = true;
        disk_read_s = false;
        disk_write_s = false;
        priority_s = false;
        searching = false;
        let mut proc_sort = &mut procdata;
        let mut sorted_proc = sort_runtime_desc(&mut proc_sort);
        table = table.clone()
        .with_opts(TableOpts { rows: (procdata.len() as i32), cols: (11), editable: (false), cell_font_size:(8), header_frame: FrameType::FlatBox, ..Default::default() });
        
        create_table(procdata.len() as i32,&mut sorted_proc, &mut table);
      },
      4 => 
      {
        pid_s = false;
        cpu_s = false;
        mem_s = false;
        run_s = false;
        disk_read_s = true;
        disk_write_s = false;
        priority_s = false;
        searching = false;
        let mut proc_sort = &mut procdata;
        let mut sorted_proc = sort_disk_read(&mut proc_sort);
        table = table.clone()
        .with_opts(TableOpts { rows: (procdata.len() as i32), cols: (11), editable: (false), cell_font_size:(8), header_frame: FrameType::FlatBox, ..Default::default() });
        
        create_table(procdata.len() as i32,&mut sorted_proc, &mut table);
      },
      5 => {
        pid_s = false;
        cpu_s = false;
        mem_s = false;
        run_s = false;
        disk_read_s = false;
        disk_write_s = true;
        priority_s = false;
        searching = false;
        let mut proc_sort = &mut procdata;
        let mut sorted_proc = sort_disk_write(&mut proc_sort);
        table = table.clone()
        .with_opts(TableOpts { rows: (procdata.len() as i32), cols: (11), editable: (false), cell_font_size:(8), header_frame: FrameType::FlatBox, ..Default::default() });
        
        create_table(procdata.len() as i32,&mut sorted_proc, &mut table);
      },
      6=>
      {
        pid_s = false;
        cpu_s = false;
        mem_s = false;
        run_s = false;
        disk_read_s = false;
        disk_write_s = false;
        priority_s = true;
        searching = false;
        let mut proc_sort = &mut procdata;
        let mut sorted_proc = sort_priority(&mut proc_sort);
        table = table.clone()
        .with_opts(TableOpts { rows: (procdata.len() as i32), cols: (11), editable: (false), cell_font_size:(8), header_frame: FrameType::FlatBox, ..Default::default() });
        
        create_table(procdata.len() as i32,&mut sorted_proc, &mut table);
      },
      _ => println!("Invalid choice"), 
      }
      }
      if choice2.triggered()
      {
        match choice2.value()
        {
          0 => {
          cpu = true;
          mem = false;
          run = false;
          chart.clear();
          let mut proc_chart = sort_cpu_usage(&mut procdata);
          chart_create(&mut proc_chart,0, &mut chart);
          },
          1 => {
          cpu = false;
          mem = true;
          run = false;
          let mut proc_chart = sort_memory_desc(&mut procdata);
          chart.clear();
          chart_create(&mut proc_chart,1, &mut chart);
          },
          2 => {
          cpu = false;
          mem = false;
          run = true;
          let mut proc_chart = sort_runtime_desc(&mut procdata);
          chart.clear();
            chart_create(&mut proc_chart,2, &mut chart);
          },
          _ => println!("Invalid choice"),
        }
    }
    wind.set_callback(move |_| 
    {
      if app::event() == enums::Event::Close 
      {
        app::quit();
      }
    });
 }

 app.run().unwrap(); 
 return app;

}
