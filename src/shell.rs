pub mod shell {
    use std::env;
    use std::io::{stdin, stdout, Write};
    use std::path::Path;
    use std::process::{Child, Command, Stdio};
    use std::fs::File;

    // do the pre=processing of the input, if > this is detected create a file
    // then delete this from user input
    // create a vector to keep which commands use this symbol

    pub fn shell_run() {
        loop {
            let current_path = env::current_dir().unwrap().to_str().unwrap().to_string();
            print!("{current_path}$", current_path = current_path);
            stdout().flush().unwrap();

            let mut user_input = String::new();
            stdin().read_line(&mut user_input).unwrap();
            user_input.pop();
            let mut reformated_input = user_input_reformat(&user_input);
            reformated_input = reformated_input.chars().into_iter().filter(|&ch| ch != '&').collect();

            // this needs to be peekable in order to determine when we are on the last command
            let mut commands = reformated_input.trim().split(" | ").peekable();
            let mut to_execute = None;


            while let Some(command) = commands.next() {

                // everything after the first whitespace character is interpreted as args to the command
                let mut parts = command.trim().split_whitespace();
                let command = parts.next().unwrap();
                let args = parts;


                match command {
                    "cd" => {
                        // default to '/' as new directory if one was not provided
                        let new_directory = args.peekable().peek().map_or("/", |x| *x);
                        let dir = Path::new(new_directory);
                        if let Err(e) = env::set_current_dir(&dir) {
                            eprintln!("{}", e);
                        }
                        to_execute = None;
                    },
                    "exit" => return,
                    command => {
                        let stdin_child = to_execute
                            .map_or(Stdio::inherit(), |output: Child| Stdio::from(output.stdout.unwrap()));

                        let stdout_child = if commands.peek().is_some() {
                            Stdio::piped()
                        } else {
                            Stdio::inherit()
                        };


                        let execution = Command::new(command)
                            .args(args)
                            .stdin(stdin_child)
                            .stdout(stdout_child)
                            .spawn();


                        match execution {
                            Ok(output) => {
                                to_execute = Some(output);
                            },
                            Err(e) => {
                                to_execute = None;
                                eprintln!("{}", e);
                            },
                        };
                    }


                    // this assigns a command along with all necessary arguments to the child process
                }
            }
            if user_input.chars().nth(user_input.len() - 1) == Some('&') && !user_input.contains(">") {
                if let Some(mut end_command) = to_execute {
                    // block till the last command in the input was executed
                    end_command.kill().expect("panic");
                }
            } else if user_input.chars().nth(user_input.len() - 1) == Some('&') && user_input.contains(">") {
                if let Some(mut end_command) = to_execute {}
            } else {
                if let Some(mut end_command) = to_execute {
                    // block till the last command in the input was executed
                    end_command.wait().unwrap();
                }
            }
        }


        pub fn user_input_reformat(input: &str) -> String {
            let mut vector: Vec<&str> = input.split(" ").collect();

            if vector.contains(&">") {
                for i in 0..vector.len() {
                    if vector[i] == ">" {
                        vector.remove(i);
                        vector.insert(i, " ");
                        vector.insert(i + 1, "|");
                        vector.insert(i + 2 as usize, " ");
                        vector.insert(i + 3 as usize, "tee");
                        vector.insert(i + 4, " ");
                        break;
                    }
                }
                let mut counter = 1;
                for x in 0..vector.len() {
                    if x == vector.len() {
                        break;
                    } else if vector[x] == " " {} else if vector[x] != " " && vector[x + 1] == " " {} else {
                        vector.insert(x + counter, " ");
                        counter += 1;
                    }
                }

                let vector_str: String = vector.into_iter().collect();

                return vector_str;
            }
            return input.to_string();
        }
    }
}