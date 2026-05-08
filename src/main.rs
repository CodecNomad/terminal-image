use anyhow::{Context, Result};
use inquire::Text;
use viuer::Config;

fn main() -> Result<()> {
    let url = Text::new("What's the url of the image?")
        .prompt()
        .context("Failed to ask image URL")?;

    let image_data = reqwest::blocking::get(url)
        .context("Failed to get image from URL")?
        .bytes()?;

    let image =
        image::load_from_memory(&image_data).context("Failed to construct image from raw bytes")?;

    let _ = clearscreen::clear();

    viuer::print(&image, &Config::default()).context("Failed to print image to terminal")?;

    Ok(())
}
