use std::collections::HashMap;
use std::fs::File;
use std::io::{Cursor, Write};

use image::DynamicImage;
use napi::bindgen_prelude::Buffer;
use rxing::{EncodeHintType, EncodeHintValue, EncodingHintDictionary, MultiFormatWriter, Writer};

use crate::JsBarcodeFormat;

#[napi(object)]
#[derive(Default)]
pub struct EncodeOptions {
  pub barcode_format: Option<JsBarcodeFormat>,
  pub width: Option<u32>,
  pub height: Option<u32>,
  pub margin: Option<u32>,
  pub error_correction: Option<String>,
  pub character_set: Option<String>,
  pub data_matrix_compact: Option<bool>,
  pub pdf417_compact: Option<bool>,
  pub pdf417_compaction: Option<String>,
  pub pdf417_auto_eci: Option<bool>,
  pub aztec_layers: Option<i32>,
  pub qr_version: Option<String>,
  pub qr_mask_pattern: Option<String>,
  pub qr_compact: Option<bool>,
  pub gs1_format: Option<bool>,
  pub force_code_set: Option<String>,
  pub force_c40: Option<bool>,
  pub code128_compact: Option<bool>,
  pub output_file: Option<String>,
}

/**
 * Encode a barcode from a string, returning a buffer representing the image
 *
 * @param {string} data The data to encode
 * @param {EncodeOptions} [options] Optional options to pass to the encoder
 *
 * @returns {Buffer|null} A buffer representing the encoded barcode, or `null` if the barcode could not be encoded or encountered an error
 *
 * @example
 * const { encode } = require('@rxing/rxing');
 * const fs = require('fs');
 *
 * const data = 'Hello World!';
 * const buffer = encode(data);
 * fs.writeFileSync('hello-world.png', buffer);
 */
#[napi]
pub fn encode(data: String, options: Option<EncodeOptions>) -> Option<Buffer> {
  let options = options.unwrap_or_default();
  let mut hints: EncodingHintDictionary = HashMap::new();

  let barcode_format = options.barcode_format.unwrap_or(JsBarcodeFormat::QrCode);
  let width = options.width.unwrap_or(200);
  let height = options.height.unwrap_or_else(|| {
    if barcode_format == JsBarcodeFormat::QrCode {
      width
    } else {
      200
    }
  });
  let margin = options.margin.unwrap_or(0);

  hints.insert(
    EncodeHintType::MARGIN,
    EncodeHintValue::Margin(margin.to_string()),
  );

  if let Some(error_correction) = options.error_correction {
    hints.insert(
      EncodeHintType::ERROR_CORRECTION,
      EncodeHintValue::ErrorCorrection(error_correction),
    );
  }

  if let Some(character_set) = options.character_set {
    hints.insert(
      EncodeHintType::CHARACTER_SET,
      EncodeHintValue::CharacterSet(character_set),
    );
  }

  if let Some(data_matrix_compact) = options.data_matrix_compact {
    hints.insert(
      EncodeHintType::DATA_MATRIX_COMPACT,
      EncodeHintValue::DataMatrixCompact(data_matrix_compact),
    );
  }

  if let Some(pdf417_compact) = options.pdf417_compact {
    hints.insert(
      EncodeHintType::PDF417_COMPACT,
      EncodeHintValue::Pdf417Compact(pdf417_compact.to_string()),
    );
  }

  if let Some(pdf417_compaction) = options.pdf417_compaction {
    hints.insert(
      EncodeHintType::PDF417_COMPACTION,
      EncodeHintValue::Pdf417Compaction(pdf417_compaction),
    );
  }

  if let Some(pdf417_auto_eci) = options.pdf417_auto_eci {
    hints.insert(
      EncodeHintType::PDF417_AUTO_ECI,
      EncodeHintValue::Pdf417AutoEci(pdf417_auto_eci.to_string()),
    );
  }

  if let Some(aztec_layers) = options.aztec_layers {
    hints.insert(
      EncodeHintType::AZTEC_LAYERS,
      EncodeHintValue::AztecLayers(aztec_layers),
    );
  }

  if let Some(qr_version) = options.qr_version {
    hints.insert(
      EncodeHintType::QR_VERSION,
      EncodeHintValue::QrVersion(qr_version),
    );
  }

  if let Some(qr_mask_pattern) = options.qr_mask_pattern {
    hints.insert(
      EncodeHintType::QR_MASK_PATTERN,
      EncodeHintValue::QrMaskPattern(qr_mask_pattern),
    );
  }

  if let Some(qr_compact) = options.qr_compact {
    hints.insert(
      EncodeHintType::QR_COMPACT,
      EncodeHintValue::QrCompact(qr_compact.to_string()),
    );
  }

  if let Some(gs1_format) = options.gs1_format {
    hints.insert(
      EncodeHintType::GS1_FORMAT,
      EncodeHintValue::Gs1Format(gs1_format),
    );
  }

  if let Some(force_code_set) = options.force_code_set {
    hints.insert(
      EncodeHintType::FORCE_CODE_SET,
      EncodeHintValue::ForceCodeSet(force_code_set),
    );
  }

  if let Some(force_c40) = options.force_c40 {
    hints.insert(
      EncodeHintType::FORCE_C40,
      EncodeHintValue::ForceC40(force_c40),
    );
  }

  if let Some(code128_compact) = options.code128_compact {
    hints.insert(
      EncodeHintType::CODE128_COMPACT,
      EncodeHintValue::Code128Compact(code128_compact),
    );
  }

  let writer = MultiFormatWriter::default();
  if let Ok(bit_matrix) = writer.encode_with_hints(
    &data,
    &barcode_format.into(),
    width as i32,
    height as i32,
    &hints,
  ) {
    let image: DynamicImage = bit_matrix.into();
    let mut bytes: Vec<u8> = Vec::new();

    if image
      .write_to(
        &mut Cursor::new(&mut bytes),
        image::ImageOutputFormat::Jpeg(100),
      )
      .is_ok()
    {
      if let Some(file_path) = options.output_file {
        if write_to_file(&file_path, &bytes).is_ok() {
          Some(Buffer::from(bytes))
        } else {
          None
        }
      } else {
        Some(Buffer::from(bytes))
      }
    } else {
      None
    }
  } else {
    None
  }
}

fn write_to_file(file_path: &str, bytes: &[u8]) -> Result<(), std::io::Error> {
  let mut file = File::create(file_path)?;
  file.write_all(bytes)
}
