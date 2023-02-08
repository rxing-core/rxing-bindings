#![deny(clippy::all)]

use napi::bindgen_prelude::*;
use rxing::BarcodeFormat;

#[napi(js_name = "BarcodeFormat")]
pub enum JsBarcodeFormat {
    /** Aztec 2D barcode format. */
    AZTEC,

    /** CODABAR 1D format. */
    CODABAR,

    /** Code 39 1D format. */
    Code39,

    /** Code 93 1D format. */
    Code93,

    /** Code 128 1D format. */
    Code128,

    /** Data Matrix 2D barcode format. */
    DataMatrix,

    /** EAN-8 1D format. */
    Ean8,

    /** EAN-13 1D format. */
    Ean13,

    /** ITF (Interleaved Two of Five) 1D format. */
    ITF,

    /** MaxiCode 2D barcode format. */
    MAXICODE,

    /** PDF417 format. */
    Pdf417,

    /** QR Code 2D barcode format. */
    QrCode,

    /** RSS 14 */
    Rss14,

    /** RSS EXPANDED */
    RssExpanded,

    /** UPC-A 1D format. */
    UpcA,

    /** UPC-E 1D format. */
    UpcE,

    /** UPC/EAN extension format. Not a stand-alone format. */
    UpcEanExtension,

    ///
    UnsupportedFormat,
}

impl From<JsBarcodeFormat> for BarcodeFormat {
    fn from(value: JsBarcodeFormat) -> Self {
        match value {
            JsBarcodeFormat::AZTEC => BarcodeFormat::AZTEC,
            JsBarcodeFormat::CODABAR => BarcodeFormat::CODABAR,
            JsBarcodeFormat::Code39 => BarcodeFormat::CODE_39,
            JsBarcodeFormat::Code93 => BarcodeFormat::CODE_93,
            JsBarcodeFormat::Code128 => BarcodeFormat::CODE_128,
            JsBarcodeFormat::DataMatrix => BarcodeFormat::DATA_MATRIX,
            JsBarcodeFormat::Ean8 => BarcodeFormat::EAN_8,
            JsBarcodeFormat::Ean13 => BarcodeFormat::EAN_13,
            JsBarcodeFormat::ITF => BarcodeFormat::ITF,
            JsBarcodeFormat::MAXICODE => BarcodeFormat::MAXICODE,
            JsBarcodeFormat::Pdf417 => BarcodeFormat::PDF_417,
            JsBarcodeFormat::QrCode => BarcodeFormat::QR_CODE,
            JsBarcodeFormat::Rss14 => BarcodeFormat::RSS_14,
            JsBarcodeFormat::RssExpanded => BarcodeFormat::RSS_EXPANDED,
            JsBarcodeFormat::UpcA => BarcodeFormat::UPC_A,
            JsBarcodeFormat::UpcE => BarcodeFormat::UPC_E,
            JsBarcodeFormat::UpcEanExtension => BarcodeFormat::UPC_EAN_EXTENSION,
            JsBarcodeFormat::UnsupportedFormat => BarcodeFormat::UNSUPORTED_FORMAT,
        }
    }
}

impl From<BarcodeFormat> for JsBarcodeFormat {
    fn from(value: BarcodeFormat) -> Self {
        match value {
            BarcodeFormat::AZTEC => JsBarcodeFormat::AZTEC,
            BarcodeFormat::CODABAR => JsBarcodeFormat::CODABAR,
            BarcodeFormat::CODE_39 => JsBarcodeFormat::Code39,
            BarcodeFormat::CODE_93 => JsBarcodeFormat::Code93,
            BarcodeFormat::CODE_128 => JsBarcodeFormat::Code128,
            BarcodeFormat::DATA_MATRIX => JsBarcodeFormat::DataMatrix,
            BarcodeFormat::EAN_8 => JsBarcodeFormat::Ean8,
            BarcodeFormat::EAN_13 => JsBarcodeFormat::Ean13,
            BarcodeFormat::ITF => JsBarcodeFormat::ITF,
            BarcodeFormat::MAXICODE => JsBarcodeFormat::MAXICODE,
            BarcodeFormat::PDF_417 => JsBarcodeFormat::Pdf417,
            BarcodeFormat::QR_CODE => JsBarcodeFormat::QrCode,
            BarcodeFormat::RSS_14 => JsBarcodeFormat::Rss14,
            BarcodeFormat::RSS_EXPANDED => JsBarcodeFormat::RssExpanded,
            BarcodeFormat::UPC_A => JsBarcodeFormat::UpcA,
            BarcodeFormat::UPC_E => JsBarcodeFormat::UpcE,
            BarcodeFormat::UPC_EAN_EXTENSION => JsBarcodeFormat::UpcEanExtension,
            BarcodeFormat::UNSUPORTED_FORMAT => JsBarcodeFormat::UnsupportedFormat,
        }
    }
}
