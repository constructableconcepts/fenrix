use anyhow::{Context, Result};
use axum::{routing::get_service, Router};
use clap::{Parser, Subcommand};
use std::fs;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::process::Command;
use tower_http::services::ServeDir;
use walkdir::WalkDir;

/// A CLI for the Fenrix web framework.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Creates a new Fenrix project from a template.
    New {
        /// The name of the project to create.
        name: String,
    },
    /// Runs a development server.
    Dev,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::New { name } => {
            create_new_project(name)?;
        }
        Commands::Dev => {
            run_dev_server().await?;
        }
    }

    Ok(())
}

/// Runs the wasm-pack build command and starts a static file server.
async fn run_dev_server() -> Result<()> {
    println!("Building the wasm package...");
    let build_status = Command::new("wasm-pack")
        .arg("build")
        .arg("--target")
        .arg("web")
        .status()
        .context("Failed to execute 'wasm-pack build'. Make sure wasm-pack is installed and in your PATH.")?;

    if !build_status.success() {
        anyhow::bail!("'wasm-pack build' command failed.");
    }
    println!("Build successful.");

    println!("Starting the development server...");
    // Serve the project root and the /pkg directory
    let app = Router::new()
        .nest_service("/pkg", ServeDir::new("pkg"))
        .fallback_service(ServeDir::new("."));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("View your app at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// Creates a new project directory and populates it from the template.
fn create_new_project(name: &str) -> Result<()> {
    let project_path = PathBuf::from(name);
    if project_path.exists() {
        anyhow::bail!("Directory '{}' already exists.", name);
    }
    fs::create_dir_all(&project_path).context(format!("Failed to create project directory '{}'", name))?;

    let template_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("templates");

    println!("Creating project '{}' from template...", name);

    for entry in WalkDir::new(&template_path) {
        let entry = entry.context("Failed to read template directory entry")?;
        let src_path = entry.path();
        let relative_path = src_path.strip_prefix(&template_path).unwrap();
        let dest_path = project_path.join(relative_path);

        if src_path.is_dir() {
            fs::create_dir_all(&dest_path).context(format!("Failed to create directory '{:?}'", dest_path))?;
        } else if src_path.is_file() {
            let mut content = fs::read_to_string(src_path)
                .context(format!("Failed to read template file '{:?}'", src_path))?;

            content = content.replace("{{project-name}}", name);

            let final_dest_path = PathBuf::from(dest_path.to_string_lossy().replace(".template", ""));
            fs::write(&final_dest_path, content)
                .context(format!("Failed to write file '{:?}'", final_dest_path))?;
        }
    }

    println!("Project '{}' created successfully!", name);
    println!("To get started, run:");
    println!("  cd {}", name);
    println!("  cargo fenrix dev");

    Ok(())
}