use clap::{Parser, Subcommand};
use ducky_core::{DuckyCompiler, KeyboardLayout};
use ducky_fmt::{DuckyFormatter, FormatterConfig};
use ducky_lint::{DuckyLinter, LinterConfig, LintSeverity};
use sha2::{Digest, Sha256};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser, Debug)]
#[command(name = "duck")]
#[command(about = "DuckyScript toolchain - Build, format, and lint DuckyScript payloads", long_about = None)]
#[command(version)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(about = "Compile DuckyScript to inject.bin")]
    Build {
        #[arg(short, long, help = "Input DuckyScript file (default: from duck.toml workspace.main_file)")]
        input: Option<PathBuf>,
        
        #[arg(short, long, help = "Output file (default: inject.bin)")]
        output: Option<PathBuf>,
        
        #[arg(short, long, help = "Keyboard layout JSON file")]
        layout: Option<PathBuf>,
        
        #[arg(short = 'c', long, help = "Config file path (default: duck.toml)")]
        config: Option<PathBuf>,
        
        #[arg(short, long, help = "Verbose output")]
        verbose: bool,
        
        #[arg(long, help = "Show compiler statistics")]
        stats: bool,
        
        #[arg(long, help = "Output hex dump instead of binary")]
        hex: bool,
    },
    #[command(about = "Format DuckyScript files")]
    Fmt {
        #[arg(help = "Input DuckyScript file(s)")]
        input: Vec<PathBuf>,
        
        #[arg(short = 'c', long, help = "Config file path (default: duck.toml)")]
        config: Option<PathBuf>,
        
        #[arg(long, help = "Check formatting without writing changes")]
        dry_run: bool,
        
        #[arg(short, long, help = "Verbose output")]
        verbose: bool,
    },
    #[command(about = "Lint DuckyScript files")]
    Lint {
        #[arg(help = "Input DuckyScript file(s)")]
        input: Vec<PathBuf>,
        
        #[arg(short = 'c', long, help = "Config file path (default: duck.toml)")]
        config: Option<PathBuf>,
        
        #[arg(long, help = "Show lint results without failing")]
        dry_run: bool,
        
        #[arg(short, long, help = "Verbose output")]
        verbose: bool,
    },
    #[command(about = "Initialize a new ducky.toml configuration file")]
    Init {
        #[arg(short, long, help = "Output path for config file (default: ducky.toml)")]
        output: Option<PathBuf>,
    },
    #[command(about = "Show version information")]
    Version,
    #[command(about = "Update duck toolchain to the latest version")]
    Update,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct WorkspaceConfig {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub main_file: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DuckyConfig {
    #[serde(default)]
    workspace: WorkspaceConfig,
    #[serde(skip_serializing_if = "Option::is_none")]
    formatter: Option<FormatterConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    linter: Option<LinterConfig>,
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Build { input, output, layout, config, verbose, stats, hex } => {
            build_command(input, output, layout, config.clone(), verbose, stats, hex);
        }
        Commands::Fmt { input, config, dry_run, verbose } => {
            fmt_command(input, config, dry_run, verbose);
        }
        Commands::Lint { input, config, dry_run, verbose } => {
            lint_command(input, config, dry_run, verbose);
        }
        Commands::Init { output } => {
            init_command(output);
        }
        Commands::Version => {
            version_command();
        }
        Commands::Update => {
            update_command();
        }
    }
}

