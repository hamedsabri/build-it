use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum Stage {
    Clean,
    Configure,
    Build,
    Install,
}

impl PartialEq for Stage {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

pub type StageResult = Result<Box<[Stage]>, String>;

fn parse_stages(arg: &str) -> StageResult {
    arg.split(',').map(|s| Stage::from_str(s, true)).collect()
}

#[derive(Clone, Debug)]
pub enum Variant {
    Debug,
    Release,
    RelWithDebInfo,
}

impl Variant {
    pub fn as_str(&self) -> &str {
        match self {
            Variant::Debug => "Debug",
            Variant::Release => "Release",
            Variant::RelWithDebInfo => "RelWithDebInfo",
        }
    }
}

fn parse_variant(arg: &str) -> Result<Variant, String> {
    match arg.to_lowercase().as_str() {
        "debug" => Ok(Variant::Debug),
        "release" => Ok(Variant::Release),
        "relWithDebInfo" => Ok(Variant::RelWithDebInfo),
        _ => Err(format!("unknown variant '{arg}'")),
    }
}

#[derive(Clone, Parser, Debug)]
#[command(
    name = "BuildMe",
    about = "Utility program to build and install a cmake project!",
    rename_all = "kebab-case"
)]
pub struct CommandArgs {
    #[arg(
        long = "project-location",
        help = "Path to the root project where the top level CMakeLists resides."
    )]
    pub project_location: PathBuf,

    #[arg(
        long = "stages",
        help = "Comma-separated list of stages to run",
        value_parser = parse_stages,
    )]
    pub stages: Box<[Stage]>,

    #[arg(
        long = "variant",
        help ="possible variants: debug, release, relWithDebInfo",
        value_parser = parse_variant,
    )]
    pub variant: Variant,

    #[arg(
        long = "generator",
        help = "CMake generator to use. By defalut Ninja is used."
    )]
    pub generator: Option<String>,

    #[arg(
        long = "redirect-outstream",
        help = "Redirect output stream to a text file. Set this flag to redirect output stream to console instead."
    )]
    pub redirect_outstream: Option<bool>,
}
