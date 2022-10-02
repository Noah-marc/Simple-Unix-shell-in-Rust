pub mod shell {
    // Functions form other files
    use crate::utilities::utilities::*;

    // Rust libraries
    use std::env;
    use std::io::{stdin, stdout, Write};
    use std::path::Path;
    use std::process::{Child, Command, Stdio};

    pub fn shell_run() {
        loop {
            // At the beginning of each shell iteration, we firstly retrieve a current directory
            // we are working in.

            let current_path = env::current_dir().unwrap().to_str().unwrap().to_string();
            print!("{current_path}$", current_path = current_path);

            // this is flushed in order to make sure that the current path is printed before user input,
            // therefore ensuring that the input command will be executed on the same line as
            // the current directory was printed
            stdout().flush().unwrap();

            // Here we take in user input from the console.
            let mut user_input = String::new();
            stdin().read_line(&mut user_input).unwrap();

            // .pop() function is used on the input to delete /n character at the end
            user_input.pop();

            //Here a helper function user_input_reformat is used in order to adjust the user
            // input in such a way that we can process it. (more information in the file utilities.rs)
            let mut reformated_input = user_input_reformat(&user_input);

            //Here we use "streams" to remove '&' character (the division of tasks based
            // on the '&' is processed based on the initial user input) in order to process
            // the commands correctly. TODO check if it is fine
            reformated_input = reformated_input.chars().into_iter().filter(|&ch| ch != '&').collect();

            // Here we split commands based on pipes, this way we get a vector of commands and
            // their arguments. This vector is peekable() meaning the it is possible to check the
            // next value without advancing the iterator (this is done by the .peek() function)
            let mut commands = reformated_input.trim().split(" | ").peekable();

            // This variable will be used to store an ongoing child process. Later from this
            // variable the output of the command will be striped and assigned as an
            // input for the next command. (This is done to handel pipes)
            let mut input_for_next_child = None;

            // At this line the main while which handles command execution begins
            // The condition says that as long as it is possible to get a next command
            // from 'commands' and unpack it 'command' the loop will keep on repeating
            while let Some(command) = commands.next() {

                if command.to_string().is_empty(){
                    break;
                }
                // Here we divided the element taken from 'commands' based on whitespaces
                // we made is so that everything after the first whitespace is
                // interpreted as arguments for the command
                let mut elements_command = command.trim().split_whitespace();
                let command = elements_command.next().unwrap();
                let args = elements_command;

                // Here we have a pattern-matching block allowing us to handel commands
                // in the specific way to execute them correctly
                match command {
                    "cd" => {
                        // The cd command goes to '/' directory if no directory is
                        // provided
                        let new_directory = args.peekable().peek().map_or("/", |x| *x);
                        let dir = Path::new(new_directory);
                        if let Err(e) = env::set_current_dir(&dir) {
                            eprintln!("{}", e);
                        }
                        // This variable is evaluated here to None to make
                        input_for_next_child = None;
                    },

                    // Here we handel the 'exit' keyword which terminates the program.
                    "exit" => return,

                    // Here is the part of the block which handles all other commands
                    command => {
                        // Here we take the earlier mentioned variable input_for_next_child and we strip the output
                        // of the previous command from it to use it as an 'stdin' for the
                        // next command in case of pipes. But if there is no pipe, and there is no
                        // output from previous command the 'stdin' is inherited after corresponding
                        // parent descriptor.
                        let new_stdin = input_for_next_child
                            .map_or(Stdio::inherit(), |output: Child| Stdio::from(output.stdout.unwrap()));

                        // In case of 'stdout' the same rule applies. If there is another command
                        // to which the output will have to be redirected we create a pipe
                        // in order to connect the parent and child process. If there is no
                        // next command the 'stdout' is inherited after the parent descriptor.

                        let new_stdout = if commands.peek().is_some() {
                            Stdio::piped()
                        } else {
                            Stdio::inherit()
                        };

                        // Here we executed the command. We spawn this command as a child.
                        let execution = Command::new(command)
                            .args(args)
                            .stdin(new_stdin)
                            .stdout(new_stdout)
                            .spawn();

                        // Then if the execution of the command was successful we assign it
                        // to the 'input_for_the_next_child' variable, in order to later (see above) get the output of
                        // this command, if the process was piped.
                        // If the execution failed we print an error.
                        match execution {
                            Ok(output) => {
                                input_for_next_child = Some(output);
                            },
                            Err(e) => {
                                input_for_next_child = None;
                                eprintln!("{}", e);
                            },
                        };
                    }
                }
            }
            // Here we handel how the loop should proceed based on the user input, specifically
            // the '&' symbol.

            //If user wrote the '&' symbol at the end of the command and no file redirection was provided then the 'child'
            // is killed.
            // This action is only performed when user input is not empty
            if !user_input.is_empty() {

                if user_input.chars().nth(user_input.len() - 1) == Some('&') && !user_input.contains(">") {
                    if let Some(mut end_command) = input_for_next_child {
                        end_command.kill().expect("panic");
                    }

                    //In this case if user wrote '&' symbol at the end of the command and file redirection
                    // is provided the shell gets respawned immediately (we do not wait for the child processes).
                } else if user_input.chars().nth(user_input.len() - 1) == Some('&') && user_input.contains(">") {
                    input_for_next_child = None;
                }

                // In any other case we proceed with normal command execution, where we wait for
                // all of the commands to finish and then respawn the shell
                else {
                    if let Some(mut end_command) = input_for_next_child {

                        // This command ensures that all commands given in the user input,
                        // were executed, before the we go back to the beginning of the
                        // infinite loop.
                        end_command.wait().unwrap();
                    }
                }
            }
        }
    }
}