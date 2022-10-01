pub mod shell {
    use std::borrow::Borrow;
    use std::env;
    use std::io::{stdin, stdout, Write};
    use std::path::Path;
    use std::process::{Child, Command, Stdio};
    use std::fs::File;
    use std::ops::Deref;
    use std::str;

    pub fn shell_run() {
    loop {
        let current_path = env::current_dir().unwrap().to_str().unwrap().to_string();
        print!("{current_path}$", current_path = current_path);
        stdout().flush().unwrap();

        let mut user_input = String::new();
        stdin().read_line(&mut user_input).unwrap();

        let file_name = "";

        let reformated_input = user_input_reformat(user_input);
        create_file(&reformated_input);


        // this needs to be peekable in order to determine when we are on the last command
        let mut commands =  reformated_input.trim().split(" | ").peekable();
        let mut to_execute = None;


        while let Some(command) = commands.next()  {

            // everything after the first whitespace character is interpreted as args to the command
            let mut parts = command.trim().split_whitespace();
            let command = parts.next().unwrap();
            let mut args = parts;


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
                        .map_or(Stdio::inherit(),
                                |output: Child| Stdio::from(output.stdout.unwrap()));

                    let stdout_child = if commands.peek().is_some() {
                        Stdio::piped()
                    } else {
                        Stdio::inherit()
                    };

                    if command == &">" {
                        let exe = Command::new("tee")
                            .args(file_name)
                            .stdin(stdin_child)
                            .spawn();
                    } else {

                        // this assigns a command along with all necessary arguments to the child process
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

                }
            }
        }

        if let Some(mut end_command) = to_execute {
            // block till final command is done
            end_command.wait().unwrap();
        }

        }
    }

    pub fn user_input_reformat(input:String) -> String {

        let mut vector:Vec<char> = Vec::new();

        for x in 0..input.len() {
            vector.push(input.chars().nth(x).unwrap());
        }

        for i in 0..vector.len(){
            if vector[i] == '>' {
                vector.insert(i, '|');
                vector.insert(i+1 as usize, ' ');
                break;

            }
        }

        let vector_str:String = vector.into_iter().collect();

        return vector_str;
    }
    pub fn create_file(input:&str) {
        let mut index = 0;
        let k:Vec<&str> = input.trim().split(" ").collect();

        if k.contains(&">") {
            for i in 0..k.len() {
                if k[i].to_string() == ">" {
                    index = i;
                    break;
                }
            }
            file_name = k[index+1];

            let new_path = Path::new(k[index + 1]);
            let mut file = File::create(new_path);
        }

        }


}