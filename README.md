# LinuxProcessManager
Created a Process Manager for Linux using Rust. This project implements both a cli and gui. The Gui is implemented using FLTK crate. 


GUI
1) A table of the process information as well showing for every process whether it is a system process or a user process
2) Sorting by Priority, Memory Usage, CPU Usage, Run Time, Disk Read and Written Bytes as well as PID
3) Searching for a process by its name or PID
4) Showing Memory Usage Graph, CPU Usage Graph as well as Run Time Graph
5) Kill a process by selecting it and pressing Kill Process button
6) Kill a branch by just selecting the parent process from the table and clicking on Kill branch button
7) Real-Time Updates for the data every 5 seconds
B. CLI
1) Search for all the processes with the same name by entering the process name (will get shown as a table)
2) Kill process by PID
3) Save process table as a CSV file
4) Get all processes with the given UID
5) Set priority of processes
6) Print Process Tree
7) Kill a branch of the process tree by PID
8) Ask for help by displaying a list with all the provided functionalities.


LIBRARIES NEEDED AND THEIR VERSIONS
Here is the list of all the libraries used in the GUI and their versions:
1) fltk = ”1.4.1” used for the GUI window
2) fltk-evented = ”0.3.0” used for drop-down menus and buttons
3) fltk-table = ”0.3.0” used to create the table in the GUI
4) misc = ”0.1.0” used to create the chart in the GUI
5) procfs = ”0.15.1” used to retrieve some information about the processes
6) rand = ”0.8.5” used to generate the colors in the charts in the GUI
7) sysinfo = ”0.28.4” used to retrieve some information about the processes
8) time = ”0.3.20” used for the real-time updates in the GUI
9) libc = ”0.2.141” used to get the priority of the processes
Here is the list of all the libraries used in the CLI and their versions:
1) csv = ”1.2.1”, used to save in csv in the CLI
2) libc = ”0.2.142” used to get the priority of the processes
3) prettytable = ”0.10.0” used in the CLI to print the table when searching for processes
4) procfs = ”0.15.1” used to retrieve some information about the processes
5) sysinfo = ”0.28.4” used to retrieve some information about the processes


A shell file was used in order to run both the CLI and GUI together
