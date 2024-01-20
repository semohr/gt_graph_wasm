use ruzstd::frame::ReadFrameHeaderError;
use ruzstd::frame_decoder::FrameDecoderError;
use ruzstd::{BlockDecodingStrategy, FrameDecoder};
use std::io::Read;
use std::io::Seek;
use wasm_bindgen::JsValue;

use crate::log;

/* Decompress the buffer if it is compressed
*/
pub fn decodebuffer(input: &[u8]) -> Result<Vec<u8>, JsValue> {
    match &input[0..6] {
        //xz (.xz) format description, starts with 0xfd, 0x37, 0x7a, 0x58, 0x5a, 0x00
        &[0xfd, 0x37, 0x7a, 0x58, 0x5a, 0x00] => {
            console_log!("xz compression detected");
            Err("xz compression not supported".into())
        }
        // zstd (.zst) format description, starts with 0x28, 0xb5, 0x2f, 0xfd
        &[0x28, 0xb5, 0x2f, 0xfd, _, _] => {
            console_log!("zstd compression detected");
            Ok(decodebuffer_zstd(input))
        }
        //Gzip (.gz) format description, starts with 0x1f, 0x8b, 0x08
        &[0x1f, 0x8b, 0x08, _, _, _] => {
            console_log!("gz compression detected");
            Err("gz compression not supported".into())
        }
        //Zip (.zip) format description, starts with 0x50, 0x4b, 0x03, 0x04 (unless empty — then the last two are 0x05, 0x06 or 0x06, 0x06)
        &[0x50, 0x4b, 0x03, 0x04, _, _] => {
            console_log!("zip compression detected");
            Err("zip compression not supported".into())
        }
        _ => {
            console_log!("No compression detected");
            Ok(input.to_vec())
        }
    }
}

struct StateTracker {
    bytes_used: u64,
    frames_used: usize,
}

pub fn decodebuffer_zstd(input: &[u8]) -> Vec<u8> {
    let mut frame_dec = FrameDecoder::new();

    let mut tracker = StateTracker {
        bytes_used: 0,
        frames_used: 0,
    };

    let batch_size = 1024 * 1024 * 10;
    let mut result = Vec::with_capacity(input.len());

    let mut cursor = std::io::Cursor::new(input);

    while cursor.position() < input.len() as u64 {
        match frame_dec.reset(&mut cursor) {
            Err(FrameDecoderError::ReadFrameHeaderError(ReadFrameHeaderError::SkipFrame(
                _magic_num,
                skip_size,
            ))) => {
                cursor
                    .seek(std::io::SeekFrom::Current(skip_size as i64))
                    .unwrap();
                continue;
            }
            other => other.unwrap(),
        }

        tracker.frames_used += 1;

        while !frame_dec.is_finished() {
            frame_dec
                .decode_blocks(&mut cursor, BlockDecodingStrategy::UptoBytes(batch_size))
                .unwrap();

            if frame_dec.can_collect() > batch_size {
                let x = frame_dec.read_to_end(&mut result).unwrap();
                tracker.bytes_used += x as u64;
            }
        }

        // handle the last chunk of data
        while frame_dec.can_collect() > 0 {
            let x = frame_dec.read_to_end(&mut result).unwrap();
            tracker.bytes_used += x as u64;
        }
    }

    result
}