fn build_command(
    input: Option<PathBuf>,
    output: Option<PathBuf>,
    layout: Option<PathBuf>,
    config_path: Option<PathBuf>,
    verbose: bool,
    stats: bool,
    hex: bool,
) {
    // Determine input file: from -i flag or from duck.toml workspace.main_file
    let input = if let Some(input_path) = input {
        input_path
    } else {
        let config = load_config(config_path);
        if let Some(cfg) = config {
            if let Some(main_file) = cfg.workspace.main_file {
                PathBuf::from(main_file)
            } else {
                eprintln!("Error: No input file specified.");
                eprintln!("  Use -i <file> or set workspace.main_file in duck.toml");
                std::process::exit(1);
            }
        } else {
            eprintln!("Error: No input file specified.");
            eprintln!("  Use -i <file> or set workspace.main_file in duck.toml");
            std::process::exit(1);
        }
    };

    if verbose {
        println!("DuckyScript Compiler v0.1.0");
        println!("Compiling: {:?}", input);
    }

    let source = match fs::read_to_string(&input) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading input file: {}", e);
            std::process::exit(1);
        }
    };

    let keyboard_layout = if let Some(layout_path) = layout {
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
                    eprintln!("  Line {}: {}", warning.line(), warning.message());
                }
            }
            
            std::process::exit(1);
        }
    };
    let compile_time = start_time.elapsed();

    let output_path = output.unwrap_or_else(|| PathBuf::from("inject.bin"));

    if hex {
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

    // Match official DuckyScript compiler: hash the hex string, not raw bytes
    let hex_string = hex::encode(&result);
    let mut hasher = Sha256::new();
    hasher.update(hex_string.as_bytes());
    let hash = hasher.finalize();
    let checksum = hex::encode(hash);

    let ds_version = if compiler.ds3_detected {
        "DuckyScript 3.0"
    } else {
        "DuckyScript 1.0"
    };

    let payload_percent = (result.len() as f64 / 16384.0) * 100.0;

    println!("Successfully compiled {} into {}", 
        input.display(), 
        output_path.display()
    );
    println!("  Version: {}", ds_version);
    println!("  Size: {} bytes ({:.1}% of 16KB)", result.len(), payload_percent);
    println!("  Compile time: {:.2?}", compile_time);
    println!("  SHA256: {}", checksum);

    if stats {
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
            println!("  Line {}: {}", warning.line(), warning.message());
        }
    }

    if verbose {
        println!("\nCompilation complete!");
    }
}

fn init_command(output: Option<PathBuf>) {
    let config_path = output.unwrap_or_else(|| PathBuf::from("duck.toml"));
    
    // Check if config already exists
    let mut config = if config_path.exists() {
        println!("Found existing configuration: {}", config_path.display());
        match fs::read_to_string(&config_path) {
            Ok(content) => match toml::from_str(&content) {
                Ok(existing_config) => existing_config,
                Err(e) => {
                    eprintln!("Warning: Failed to parse existing config: {}", e);
                    eprintln!("Creating new configuration.");
                    DuckyConfig {
                        workspace: WorkspaceConfig {
                            name: None,
                            version: None,
                            main_file: Some("helloworld.txt".to_string()),
                        },
                        formatter: None,
                        linter: None,
                    }
                }
            },
            Err(e) => {
                eprintln!("Warning: Failed to read existing config: {}", e);
                eprintln!("Creating new configuration.");
                DuckyConfig {
                    workspace: WorkspaceConfig {
                        name: None,
                        version: None,
                        main_file: Some("helloworld.txt".to_string()),
                    },
                    formatter: None,
                    linter: None,
                }
            }
        }
    } else {
        DuckyConfig {
            workspace: WorkspaceConfig {
                name: None,
                version: None,
                main_file: Some("helloworld.txt".to_string()),
            },
            formatter: None,
            linter: None,
        }
    };
    
    // Ensure workspace.main_file is set if missing
    if config.workspace.main_file.is_none() {
        config.workspace.main_file = Some("helloworld.txt".to_string());
        println!("  Added workspace.main_file = \"helloworld.txt\"");
    }
    
    let toml_str = toml::to_string_pretty(&config).expect("Failed to serialize config");
    let content = format!("# DuckyScript Configuration File\n# Run 'duck fmt' or 'duck lint' to add formatter/linter sections\n\n{}", toml_str);
    
    if let Err(e) = fs::write(&config_path, content) {
        eprintln!("Error writing config file: {}", e);
        std::process::exit(1);
    }
    
    // Create helloworld.txt example only if it doesn't exist
    let hello_path = PathBuf::from("helloworld.txt");
    if hello_path.exists() {
        println!("  Skipped: {} already exists", hello_path.display());
    } else {
        let hello_world = r#"REM DuckyScript Hello World Example
REM This is a simple example to get you started

DELAY 1000
STRING Hello, World!
ENTER
"#;
        
        if let Err(e) = fs::write(&hello_path, hello_world) {
            eprintln!("Warning: Failed to create helloworld.txt: {}", e);
        } else {
            println!("Created example file: {}", hello_path.display());
        }
    }
    
    if config_path.exists() {
        println!("Updated configuration file: {}", config_path.display());
    } else {
        println!("Created configuration file: {}", config_path.display());
    }
    println!("  Run 'duck fmt' or 'duck lint' to add tool-specific configuration.");
    println!("  Run 'duck build -i helloworld.txt' to compile the example.");
}

