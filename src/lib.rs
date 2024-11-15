//! Command-line tool for creating [Mermaid](https://mermaid.js.org/) diagrams from Java sources.
//! As of now, this tool can create flowcharts to represent the usage relationships between mappers
//! created with the [Mapstruct](https://mapstruct.org/) library.

/// Contains the elements that will be diagramed.
pub mod domain {
    /// Represents a single Mapstruct mapper with a list of the names of all other mappers it uses.
    pub struct Mapper {
        pub name: String,
        pub used_sources: Vec<String>,
    }
}

/// Contains definitions for the program's interface.
pub mod cli {
    use clap::{Parser, Subcommand};
    use std::path::PathBuf;

    /// A command-line tool for creating Mermaid diagrams from Java sources.
    #[derive(Parser)]
    #[command(version, about, long_about = None)]
    pub struct Cli {
        #[command(subcommand)]
        pub command: Commands,
    }

    /// Possible diagraming operations.
    #[derive(Subcommand)]
    pub enum Commands {
        /// Construct flowchart for mapper usage relationships.
        MapperUsage {
            /// Root directory for mapper usage analysis.
            #[arg(short, long, default_value = PathBuf::from(".").into_os_string(),
            value_name = "DIRECTORY")]
            directory: PathBuf,
            /// Output file. Prints to standard output if left blank.
            #[arg(short, long, value_name = "OUTPUT")]
            output: Option<PathBuf>,
            /// Print a top to bottom diagram instead of a left to right one.
            #[arg(short, long)]
            top_bottom: bool,
        },
    }
}

use domain::Mapper;
use regex::Regex;
use std::{fs, path::PathBuf};

use lazy_static::lazy_static;

lazy_static! {
    static ref USED_MAPPERS_RE: Regex =
        Regex::new(r"(?ms)uses\s*=\s*(\{([^}]+)\}|([^\.]*\.class))").unwrap();
    static ref COMMENT_TO_EOL_RE: Regex = Regex::new(r"(?ms)//[^\n]*\n").unwrap();
}

/// Obtains a vector of mappers from the current directory recursively.
pub fn get_mappers(path: &str, mappers: &mut Vec<Mapper>) {
    for entry in fs::read_dir(path).unwrap().filter_map(|elmt| elmt.ok()) {
        let path = entry.path();
        if path.is_dir() {
            get_mappers(path.to_str().unwrap(), mappers);
        } else if path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .contains("Mapper")
        {
            mappers.push(create_mapper(path));
        }
    }
}

fn create_mapper(source: PathBuf) -> Mapper {
    return Mapper {
        name: source
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .strip_suffix(".java")
            .unwrap()
            .to_string(),
        used_sources: extract_used_sources(fs::read_to_string(source).unwrap()),
    };
}

fn extract_used_sources(content: String) -> Vec<String> {
    let content = COMMENT_TO_EOL_RE.replace_all(&content, "");

    if let Some(caps) = USED_MAPPERS_RE.captures(&content) {
        if let Some(list) = caps.get(2) {
            return list
                .as_str()
                .split(',')
                .map(|s| s.trim().strip_suffix(".class").unwrap_or(s).to_string())
                .collect();
        }

        if let Some(single) = caps.get(3) {
            return vec![single
                .as_str()
                .trim()
                .strip_suffix(".class")
                .unwrap_or(single.as_str())
                .to_string()];
        }
    }

    Vec::new()
}
