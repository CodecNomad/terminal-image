use anyhow::{Context, Result};
use clap::Parser;
use inquire::Text;
use viuer::Config;

#[derive(Parser)]
#[command(name = "terminal-image")]
#[command(version = "1.0")]
#[command(about = "Draws an image from a http/s link into your terminal", long_about = None)]
struct Cli {
    #[arg(long)]
    url: Option<String>,
}

fn parse_cli() -> Result<String> {
    let parsed_cli = Cli::parse();

    parsed_cli.url.map_or_else(
        || {
            Text::new("What's the URL of the image?")
                .prompt()
                .context("Failed to ask image URL")
        },
        Ok,
    )
}

fn main() -> Result<()> {
    let url = parse_cli()?;

    let image_data = reqwest::blocking::get(url)
        .context("Failed to get image from URL")?
        .bytes()?;

    let image =
        image::load_from_memory(&image_data).context("Failed to construct image from raw bytes")?;

    let _ = clearscreen::clear();

    viuer::print(&image, &Config::default()).context("Failed to print image to terminal")?;

    Ok(())
}
