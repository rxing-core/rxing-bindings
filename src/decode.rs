#![deny(clippy::all)]

use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::str;

use base64::Engine;
use base64::engine::general_purpose;
use data_url::DataUrl;
use napi::bindgen_prelude::Either;
use rxing::{BarcodeFormat, DecodeHintType, DecodeHintValue, DecodingHintDictionary, RXingResult};

use crate::barcode_format::JsBarcodeFormat;

#[napi(object)]
#[derive(Default)]
pub struct DecodeOptions {
    pub try_harder: Option<bool>,
    pub decode_multi: Option<bool>,
    pub barcode_format: Option<Vec<JsBarcodeFormat>>,
    pub pure_barcode: Option<bool>,
    pub character_set: Option<String>,
    pub allowed_lengths: Option<Vec<u32>>,
    pub assume_code39_check_digit: Option<bool>,
    pub assume_gs1: Option<bool>,
    pub return_codabar_start_end: Option<bool>,
    pub allowed_ean_extensions: Option<Vec<u32>>,
    pub also_inverted: Option<bool>,
    pub other: Option<String>,
}

#[napi(object)]
pub struct DecodeResult {
    pub text: String,
    pub raw_bytes: Vec<u8>,
    pub num_bits: u32,
    pub format: JsBarcodeFormat,
}

impl From<RXingResult> for DecodeResult {
    fn from(value: RXingResult) -> Self {
        DecodeResult {
            text: value.getText().to_string(),
            raw_bytes: value.getRawBytes().to_vec(),
            num_bits: value.getNumBits() as u32,
            format: (*value.getBarcodeFormat()).into(),
        }
    }
}

/**
 * Decode a barcode from a file or base64 string
 *
 * @param {string} input Either a path to a file or a base64 string
 * @param {DecodeOptions} [options] Optional options to pass to the decoder
 *
 * @returns {DecodeResult|Array<DecodeResult>|null} The decode result or a list of decode results if `options.decodeMulti` is set to `true`, or `null` if the barcode could not be decoded or encountered an error
 *
 * @example
 * const { decode } = require('@rxing/rxing');
 * const result = decode('path/to/file.png');
 * console.log(result.text);
 * // Or
 * const result = decode('data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAA');
 * console.log(result.text);
 */
#[napi]
pub fn decode(input: String, options: Option<DecodeOptions>) -> Option<Either<DecodeResult, Vec<DecodeResult>>> {
    let options = options.unwrap_or_default();
    let mut hints: DecodingHintDictionary = HashMap::new();

    if let Some(other) = options.other {
        hints.insert(DecodeHintType::OTHER, DecodeHintValue::Other(other));
    }

    if let Some(pure_barcode) = options.pure_barcode {
        hints.insert(DecodeHintType::PURE_BARCODE, DecodeHintValue::PureBarcode(pure_barcode));
    }

    if let Some(character_set) = options.character_set {
        hints.insert(DecodeHintType::CHARACTER_SET, DecodeHintValue::CharacterSet(character_set));
    }

    if let Some(allowed_lengths) = options.allowed_lengths {
        hints.insert(DecodeHintType::ALLOWED_LENGTHS, DecodeHintValue::AllowedLengths(allowed_lengths));
    }

    if let Some(assume_code39_check_digit) = options.assume_code39_check_digit {
        hints.insert(DecodeHintType::ASSUME_CODE_39_CHECK_DIGIT, DecodeHintValue::AssumeCode39CheckDigit(assume_code39_check_digit));
    }

    if let Some(assume_gs1) = options.assume_gs1 {
        hints.insert(DecodeHintType::ASSUME_GS1, DecodeHintValue::AssumeGs1(assume_gs1));
    }

    if let Some(return_codabar_start_end) = options.return_codabar_start_end {
        hints.insert(DecodeHintType::RETURN_CODABAR_START_END, DecodeHintValue::ReturnCodabarStartEnd(return_codabar_start_end));
    }

    if let Some(allowed_ean_extensions) = options.allowed_ean_extensions {
        hints.insert(DecodeHintType::ALLOWED_EAN_EXTENSIONS, DecodeHintValue::AllowedEanExtensions(allowed_ean_extensions));
    }

    if let Some(also_inverted) = options.also_inverted {
        hints.insert(DecodeHintType::ALSO_INVERTED, DecodeHintValue::AlsoInverted(also_inverted));
    }

    // Default to true if not specified
    let try_harder = options.try_harder.unwrap_or(true);
    hints.insert(DecodeHintType::TRY_HARDER, DecodeHintValue::TryHarder(try_harder));

    if let Some(barcode_format) = options.barcode_format {
        let barcode_format: Vec<BarcodeFormat> = barcode_format.into_iter().map(|x| x.into()).collect();
        hints.insert(DecodeHintType::POSSIBLE_FORMATS, DecodeHintValue::PossibleFormats(HashSet::from_iter(
            barcode_format.iter().copied(),
        )));
    }

    let decode_multi = options.decode_multi.unwrap_or(false);
    match get_input(&input) {
        Either::A(input_file) => {
            detect_in_file(input_file, decode_multi, &mut hints)
        }
        Either::B(luma_tuple) => {
            detect_in_luma(luma_tuple, decode_multi, &mut hints)
        }
    }
}

