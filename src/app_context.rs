use clap::Parser;
use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;

use super::command_args::*;
use super::json_helper::*;
use super::utility_helper::*;

const NINA_GENERATOR: &str = "Ninja";
const BUILD_LOG_NAME: &str = "build_log.txt";

// AppContext
pub struct AppContext {
    pub project_location: PathBuf,
    pub workspace_dir: PathBuf,
    pub build_dir: PathBuf,
    pub install_dir: PathBuf,
    pub variant: Variant,
    pub configure_args: HashMap<String, String>,
    pub extra_args: HashMap<String, String>,
    pub generator: String,
    pub stages: Box<[Stage]>,
    pub redirect_outstream: Option<bool>,
    pub build_log_location: PathBuf,
    pub project_name: String,
}

impl AppContext {
    pub fn new() -> Result<AppContext, Box<dyn Error>> {
        let args = CommandArgs::parse();

        let (configure_args, extra_args, workspace_dir, project_name) =
            UtilityHelper::current_working_directory(&args.project_location, || {
                let json_filepath = JsonUtil::create_project_setting(&args.project_location)?;
                let (configure_args, extra_args, workspace_dir) =
                    JsonUtil::parse_json(&json_filepath)?;
                Ok((
                    configure_args,
                    extra_args,
                    workspace_dir,
                    UtilityHelper::fetch_project_name(),
                ))
            })?;

        let project_name = match &project_name {
            Some(project_name) => project_name.clone(),
            None => String::from("Not found"),
        };

        let build_location = workspace_dir.join("build");
        let install_location = workspace_dir.join("install");
        let build_log_path = build_location
            .join(args.variant.as_str())
            .join(BUILD_LOG_NAME);

        Ok(AppContext {
            project_location: args.project_location,
            workspace_dir: workspace_dir,
            build_dir: build_location,
            install_dir: install_location,
            variant: args.variant,
            configure_args: configure_args,
            extra_args: extra_args,
            generator: NINA_GENERATOR.to_string(),
            stages: args.stages,
            redirect_outstream: args.redirect_outstream,
            build_log_location: build_log_path,
            project_name: project_name,
        })
    }

    pub fn has_stage(&self, stage_value: Stage) -> bool {
        self.stages.contains(&stage_value)
    }
}