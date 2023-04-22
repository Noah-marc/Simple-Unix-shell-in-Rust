# Simple-Unix-shell-in-Rust

This was project was done by a team consisting of 2 people. The goal was to create a simple command shell in rust that is capable of processing
almost all standard commands (f.e. "cd", "ls", "mkdir", and many more). THe whole shell is written entirely in rust. 

The logic behind the shell can be found in 'src/shell.rs'. The code has comments explaining every step. The main file is only used for calling the shell 
using shell_run(). 

The shell works and exectues every command, however there was one problem during our implementation that we could not fix during the timeframe that we had. 
When implementing simple writing to files by using ">" we run into ownership problems with rust, since we had problems to provide the output of a previous 
command without breaking the ownership constraint (more info can be found in the utilities.rs file). However, as a workaround we basically reformated 
commands to include a piped tee command, in order to still provide this simple writing functionality for our shell (again, more info in the utilities file)