fn version_command() {
    println!("duck {}", VERSION);
    println!("DuckyScript toolchain");
    println!();
    println!("Components:");
    println!("  compiler:  {}", VERSION);
    println!("  formatter: {}", VERSION);
    println!("  linter:    {}", VERSION);
    println!("  lsp:       {}", VERSION);
}

fn update_command() {
    println!("Checking for updates...");
    println!("Current version: {}", VERSION);
    println!();
    
    // Call cargo-dist-updater to perform the update
    let status = Command::new("cargo-dist-updater")
        .arg("update")
        .status();
    
    match status {
        Ok(exit_status) if exit_status.success() => {
            println!();
            println!("Update completed successfully!");
            println!("  Please restart your terminal to use the new version.");
        }
        Ok(exit_status) => {
            eprintln!();
            eprintln!("Update failed with exit code: {}", exit_status);
            eprintln!("  Try re-running the installer manually:");
            eprintln!("  curl -L https://github.com/Greenstorm5417/duck-tools/releases/latest/download/install.sh | sh");
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!();
            eprintln!("Failed to run updater: {}", e);
            eprintln!("  Make sure cargo-dist-updater is installed in the same directory as duck.");
            eprintln!("  Or re-run the installer manually:");
            eprintln!("  curl -L https://github.com/Greenstorm5417/duck-tools/releases/latest/download/install.sh | sh");
            std::process::exit(1);
        }
    }
}

fn find_config_file(config_path: Option<PathBuf>) -> Option<PathBuf> {
    if let Some(path) = config_path {
        if path.exists() {
            return Some(path);
        }
        return None;
    }
    
    // Try current directory first
    let current_dir = std::env::current_dir().ok()?;
    let config_in_current = current_dir.join("duck.toml");
    if config_in_current.exists() {
        return Some(config_in_current);
    }
    
    // Try parent directory
    if let Some(parent) = current_dir.parent() {
        let config_in_parent = parent.join("duck.toml");
        if config_in_parent.exists() {
            return Some(config_in_parent);
        }
    }
    
    None
}

fn load_config(config_path: Option<PathBuf>) -> Option<DuckyConfig> {
    let path = find_config_file(config_path)?;
    
    match fs::read_to_string(&path) {
        Ok(content) => match toml::from_str(&content) {
            Ok(config) => Some(config),
            Err(e) => {
                eprintln!("Error: Failed to parse config file: {}", e);
                None
            }
        },
        Err(e) => {
            eprintln!("Error: Failed to read config file: {}", e);
            None
        }
    }
}

