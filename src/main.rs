use std::io::{Read, Write};

use base64::prelude::*;
use clap::Parser;
use flate2::write::ZlibEncoder;
use j2mmd::{
    cli::{Cli, Commands},
    get_mappers,
};

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::MapperUsage {
            directory,
            output,
            top_bottom,
            live_editor,
        } => {
            let mut mappers = Vec::new();
            let mut independent_mapper_lines = Vec::new();
            get_mappers(directory.to_str().unwrap(), &mut mappers);
            let mut body_lines = Vec::new();

            for mapper in &mappers {
                for used in &mapper.used_sources {
                    body_lines.push(format!("  {} --> {}", mapper.name, used));
                }
            }
            let body_lines = body_lines.join("\n");

            // Add the mappers that don't have any dependencies to the diagram lines
            for mapper in mappers {
                if !body_lines.contains(&mapper.name) {
                    independent_mapper_lines.push(format!("  {}", mapper.name));
                }
            }
            let diagram = format!(
                "flowchart {}\n{}\n{}",
                if top_bottom { "TB" } else { "LR" },
                body_lines,
                independent_mapper_lines.join("\n")
            );

            if let Some(out_file) = output {
                std::fs::write(out_file, &diagram).expect("Error writing file");
            } else if live_editor {
                let mut encoder = ZlibEncoder::new(Vec::new(), flate2::Compression::default());
                let json_data = format!("{{ \"code\": \"{}\", \"mermaid\": {{ \"theme\": \"default\" }} }}", diagram.replace("\n", "\\n"));
                let _ = encoder.write_all(json_data.as_bytes());
                let encoded_diagram = encoder.finish().expect("Unable to compress diagram");
                println!("https://mermaid.live/view#pako:{}", BASE64_STANDARD.encode(encoded_diagram));
            }else {
                println!("{diagram}");
            }
        }
    }
}