fn get_input(input: &str) -> Either<&str, (Vec<u8>, u32, u32)> {
    match DataUrl::process(input) {
        Ok(data_url) => {
            if let Ok((body, _)) = data_url.decode_to_vec() {
                Either::B(create_luma_image(&body))
            } else {
                Either::A(input)
            }
        }
        Err(_) => { // invalid data url
            if let Ok(bytes) = general_purpose::STANDARD.decode(input.as_bytes()) {
                Either::B(create_luma_image(&bytes))
            } else {
                Either::A(input)
            }
        }
    }
}

fn create_luma_image(bytes: &[u8]) -> (Vec<u8>, u32, u32) {
    let image = image::load_from_memory(bytes).unwrap();
    let image = image.to_luma8();
    let (width, height) = image.dimensions();
    let image = image.into_raw();

    (image, width, height)
}

fn detect_in_file(input_file: &str, decode_multi: bool, hints: &mut DecodingHintDictionary) -> Option<Either<DecodeResult, Vec<DecodeResult>>> {
    let path = Path::new(&input_file);
    let extension = path.extension().unwrap_or_default();

    if decode_multi {
        let result = if extension == "svg" {
            rxing::helpers::detect_multiple_in_svg_with_hints(input_file, hints)
        } else {
            rxing::helpers::detect_multiple_in_file_with_hints(input_file, hints)
        };
        process_multi_result(result)
    } else {
        let result = if extension == "svg" {
            rxing::helpers::detect_in_svg_with_hints(input_file, None, hints)
        } else {
            rxing::helpers::detect_in_file_with_hints(input_file, None, hints)
        };

        if let Ok(result) = result {
            Some(Either::A(result.into()))
        } else {
            None
        }
    }
}

fn detect_in_luma(luma_tuple: (Vec<u8>, u32, u32), decode_multi: bool, hints: &mut DecodingHintDictionary) -> Option<Either<DecodeResult, Vec<DecodeResult>>> {
    if decode_multi {
        let result = rxing::helpers::detect_multiple_in_luma_with_hints(luma_tuple.0, luma_tuple.1, luma_tuple.2, hints);
        process_multi_result(result)
    } else {
        let result = rxing::helpers::detect_in_luma_with_hints(luma_tuple.0, luma_tuple.1, luma_tuple.2, None, hints);

        if let Ok(result) = result {
            Some(Either::A(result.into()))
        } else {
            None
        }
    }
}

fn process_multi_result<E>(results: Result<Vec<RXingResult>, E>) -> Option<Either<DecodeResult, Vec<DecodeResult>>> {
    if let Ok(results) = results {
        let results: Vec<DecodeResult> = results.into_iter().map(|x| x.into()).collect();
        Some(Either::B(results))
    } else {
        None
    }
}
