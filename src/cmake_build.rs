use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};

use super::app_context::*;
use super::command_args::*;
use super::utility_helper::*;

use colored::*;

use num_cpus;
use std::error::Error;

// TODO: can't handle spaces in command
// https://stackoverflow.com/questions/44757893/cmd-c-doesnt-work-in-rust-when-command-includes-spaces
// This function runs a shell command and waits for it to complete before returning.
fn run_command(cmd: &str, context: &AppContext) -> Result<(), std::io::Error> {
    let redirect_outstream = context.redirect_outstream.is_some();

    let mut command = match std::env::consts::OS {
        "windows" => Command::new("cmd")
            .args(&["/C", cmd])
            .stdout(if redirect_outstream {
                Stdio::piped()
            } else {
                Stdio::inherit()
            })
            .stderr(if redirect_outstream {
                Stdio::piped()
            } else {
                Stdio::inherit()
            })
            .spawn()?,
        _ => Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .stdout(if redirect_outstream {
                Stdio::piped()
            } else {
                Stdio::inherit()
            })
            .stderr(if redirect_outstream {
                Stdio::piped()
            } else {
                Stdio::inherit()
            })
            .spawn()?,
    };

    if redirect_outstream {
        let mut logfile = std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(&context.build_log_location)?;

        writeln!(logfile)?;
        writeln!(logfile, "{}", cmd)?;
        writeln!(logfile)?;

        let stdout = command.stdout.take().unwrap();
        let mut reader = BufReader::new(stdout);
        let mut buffer = String::new();
        loop {
            let length = reader.read_line(&mut buffer)?;
            if length == 0 {
                break;
            }
            logfile.write_all(buffer.as_bytes())?;
            buffer.clear();
        }
    }

    command.wait()?;
    Ok(())
}

// This function runs CMake commands to configure, build, and install a project.
fn run_cmake(context: &AppContext) -> Result<(), Box<dyn Error>> {
    let build_variant_dirpath = &context.build_dir.join(context.variant.as_str());
    let install_variant_dirpath = &context.install_dir.join(context.variant.as_str());

    UtilityHelper::current_working_directory(&build_variant_dirpath, || {
        if context.has_stage(Stage::Configure) {
            let command_format = format!(
                "cmake \
                 -DCMAKE_INSTALL_PREFIX={install_dir} \
                 -DCMAKE_BUILD_TYPE={variant} \
                 -DCMAKE_EXPORT_COMPILE_COMMANDS={value} \
                 -G {generator} \
                 {userConfigureArgs} \
                 {extraArgs} \
                 {srcDir} \
                ",
                install_dir = install_variant_dirpath.display(),
                variant = &context.variant.as_str(),
                value = "ON",
                generator = &context.generator,
                userConfigureArgs = UtilityHelper::stringify(&context.configure_args),
                extraArgs = UtilityHelper::stringify(&context.extra_args),
                srcDir = &context.project_location.display()
            );

            println!("{}", command_format.green());

            run_command(&command_format, &context)?;
        }

        if context.has_stage(Stage::Build) || context.has_stage(Stage::Install) {
            let multiproc = format!("-j {}", num_cpus::get());

            let command_format = format!(
                "cmake --build . --config {variant} {installArg} -- {multiproc}",
                variant = &context.variant.as_str(),
                installArg = if context.has_stage(Stage::Install) {
                    "--target install"
                } else {
                    ""
                },
                multiproc = multiproc
            );

            run_command(&command_format, &context)?;

            if context.has_stage(Stage::Build) {
                let colored_build_variant_dirpath =
                    format!("Build Success: {}", build_variant_dirpath.display())
                        .green()
                        .bold();
                println!("{}", colored_build_variant_dirpath);
            }

            if context.has_stage(Stage::Install) {
                let colored_install_variant_dirpath =
                    format!("Install Success: {}", install_variant_dirpath.display())
                        .green()
                        .bold();
                println!("{}", colored_install_variant_dirpath);
            }
        }

        Ok(())
    })?;

    Ok(())
}

// This is the entry point function for building a cmake project.
pub fn build_project(context: &AppContext) -> Result<(), Box<dyn Error>> {
    UtilityHelper::current_working_directory(&context.project_location, || {
        if context.has_stage(Stage::Clean) {
            UtilityHelper::delete_directory(&context.build_dir)?;
        }

        // create workspace, build, install directories
        let build_variant_dir = &context.build_dir.join(context.variant.as_str());
        let install_variant_dir = &context.install_dir.join(context.variant.as_str());

        for dir in [
            &context.workspace_dir,
            &build_variant_dir,
            &install_variant_dir,
        ] {
            UtilityHelper::create_new_directory(dir)?;
        }

        run_cmake(&context)?;

        Ok(())
    })?;

    Ok(())
}