use super::QrRenderer;
use ansi_term::Colour::{Black, White};
use std::io::{self, Write};

pub struct AsciiArtRenderer {
    invert: bool,
}

impl AsciiArtRenderer {
    pub fn new(invert: bool) -> Self {
        Self { invert }
    }
}

impl QrRenderer for AsciiArtRenderer {
    fn render(&self, qr: &qrcode::QrCode) -> std::io::Result<()> {
        let stdout = io::stdout();
        let mut writer = stdout.lock();

        let (fg, bg) = if self.invert {
            (White, Black)
        } else {
            (Black, White)
        };

        let size = qr.width();
        let modules = qr.to_colors();
        let border = bg.paint(" ".repeat(size * 2 + 4));

        writeln!(writer, "{}", border)?;

        for y in 0..size {
            write!(writer, "{}", bg.paint("  "))?;

            for x in 0..size {
                let module = modules[y * size + x];
                let color = if (module == qrcode::Color::Dark) ^ self.invert {
                    fg
                } else {
                    bg
                };

                write!(writer, "{}", color.paint("  "))?;
            }

            writeln!(writer, "{}", bg.paint("  "))?;
        }

        writeln!(writer, "{}", border)?;
        writer.flush()?;

        Ok(())
    }
}
