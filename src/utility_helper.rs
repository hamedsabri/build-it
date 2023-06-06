use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use walkdir::WalkDir;

pub struct UtilityHelper;

impl UtilityHelper {
    // This function changes the current working directory to the given directory, calls the provided
    // closure, and then changes the current working directory back to the original directory.
    pub fn current_working_directory<F, T>(dir: &PathBuf, f: F) -> Result<T, Box<dyn Error>>
    where
        F: FnOnce() -> Result<T, Box<dyn Error>>,
    {
        let curdir = env::current_dir()?;
        env::set_current_dir(dir)?;
        let result = f()?;
        env::set_current_dir(curdir)?;
        Ok(result)
    }

    // Deletes a directory and all its contents. Returns an error if the directory cannot be deleted.
    pub fn delete_directory(dir_path: &PathBuf) -> std::io::Result<()> {
        match fs::remove_dir_all(dir_path) {
            Ok(_) => {}
            Err(e) => println!("Error deleting folder: {:?}", e),
        }
        Ok(())
    }

    // Creates a new directory at the specified path if it does not already exist.
    // Returns an error if the directory cannot be created.
    pub fn create_new_directory(path: &PathBuf) -> std::io::Result<()> {
        if !path.exists() {
            match fs::create_dir_all(path) {
                Ok(_) => {}
                Err(e) => {
                    println!("Error creating directory {}: {}", path.display(), e);
                    return Err(e);
                }
            }
        }
        Ok(())
    }

    // Creates a new file at the specified path. Returns an error if the file cannot be created.
    pub fn create_new_file(file_path: &PathBuf) -> std::io::Result<File> {
        File::create(file_path).map_err(|e| {
            println!("Error creating file {}: {}", file_path.display(), e);
            e
        })
    }

    // Parses the `CMakeLists.txt` file and all `*.cmake` files in the `./cmake` directory
    // to extract CMake project options defined using the `option()` command.
    // Returns a HashMap of the options and their values.
    pub fn fetch_cmake_project_options() -> HashMap<String, String> {
        let contents = fs::read_to_string("CMakeLists.txt").expect("Error reading file!");
        let re = Regex::new(r#"option\((\w+)\s+".*?"\s+(ON|OFF)\)"#).unwrap();

        let mut options = HashMap::new();

        // also walk the the top level cmake file as well for any additional options
        for entry in WalkDir::new("./cmake").into_iter().filter_map(|e| e.ok()) {
            if entry.path().extension().and_then(|e| e.to_str()) == Some("cmake") {
                let contents = fs::read_to_string(entry.path()).unwrap();

                for line in contents.lines() {
                    if let Some(captures) = re.captures(line) {
                        let option_name = "-D".to_string() + &captures[1].to_string();
                        options.insert(option_name, captures[2].to_string());
                    }
                }
            }
        }

        for cap in re.captures_iter(&contents) {
            let option_name = "-D".to_string() + &cap[1].to_string();
            options.insert(option_name, cap[2].to_string());
        }

        return options;
    }

    pub fn fetch_project_name() -> Option<String> {
        // Open the CMakeLists.txt file
        let file = File::open("CMakeLists.txt").ok()?;
        let reader = BufReader::new(file);

        // Loop through each line in the file
        let mut in_project_block = false;
        for line in reader.lines() {
            let line = line.ok()?;
            let trimmed = line.trim();

            // Check if we're currently inside the project block
            if in_project_block {
                // If the line ends with a parenthesis, extract the project name
                if let Some(captures) = regex::Regex::new(r"^\s*([^\s()]+)")
                    .unwrap()
                    .captures(trimmed)
                {
                    return Some(captures[1].to_owned());
                }
            } else if trimmed.starts_with("project(") {
                // If the line starts a new project block, mark that we're inside it
                in_project_block = true;

                // Extract the project name from the line
                if let Some(captures) = regex::Regex::new(r#"^project\(\s*"?([^\s"]+)"?\s*"#)
                    .unwrap()
                    .captures(trimmed)
                {
                    let mut project_name = captures[1].to_owned();
                    project_name = project_name.trim_end_matches(')').to_string();
                    return Some(project_name);
                }
            }
        }

        None
    }

    // converts a HashMap of string key-value
    // pairs into a string of space-separated
    // key-value pairs in the format "key=value".
    pub fn stringify(entry: &HashMap<String, String>) -> String {
        entry
            .iter()
            .map(|(key, value)| format!("{}={}", key, value))
            .collect::<Vec<String>>()
            .join(" ")
    }
}

#[macro_export]
macro_rules! time_it {
    ($code:block) => {{
        let start_time = std::time::Instant::now();
        $code;
        let elapsed = std::time::Instant::now() - start_time;
        let elapsed_seconds = elapsed.as_secs();
        let hours = elapsed_seconds / 3600;
        let minutes = (elapsed_seconds % 3600) / 60;
        let seconds = elapsed_seconds % 60;
        let eplased_time_colored =
            format!("Elapsed time: {:02}:{:02}:{:02}", hours, minutes, seconds)
                .blue()
                .bold();
        println!("{}", eplased_time_colored);
    }};
}
