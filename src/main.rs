use clap::Parser;
use j2mmd::{
    cli::{Cli, Commands},
    get_mappers,
};

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::MapperUsage { directory, output } => {
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
                "flowchart LR\n{}\n{}",
                body_lines,
                independent_mapper_lines.join("\n")
            );

            if let Some(out_file) = output {
                std::fs::write(out_file, diagram).expect("Error writing file");
            } else {
                println!("{diagram}");
            }
        }
    }
}
