mod mesh;
mod parser;

use {
    clap::Parser,
    std::{
        env,
        fs::{self, File},
        io,
        path::PathBuf,
        process::ExitCode,
    },
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// File to parse (stdin by default)
    filepath: Option<PathBuf>,

    /// Enable verbore output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> ExitCode {
    use crate::parser::Parameters;

    let Cli { filepath, verbose } = Cli::parse();
    let src = match &filepath {
        Some(path) => {
            if let Ok(src) = fs::read_to_string(path) {
                src
            } else {
                eprintln!("failed to read file {path:?}");
                return ExitCode::FAILURE;
            }
        }
        None => {
            if let Ok(src) = io::read_to_string(io::stdin()) {
                src
            } else {
                eprintln!("failed to read stdin");
                return ExitCode::FAILURE;
            }
        }
    };

    let params = Parameters {
        verbose,
        pos_fn: pos,
        map_fn: map,
    };

    match parser::parse(params, &src) {
        Ok(elements) => {
            let Ok(curr) = env::current_dir() else {
                eprintln!("failed to get current directory");
                return ExitCode::FAILURE;
            };

            for element in elements {
                let mut path = curr.join(element.name);
                path.set_extension("json");
                let Ok(file) = File::create(&path) else {
                    eprintln!("failed to open file {path:?}");
                    return ExitCode::FAILURE;
                };

                println!("write element to file {path:?}");
                serde_json::to_writer(file, &element.mesh).expect("serialize element");
            }
        }
        Err(err) => {
            eprintln!("error: {err}");
            return ExitCode::FAILURE;
        }
    }

    ExitCode::SUCCESS
}

fn pos(points: [f32; 3]) -> [f32; 3] {
    const DECIMALS: u32 = 4;
    const ACCURACY: u32 = u32::pow(10, DECIMALS);

    let a = ACCURACY as f32;
    let update = |mut v: f32| {
        if v == -0. {
            v = 0.;
        }

        (v * a).round() / a
    };

    points.map(update)
}

fn map([u, v]: [f32; 2]) -> [f32; 2] {
    [u, 1. - v]
}
