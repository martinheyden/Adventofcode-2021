pub mod read_input {
    use std::fs;

    pub fn read_file_to_string(file_name: &str) -> String {
        let input = fs::read_to_string(file_name);
        match input {
            Ok(s) => s,
            Err(_) => panic!("Couldnt read file"),
        }
    }

    pub fn read_file_to_int_vec(file_name: &str) -> Vec<i64> {
        read_file_to_string(file_name)
            .split("\n")
            .map(|i| i.parse::<i64>().unwrap())
            .collect()
    }

    pub fn read_line_to_int_vec(file_name: &str) -> Vec<i64> {
        let str = read_file_to_string(file_name);
        str.split(",")
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>()
    }

    pub fn read_file_to_string_vec(file_name: &str) -> Vec<String> {
        read_file_to_string(file_name)
            .split("\n")
            .map(|i| i.to_string())
            .collect()
    }
}
