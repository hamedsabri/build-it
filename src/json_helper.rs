use serde_json::Value;
use serde_json::{json, to_writer_pretty};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

use super::utility_helper::*;

const PROJECT_SETTINGS_FILEPATH_NAME: &str = "project_settings.json";
const WORKSPACE_DIR: &str = "workspace_dir";
const PROJECT_OPTIONS: &str = "project_options";
const EXTRA_ARGS: &str = "extra_args";

type StringHashMap = HashMap<String, String>;

// A struct with utility functions to create and parse project settings in JSON format
pub struct JsonUtil {}

impl JsonUtil {
    // Function to create project settings in JSON format at a given directory path
    pub fn create_project_setting(dir_path: &PathBuf) -> Result<PathBuf, Box<dyn Error>> {
        let project_settings_filepath = dir_path.join(PROJECT_SETTINGS_FILEPATH_NAME);
        if project_settings_filepath.exists() {
            return Ok(project_settings_filepath);
        }
        // set the data to write to the JSON file
        let workspace_dir = "path/to/valid/directory";
        let extra_args = json!({});

        let data = json!({
            WORKSPACE_DIR: workspace_dir,
            PROJECT_OPTIONS: UtilityHelper::fetch_cmake_project_options(),
            EXTRA_ARGS: extra_args,
        });

        // Use a closure to create a new file with project settings data and write it in JSON format
        let file_path = UtilityHelper::current_working_directory(&dir_path, || {
            let j_file = UtilityHelper::create_new_file(&project_settings_filepath)?;
            to_writer_pretty(&j_file, &data)?;
            Ok(project_settings_filepath)
        })?;

        Ok(file_path)
    }

    // Function to parse a project settings JSON file
    pub fn parse_json(
        file_path: &PathBuf,
    ) -> Result<(StringHashMap, StringHashMap, PathBuf), Box<dyn std::error::Error>> {
        let json_file = fs::read_to_string(file_path)?;
        let parsed: Value = serde_json::from_str(&json_file)?;

        let workspace_dir = PathBuf::from(parsed[WORKSPACE_DIR].as_str().unwrap());

        let project_options = parsed[PROJECT_OPTIONS].as_object().unwrap();
        let mut configure_args: StringHashMap = HashMap::new();
        for (key, value) in project_options {
            configure_args.insert(key.to_string(), value.as_str().unwrap().to_string());
        }

        let user_extra_args = parsed[EXTRA_ARGS].as_object().unwrap();
        let mut extra_args: StringHashMap = HashMap::new();
        for (key, value) in user_extra_args {
            extra_args.insert(key.to_string(), value.as_str().unwrap().to_string());
        }

        Ok((configure_args, extra_args, workspace_dir))
    }
}
