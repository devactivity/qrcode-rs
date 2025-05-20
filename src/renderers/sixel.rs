use super::QrRenderer;
use std::io::{self, Write};

pub struct SixelRenderer {
    invert: bool,
}

impl SixelRenderer {
    pub fn new(invert: bool) -> Self {
        Self { invert }
    }
}

impl QrRenderer for SixelRenderer {
    fn render(&self, qr: &qrcode::QrCode) -> std::io::Result<()> {
        let stdout = io::stdout();
        let mut writer = stdout.lock();
        let modules = qr.to_colors();

        write!(writer, "\x1BPq\"1;1")?;

        let (black_idx, white_idx) = if self.invert { ("1", "0") } else { ("0", "1") };

        write!(writer, "#{};2;0;0;0#{};2;100;100;100", black_idx, white_idx)?;

        let size = qr.width();
        let line = format!("#{white_idx}!{}~", (size + 2) * 6);

        // top border
        write!(writer, "{}-", line)?;

        // QR code content
        for y in 0..size {
            write!(writer, "#{white_idx}")?;

            let mut color = white_idx;
            let mut repeat = 6;

            for x in 0..size {
                let module = modules[y * size + x];
                let current = if (module == qrcode::Color::Dark) ^ self.invert {
                    black_idx
                } else {
                    white_idx
                };

                if current != color {
                    write!(writer, "#{color}!{repeat}~")?;
                    color = current;
                    repeat = 0;
                }
                repeat += 6;
            }

            if color == white_idx {
                write!(writer, "#{white_idx}!{}~", repeat + 6)?;
            } else {
                write!(writer, "#{color}!{repeat}~#{white_idx}!6~")?;
            }

            write!(writer, "-")?;
        }

        // bottom border end exit
        write!(writer, "{}\x1B\\", line)?;
        writer.flush()?;

        Ok(())
    }
}