fn fmt_command(inputs: Vec<PathBuf>, config_path: Option<PathBuf>, dry_run: bool, verbose: bool) {
    let config = match load_config(config_path.clone()) {
        Some(cfg) => cfg,
        None => {
            eprintln!("Error: No configuration file found.");
            eprintln!("  Run 'duck init' to create duck.toml");
            std::process::exit(1);
        }
    };
    
    // If no input files specified, try to use workspace.main_file
    let inputs = if inputs.is_empty() {
        if let Some(main_file) = &config.workspace.main_file {
            vec![PathBuf::from(main_file)]
        } else {
            eprintln!("Error: No input files specified.");
            eprintln!("  Usage: duck fmt <file1> <file2> ...");
            eprintln!("  Or set workspace.main_file in duck.toml");
            std::process::exit(1);
        }
    } else {
        inputs
    };
    
    let mut formatter_config = config.formatter.unwrap_or_default();
    formatter_config.enabled = true;
    
    let formatter = DuckyFormatter::new(formatter_config);
    let mut total_files = 0;
    let mut formatted_files = 0;
    let mut total_changes = 0;
    let mut errors = 0;
    
    for input in inputs {
        total_files += 1;
        
        let source = match fs::read_to_string(&input) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Error reading {}: {}", input.display(), e);
                errors += 1;
                continue;
            }
        };
        
        let formatted = match formatter.format(&source) {
            Ok(result) => result,
            Err(e) => {
                eprintln!("Error formatting {}: {}", input.display(), e);
                errors += 1;
                continue;
            }
        };
        
        if source != formatted {
            formatted_files += 1;
            
            // Count changed lines
            let changes = source.lines().zip(formatted.lines())
                .filter(|(a, b)| a != b)
                .count()
                + (source.lines().count() as isize - formatted.lines().count() as isize).abs() as usize;
            total_changes += changes;
            
            if dry_run {
                if verbose {
                    println!("Would format {} ({} changes)", input.display(), changes);
                }
            } else {
                if let Err(e) = fs::write(&input, formatted) {
                    eprintln!("Error writing {}: {}", input.display(), e);
                    errors += 1;
                    continue;
                }
                if verbose {
                    println!("Formatted {} ({} changes)", input.display(), changes);
                }
            }
        } else if verbose {
            println!("Already formatted {}", input.display());
        }
    }
    
    if dry_run {
        if formatted_files > 0 {
            println!("\n{} file(s) would be formatted ({} changes)", formatted_files, total_changes);
        } else {
            println!("\nNo files need formatting");
        }
    } else {
        if formatted_files > 0 {
            println!("\nFormatted {} of {} file(s) ({} changes)", formatted_files, total_files, total_changes);
        } else {
            println!("\nNo files needed formatting");
        }
    }
    
    if errors > 0 {
        eprintln!("\n{} error(s) occurred", errors);
        std::process::exit(1);
    }
}

fn lint_command(inputs: Vec<PathBuf>, config_path: Option<PathBuf>, dry_run: bool, verbose: bool) {
    let config = match load_config(config_path.clone()) {
        Some(cfg) => cfg,
        None => {
            eprintln!("Error: No configuration file found.");
            eprintln!("  Run 'duck init' to create duck.toml");
            std::process::exit(1);
        }
    };
    
    // If no input files specified, try to use workspace.main_file
    let inputs = if inputs.is_empty() {
        if let Some(main_file) = &config.workspace.main_file {
            vec![PathBuf::from(main_file)]
        } else {
            eprintln!("Error: No input files specified.");
            eprintln!("  Usage: duck lint <file1> <file2> ...");
            eprintln!("  Or set workspace.main_file in duck.toml");
            std::process::exit(1);
        }
    } else {
        inputs
    };
    
    let mut linter_config = config.linter.unwrap_or_default();
    linter_config.enabled = true;
    
    let linter = DuckyLinter::new(linter_config);
    let mut total_issues = 0;
    let mut total_errors = 0;
    let mut total_warnings = 0;
    let mut total_infos = 0;
    let mut files_with_issues = 0;
    
    for input in &inputs {
        let source = match fs::read_to_string(input) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Error reading {}: {}", input.display(), e);
                continue;
            }
        };
        
        let issues = linter.lint(&source);
        
        if !issues.is_empty() {
            files_with_issues += 1;
            total_issues += issues.len();
            
            if verbose || !dry_run {
                println!("\nLinting issues in {}:", input.display());
                for issue in &issues {
                    let severity_str = match issue.severity {
                        LintSeverity::Error => "ERROR",
                        LintSeverity::Warning => "WARN",
                        LintSeverity::Info => "INFO",
                    };
                    println!("  {}:{} [{}] {} ({})", 
                        issue.line, issue.column, severity_str, issue.message, issue.rule);
                }
            }
            
            total_errors += issues.iter().filter(|i| i.severity == LintSeverity::Error).count();
            total_warnings += issues.iter().filter(|i| i.severity == LintSeverity::Warning).count();
            total_infos += issues.iter().filter(|i| i.severity == LintSeverity::Info).count();
        } else if verbose {
            println!("No issues in {}", input.display());
        }
    }
    
    println!("\nLinted {} file(s)", inputs.len());
    
    if total_issues > 0 {
        println!("Found {} issue(s) in {} file(s):", total_issues, files_with_issues);
        println!("  {} error(s)", total_errors);
        println!("  {} warning(s)", total_warnings);
        println!("  {} info(s)", total_infos);
        
        if !dry_run && total_errors > 0 {
            std::process::exit(1);
        }
    } else {
        println!("No linting issues found");
    }
}
