use crate::modules::feat::*;
use crate::modules::structs::*;
use crate:: modules::tree::*;
use crate::modules::initial::*;
pub fn flags(_procdata: &mut Vec<Proc>)->()
{
    

    loop
        {
            
            let mut ch = String::new();
            
            std::io::stdin().read_line(&mut ch).expect("Failed to read line");
            let mut procdata;
            procdata = initialise_vec1();
            

            let mut input_parts = ch.trim().split_whitespace();

            let first_input = input_parts.next().expect("No first input provided");
            
              if first_input=="-sn"  {
               let second_input = input_parts.next().expect("No second input provided");
               let searching = &mut procdata;
               search_name(searching, second_input);
           }
           else if first_input=="-kp" {
                let second_input = input_parts.next().expect("No second input provided");
            
                let proc_kill= &mut procdata;
                let second: u32 = second_input.trim().parse().expect("Please type a number!");
               kill_process(second, proc_kill);
           }
           else if first_input== "-csv" {
               let proc_csv = &mut procdata;
               save_csv(proc_csv);
           }
           else if first_input=="-pu"{
                let proc_uid = &mut procdata;
                let second_input = input_parts.next().expect("No second input provided");
                let second: u32 = second_input.trim().parse().expect("Please type a number!");
               get_processes_by_uid(proc_uid, second);
           }
           else if first_input=="-pr" {
            let second_input = input_parts.next().expect("No second input provided");
            let third_input = input_parts.next().expect("No third input provided");
                let second: u32 = second_input.trim().parse().expect("Please type a number!");
                let third: i32 = third_input.trim().parse().expect("Please type a number!");

               unsafe{
                   set_priority(second, third);
               }
           }
           else if first_input=="-q" {
               println!("Quitting");
                break;
           }
          else if first_input== "-pt" {

            let mut procdata2 = procdata.clone();

               let _parent = TreeProc{pid: 1 as u32, name: "systemd".to_string(), children: Vec::new()};
               let tree = build_tree(&mut procdata, &mut procdata2);
               print_tree(&tree, 0);
           }
           else if first_input=="-bk"
           {
            //call recursive kill
            let second_input = input_parts.next().expect("No second input provided");
            let second: u32 = second_input.trim().parse().expect("Please type a number!");

            
            for process in procdata 
            {
                if process.pid == second
                {
                    let mut procdata = initialise_vec1();
                    let procdata2=initialise_vec1();
                    let root =build_baby_tree(&mut procdata, &procdata2, process.pid, process.name);
                    let proc_kill= &mut procdata;
                    recursive_kill(root, proc_kill);
                    break;
                }

            }
           
           }
          
           else if first_input=="-h"
           {
            println!("-sn. Search by name");
            println!("-kp. Kill process by PID");
            println!("-csv. Save to CSV");
            println!("-pu. get processes with given uid");
            println!("-pr. set priority of process");
            println!("-pt. print process tree");
            println!("-bk. kill branch of process tree by pid");
            println!("-q. Quit");

           }
           else {
            // print ch 
            println!("You entered {}", first_input);
            println!("Invalid choice");
           }
           
            
        }
       
      
}