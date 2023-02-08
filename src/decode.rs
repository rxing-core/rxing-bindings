#![deny(clippy::all)]

use std::collections::{HashMap, HashSet};
use std::path::Path;

use napi::bindgen_prelude::*;
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

#[napi]
pub fn decode(file_name: String, options: Option<DecodeOptions>) -> Option<Either<DecodeResult, Vec<DecodeResult>>> {
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

    if let Some(try_harder) = options.try_harder {
        hints.insert(DecodeHintType::TRY_HARDER, DecodeHintValue::TryHarder(try_harder));
    }

    if let Some(barcode_format) = options.barcode_format {
        let barcode_format: Vec<BarcodeFormat> = barcode_format.into_iter().map(|x| x.into()).collect();
        hints.insert(DecodeHintType::POSSIBLE_FORMATS, DecodeHintValue::PossibleFormats(HashSet::from_iter(
            barcode_format.iter().copied(),
        )));
    }

    let path = Path::new(&file_name);
    let extension = path.extension().unwrap_or_default();

    if options.decode_multi == Some(true) {
        let results = if extension == "svg" {
            rxing::helpers::detect_multiple_in_svg_with_hints(&file_name, &mut hints)
        } else {
            rxing::helpers::detect_multiple_in_file_with_hints(&file_name, &mut hints)
        };

        match results {
            Ok(results) => {
                let results: Vec<DecodeResult> = results.into_iter().map(|x| x.into()).collect();
                Some(Either::B(results))
            }
            Err(_search_err) => None,
        }
    } else {
        let result = if extension == "svg" {
            rxing::helpers::detect_in_svg_with_hints(&file_name, None, &mut hints)
        } else {
            rxing::helpers::detect_in_file_with_hints(&file_name, None, &mut hints)
        };

        match result {
            Ok(result) => {
                Some(Either::A(result.into()))
            }
            Err(_search_err) => None,
        }
    }
}
