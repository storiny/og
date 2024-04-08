use lazy_static::lazy_static;
use rusttype::Font;

lazy_static! {
    // Cabinet Grotesk
    pub static ref CABINET_GROTESK_BOLD: Font<'static> =
        #[allow(clippy::expect_used)]
        Font::try_from_bytes(include_bytes!("../../fonts/CabinetGrotesk/CabinetGrotesk-Bold.ttf") as &[u8])
            .expect("error loading the `CabinetGrotesk-Bold.ttf` font file");
    //
    pub static ref CABINET_GROTESK_EXTRABOLD: Font<'static> =
        #[allow(clippy::expect_used)]
        Font::try_from_bytes(include_bytes!("../../fonts/CabinetGrotesk/CabinetGrotesk-Extrabold.ttf") as &[u8])
            .expect("error loading the `CabinetGrotesk-Extrabold.ttf` font file");
    //
    pub static ref CABINET_GROTESK_MEDIUM: Font<'static> =
        #[allow(clippy::expect_used)]
        Font::try_from_bytes(include_bytes!("../../fonts/CabinetGrotesk/CabinetGrotesk-Medium.ttf") as &[u8])
            .expect("error loading the `CabinetGrotesk-Medium.ttf` font file");
    //
    pub static ref CABINET_GROTESK_REGULAR: Font<'static> =
        #[allow(clippy::expect_used)]
        Font::try_from_bytes(include_bytes!("../../fonts/CabinetGrotesk/CabinetGrotesk-Regular.ttf") as &[u8])
            .expect("error loading the `CabinetGrotesk-Regular.ttf` font file");
    // Satoshi
    pub static ref SATOSHI_BOLD: Font<'static> =
        #[allow(clippy::expect_used)]
        Font::try_from_bytes(include_bytes!("../../fonts/Satoshi/Satoshi-Bold.ttf") as &[u8])
            .expect("error loading the `Satoshi-Bold.ttf` font file");
    //
    pub static ref SATOSHI_MEDIUM: Font<'static> =
        #[allow(clippy::expect_used)]
        Font::try_from_bytes(include_bytes!("../../fonts/Satoshi/Satoshi-Medium.ttf") as &[u8])
            .expect("error loading the `Satoshi-Medium.ttf` font file");
    //
    pub static ref SATOSHI_REGULAR: Font<'static> =
        #[allow(clippy::expect_used)]
        Font::try_from_bytes(include_bytes!("../../fonts/Satoshi/Satoshi-Regular.ttf") as &[u8])
            .expect("error loading the `Satoshi-Regular.ttf` font file");
}
