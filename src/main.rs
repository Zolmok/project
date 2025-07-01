use clap::Parser;
use dialoguer::Input;
use indicatif::{ProgressBar, ProgressStyle};
use regex::Regex;
use std::fs::{self, OpenOptions};
use std::io::Read;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Duration;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Name of the React app to create
    name: Option<String>,
}

fn setup_tailwind(app_path: &Path, spinner: &ProgressBar) {
    spinner.set_style(
        ProgressStyle::with_template("{spinner} {msg}")
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
    );
    spinner.enable_steady_tick(Duration::from_millis(100));
    spinner.set_message("Installing TailwindCSS...");

    let install = Command::new("npm")
        .arg("install")
        .arg("-D")
        .arg("tailwindcss")
        .arg("@tailwindcss/vite")
        .current_dir(app_path)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    if !matches!(install, Ok(s) if s.success()) {
        spinner.finish_and_clear();
        eprintln!("❌ Failed to install TailwindCSS.");
        std::process::exit(1);
    }

    // Patch vite.config.js
    let vite_config_path = app_path.join("vite.config.js");
    let mut contents = String::new();

    if let Ok(mut file) = OpenOptions::new().read(true).open(&vite_config_path) {
        file.read_to_string(&mut contents).ok();
    }

    // Simple idempotent patch
    if !contents.contains("@tailwindcss/vite") {
        // Inject the import at the top
        let mut patched = format!("import tailwindcss from '@tailwindcss/vite';\n{}", contents);

        // Find plugins array and inject tailwindcss()
        let plugin_re = Regex::new(r"(?s)(plugins:\s*\[)(.*?)\]").unwrap();
        patched = plugin_re
            .replace(&patched, |caps: &regex::Captures| {
                let existing = caps.get(2).map_or("", |m| m.as_str()).trim();
                let mut plugins = vec!["tailwindcss()"];
                if !existing.is_empty() {
                    plugins.push(existing);
                }
                format!("{}{}\n  ]", &caps[1], plugins.join(",\n    "))
            })
            .to_string();

        if fs::write(&vite_config_path, patched).is_err() {
            spinner.finish_and_clear();
            eprintln!("❌ Failed to update vite.config.js.");
            std::process::exit(1);
        }
    }

    // Write src/index.css
    let css_path = app_path.join("src").join("index.css");
    let tailwind_css = "@tailwind base;\n@tailwind components;\n@tailwind utilities;\n";

    if fs::write(css_path, tailwind_css).is_err() {
        spinner.finish_and_clear();
        eprintln!("❌ Failed to write src/index.css.");
        std::process::exit(1);
    }

    spinner.finish_and_clear();
    println!("✅ TailwindCSS with Vite plugin configured.");
}

fn main() {
    let args = Args::parse();

    let app_name = match args.name {
        Some(name) => name,
        None => Input::new()
            .with_prompt("Enter your React app name")
            .interact_text()
            .unwrap_or_else(|err| {
                eprintln!("Failed to read input: {}", err);
                std::process::exit(1);
            }),
    };

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::with_template("{spinner} {msg}")
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
    );
    spinner.enable_steady_tick(std::time::Duration::from_millis(100));

    spinner.set_message("Creating Vite app...");

    let create_status = Command::new("npm")
        .arg("create")
        .arg("vite@latest")
        .arg(&app_name)
        .arg("--")
        .arg("--template")
        .arg("react")
        .stdin(Stdio::null())
        .stdout(Stdio::null()) // suppress stdout
        .stderr(Stdio::null()) // suppress stderr
        .status();

    match create_status {
        Ok(code) if code.success() => {
            spinner.set_message("Installing dependencies...");
        }
        Ok(code) => {
            spinner.finish_and_clear();
            eprintln!("❌ App creation failed with exit code: {}", code);
            std::process::exit(1);
        }
        Err(err) => {
            spinner.finish_and_clear();
            eprintln!("❌ Failed to run Vite create command: {}", err);
            std::process::exit(1);
        }
    }

    let app_path = Path::new(&app_name);
    let install_status = Command::new("npm")
        .arg("install")
        .current_dir(app_path)
        .stdin(Stdio::null())
        .stdout(Stdio::null()) // suppress stdout
        .stderr(Stdio::null()) // suppress stderr
        .status();

    setup_tailwind(app_path, &spinner);

    spinner.finish_and_clear();

    match install_status {
        Ok(code) if code.success() => {
            println!("✅ React app '{}' created successfully!", app_name);
            println!("\n➡️  To get started:\n");
            println!("  cd {}\n  npm run dev\n", app_name);
        }
        Ok(code) => {
            eprintln!("❌ `npm install` failed with exit code: {}", code);
            std::process::exit(1);
        }
        Err(err) => {
            eprintln!("❌ Failed to run `npm install`: {}", err);
            std::process::exit(1);
        }
    }
}

