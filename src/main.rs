extern crate core;

use crate::shell::shell::*;

mod shell;
mod utilities;

// The logic for our shell is located in shell.rs. Here we only call the shell_run() function. 

fn main(){
    shell_run();
}