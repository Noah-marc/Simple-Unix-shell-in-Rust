pub mod utilities {

    // This function was initially created to handle file creation.
    // Unfortunately, due to ownership problems (Rust ownership)
    // we could not get the standard file writing to work (by standard we mean
    // for example: write! macro or File.write_all(), etc.) We could not figure out how to get
    // the output of previous command such as 'echo' without breaking the ownership
    // constraint (main problem: stdin_child). Therefore in order to at least provide some
    // implementation of file creation and writing to them we settled to use the 'tee' command

    // This function's job is to find all occurrences of '>' and replace them with:
    // '| tee [filename]'. We do it by inserting elements into a string. As you can notice
    // this done with the help of pipes. This solution has one drawback. We could not find
    // the way to mute the output of the command writing to the file.
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
                }
            }

            let vector_str: String = vector.into_iter().collect();

            return vector_str;
        }
        return input.to_string();
    }


}