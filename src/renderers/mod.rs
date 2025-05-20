pub mod ascii_art;
pub mod sixel;

pub trait QrRenderer {
    fn render(&self, qr: &qrcode::QrCode) -> std::io::Result<()>;
}
