pub mod renderers;

use qrcode::QrCode;
use renderers::{ascii_art::AsciiArtRenderer, sixel::SixelRenderer, QrRenderer};

use std::io;

#[derive(Debug)]
pub enum RenderMode {
    AsciiArt,
    Sixel,
}

pub fn render_qr(text: &str, mode: RenderMode, invert: bool) -> io::Result<()> {
    let qr = QrCode::new(text.as_bytes()).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    match mode {
        RenderMode::AsciiArt => {
            let renderer = AsciiArtRenderer::new(invert);
            renderer.render(&qr)
        }
        RenderMode::Sixel => {
            let renderer = SixelRenderer::new(invert);
            renderer.render(&qr)
        }
    }
}

pub fn generate_qr_string(text: &str) -> io::Result<String> {
    let qr = QrCode::new(text.as_bytes()).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    Ok(qr
        .render::<char>()
        .quiet_zone(false)
        .module_dimensions(2, 1)
        .build())
}
