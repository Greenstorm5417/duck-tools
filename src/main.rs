use clap::Parser;
use ducky_parse::{DuckyCompiler, KeyboardLayout};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "ducky-parse")]
#[command(about = "DuckyScript Compiler - Compile DuckyScript payloads to inject.bin", long_about = None)]
struct Args {
    #[arg(help = "Input DuckyScript file (.txt)")]
    input: PathBuf,

    #[arg(short, long, help = "Output file (default: inject.bin)")]
    output: Option<PathBuf>,

    #[arg(short, long, help = "Keyboard layout JSON file")]
    layout: Option<PathBuf>,

    #[arg(short, long, help = "Verbose output")]
    verbose: bool,

    #[arg(long, help = "Show compiler statistics")]
    stats: bool,

    #[arg(long, help = "Output hex dump instead of binary")]
    hex: bool,
}

fn main() {
    let args = Args::parse();

    if args.verbose {
        println!("DuckyScript Compiler v0.1.0");
        println!("Compiling: {:?}", args.input);
    }

    let source = match fs::read_to_string(&args.input) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading input file: {}", e);
            std::process::exit(1);
        }
    };

    let keyboard_layout = if let Some(layout_path) = args.layout {
        match fs::read_to_string(&layout_path) {
            Ok(json) => match serde_json::from_str::<KeyboardLayout>(&json) {
                Ok(layout) => Some(layout),
                Err(e) => {
                    eprintln!("Error parsing keyboard layout: {}", e);
                    std::process::exit(1);
                }
            },
            Err(e) => {
                eprintln!("Error reading layout file: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        None
    };

    let mut compiler = DuckyCompiler::new(keyboard_layout);

    let start_time = std::time::Instant::now();
    let result = match compiler.compile(&source) {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("Compilation failed: {}", e);
            
            if !compiler.errors.is_empty() {
                eprintln!("\nErrors:");
                for error in &compiler.errors {
                    eprintln!("  {}", error);
                }
            }
            
            if !compiler.warnings.is_empty() {
                eprintln!("\nWarnings:");
                for warning in &compiler.warnings {
                    eprintln!("  Line {}: {}", warning.line, warning.message);
                }
            }
            
            std::process::exit(1);
        }
    };
    let compile_time = start_time.elapsed();

    let output_path = args.output.unwrap_or_else(|| PathBuf::from("inject.bin"));

    if args.hex {
        let hex_output = hex::encode(&result);
        if let Err(e) = fs::write(&output_path, hex_output) {
            eprintln!("Error writing output file: {}", e);
            std::process::exit(1);
        }
    } else {
        if let Err(e) = fs::write(&output_path, &result) {
            eprintln!("Error writing output file: {}", e);
            std::process::exit(1);
        }
    }

    let mut hasher = Sha256::new();
    hasher.update(&result);
    let hash = hasher.finalize();
    let checksum = hex::encode(hash);

    let ds_version = if compiler.ds3_detected {
        "DuckyScript 3.0"
    } else {
        "DuckyScript 1.0"
    };

    let payload_percent = (result.len() as f64 / 16384.0) * 100.0;

    println!("✓ Successfully compiled {} into {}", 
        args.input.display(), 
        output_path.display()
    );
    println!("  Version: {}", ds_version);
    println!("  Size: {} bytes ({:.1}% of 16KB)", result.len(), payload_percent);
    println!("  Compile time: {:.2?}", compile_time);
    println!("  SHA256: {}", checksum);

    if args.stats {
        println!("\nStatistics:");
        println!("  Allocated registers: {}", compiler.state.var_values.len().saturating_sub(1));
        println!("  Labels defined: {}", compiler.state.label_map.len());
        println!("  Variables: {}", compiler.state.var_map.len());
        if compiler.state.requires_lang_pack {
            println!("  Language pack: Required");
        }
    }

    if !compiler.warnings.is_empty() {
        println!("\nWarnings ({}):", compiler.warnings.len());
        for warning in &compiler.warnings {
            println!("  Line {}: {}", warning.line, warning.message);
        }
    }

    if args.verbose {
        println!("\nCompilation complete!");
    }
}
