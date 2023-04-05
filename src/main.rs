// Copyright (C) 2023 Hamed Sabri
// All rights reserved.

//#![allow(dead_code, unused)]
//#![allow(unused_variables)]

mod app_context;
use app_context::*;

mod cmake_build;
use cmake_build::*;

mod command_args;
mod json_helper;
mod utility_helper;

use colored::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let context = AppContext::new()?;

    if !context.workspace_dir.exists() {
        println!(
            "{}",
            "Warning: please set `workspace_dir` in project_settings json."
                .yellow()
                .bold()
        );
    } else {
        let project_name_colored = format!("{:?}", &context.project_name).cyan();
        let project_location_colored = format!("{:?}", context.project_location).cyan();
        let workspace_location_colored = format!("{:?}", context.workspace_dir).cyan();
        let stages_colored = format!("{:?}", &context.stages).cyan();
        let variant_colored = format!("{:?}", &context.variant).cyan();
        let build_log_location_colored = format!("{:?}", &context.build_log_location).cyan();

        let mut summary_msg = format!(
            "Building with settings\n\
             \tProject name:       {}\n\
             \tProject directory:  {}\n\
             \tWorkspace directory:  {}\n\
             \tStages arguments:     {}\n\
             \tVariant:     {}",
            project_name_colored,
            project_location_colored,
            workspace_location_colored,
            stages_colored,
            variant_colored
        );

        if context.redirect_outstream.is_some() {
            summary_msg.push_str(&format!("\n\tBuild Log: {}", build_log_location_colored));
        }

        println!("{}", summary_msg.blue());

        // build & install
        time_it!({
            build_project(&context)?;
        });
    }

    Ok(())
}
