use hound;
use image::ImageReader;
use std::{
    io::{self},
    path::Path
};

pub fn extract_bits_from_bmp<P: AsRef<Path>>(
    path: P,
    positions: &[usize],
    bit_len: usize
) -> io::Result<Vec<u8>> {
    let flat = ImageReader::open(path)?.decode()
        .unwrap()
        .to_rgb8()
        .as_raw();
    let mut bits = Vec::with_capacity(bit_len);

    for &pos in positions.iter().take(bit_len) { bits.push(flat[pos] & 1); }

    Ok(bits_to_bytes(&bits))
}

pub fn extract_bits_from_wav<P: AsRef<Path>>(
    path: P,
    positions: &[usize],
    bit_len: usize
) -> io::Result<Vec<u8>> {
    let samples: Vec<i16> = hound::WavReader::open(path)
        .unwrap()
        .samples::<i16>()
        .map(|r| r.unwrap()).collect();
    let mut bits = Vec::with_capacity(bit_len);

    for &pos in positions.iter().take(bit_len) { bits.push((samples[pos] as u16 & 1) as u8); }

    Ok(bits_to_bytes(&bits))
}

fn bits_to_bytes(bits: &[u8]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity((bits.len() + 7) / 8);
    let mut current_byte = 0u8;
    for (i, &b) in bits.iter().enumerate() {
        current_byte = (current_byte << 1) | b;
        if i % 8 == 7 {
            bytes.push(current_byte);
            current_byte = 0;
        }
    }
    if bits.len() % 8 != 0 {
        current_byte <<= 8 - (bits.len() % 8);
        bytes.push(current_byte);
    }

    bytes
}
