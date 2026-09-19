use clap::{Parser, Subcommand};
use std::error::Error;

pub mod words;

#[cfg(feature = "build-graphics")]
mod svg;

#[cfg(feature = "build-graphics")]
mod emblem_svg;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create cryptographic signature for artifact
    #[cfg(feature = "verify")]
    Sign {
        /// File path of the artifact to sign
        artifact: String,

        /// File path of signing (private!) key
        key: String,
    },

    /// Verify cryptographic signature for artifact
    #[cfg(feature = "verify")]
    Verify {
        /// File path of the artifact to verify
        artifact: String,
    },

    /// Generate emblem graphics (SVG and PNG)
    #[cfg(feature = "build-graphics")]
    BuildGraphics,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    match &args.command {
        #[cfg(feature = "verify")]
        Commands::Sign {
            artifact: _,
            key: _,
        } => todo!(),
        #[cfg(feature = "verify")]
        Commands::Verify { artifact } => cilki::signing::verify(artifact)?,
        #[cfg(feature = "build-graphics")]
        Commands::BuildGraphics => build_graphics()?,
    };
    Ok(())
}

#[cfg(feature = "build-graphics")]
fn build_graphics() -> Result<(), Box<dyn Error>> {
    use emblem_svg::EmblemRenderer;

    println!("\n╭─────────────────────────────────────╮");
    println!("│  Building Graphics Assets           │");
    println!("╰─────────────────────────────────────╯\n");

    for bg_style in [true, false] {
        for emblem in [
            #[cfg(feature = "project-goldboot")]
            &cilki::emblem::GOLDBOOT,
            #[cfg(feature = "project-attest")]
            &cilki::emblem::ATTEST,
            #[cfg(feature = "project-solder")]
            &cilki::emblem::SOLDER,
            #[cfg(feature = "project-sandpolis")]
            &cilki::emblem::SANDPOLIS,
            #[cfg(feature = "project-turbine")]
            &cilki::emblem::TURBINE,
            #[cfg(feature = "project-outpost")]
            &cilki::emblem::OUTPOST,
            #[cfg(feature = "project-workset")]
            &cilki::emblem::WORKSET,
            #[cfg(feature = "project-cibox")]
            &cilki::emblem::CIBOX,
            #[cfg(feature = "project-fossdb")]
            &cilki::emblem::FOSSDB,
            #[cfg(feature = "project-codemine")]
            &cilki::emblem::CODEMINE,
        ] {
            let renderer =
                EmblemRenderer::new(emblem, if bg_style { Some("fill:#333333") } else { None });

            let emblem_svg = renderer.to_svg()?;
            let bg_suffix = if bg_style { "-bg" } else { "" };
            let path = format!("emblems/{}{}.svg", emblem.name, bg_suffix);

            // Write svg only if changed
            if emblem_svg.write_to(&path)? {
                println!("  ✓ Generated {}", path);
            }

            // Write emblem rasters in varying sizes
            for (width, height) in [(256, 64), (512, 128), (1024, 256), (2048, 512)] {
                let path = format!("emblems/{}{}-{width}.png", emblem.name, bg_suffix);
                if emblem_svg.rasterize(&path, width, height)? {
                    println!("  ✓ Generated {} ({}x{})", path, width, height);
                }
            }
        }
    }

    Ok(())
}
