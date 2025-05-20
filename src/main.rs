use clap::Parser;
use qrcode_rs::{generate_qr_string, render_qr, RenderMode};

use std::io::{self, Read};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Text to encode in QR code
    #[arg(default_value = None)]
    text: Option<String>,

    /// invert colors
    #[arg(short, long)]
    invert: bool,

    /// simple string opt
    #[arg(short, long)]
    simple: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    // get text from argument or stdin
    let text = match args.text {
        Some(text) => text,
        None => {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)?;
            buffer
        }
    };

    if text.is_empty() {
        eprintln!("error: no input text provided");
        std::process::exit(1);
    }

    if args.simple {
        // use simple string output
        let qr_string = generate_qr_string(&text)?;
        println!("{qr_string}");
    } else {
        // use colored rendering
        let mode = if has_sixel_support() {
            RenderMode::Sixel
        } else {
            RenderMode::AsciiArt
        };

        if let Err(e) = render_qr(&text, mode, args.invert) {
            eprintln!("error generating QR code: {e}");
            std::process::exit(1);
        }
    }

    Ok(())
}

fn has_sixel_support() -> bool {
    std::env::var("TERM")
        .map(|term| term.contains("sixel"))
        .unwrap_or(false)
}
