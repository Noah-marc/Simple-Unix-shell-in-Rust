use crate::shell::shell::shell_run;

mod shell;

// TODO: Writing to files does not work we have to fix it
// TODO: Pipes are also not working well, sometimes they work and sometimes not. Have to investigate more.

fn main(){
    shell_run();
}