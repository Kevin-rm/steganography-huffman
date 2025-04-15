use std::fs::File;
use std::path::Path;
use std::io::{self, BufReader};
use image::io::Reader as ImageReader;
use hound;
use crate::huffman::{Tree, Error};

/// Lit les bits LSB aux positions données dans un fichier BMP.
/// Retourne un Vec<u8> contenant les bits extraits (groupés par octets).
pub fn extract_bits_from_bmp<P: AsRef<Path>>(path: P, positions: &[usize], bit_len: usize) -> io::Result<Vec<u8>> {
    let img = ImageReader::open(path)?.decode()?.to_rgb8();
    let flat = img.as_raw();
    let mut bits = Vec::with_capacity(bit_len);
    for &pos in positions.iter().take(bit_len) {
        let byte = flat[pos];
        bits.push(byte & 1);
    }
    // Regroupe les bits en octets
    Ok(bits_to_bytes(&bits))
}

/// Lit les bits LSB aux positions données dans un fichier WAV.
/// Retourne un Vec<u8> contenant les bits extraits (groupés par octets).
pub fn extract_bits_from_wav<P: AsRef<Path>>(path: P, positions: &[usize], bit_len: usize) -> io::Result<Vec<u8>> {
    let mut reader = hound::WavReader::open(path)?;
    let samples: Vec<i16> = reader.samples::<i16>().map(|s| s.unwrap()).collect();
    let mut bits = Vec::with_capacity(bit_len);
    for &pos in positions.iter().take(bit_len) {
        let sample = samples[pos];
        bits.push((sample as u16 & 1) as u8);
    }
    Ok(bits_to_bytes(&bits))
}

/// Convertit un Vec<u8> de bits (0/1) en Vec<u8> d'octets.
fn bits_to_bytes(bits: &[u8]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity((bits.len() + 7) / 8);
    let mut cur = 0u8;
    for (i, &b) in bits.iter().enumerate() {
        cur = (cur << 1) | b;
        if i % 8 == 7 {
            bytes.push(cur);
            cur = 0;
        }
    }
    if bits.len() % 8 != 0 {
        cur <<= 8 - (bits.len() % 8);
        bytes.push(cur);
    }
    bytes
}

/// Décode le message caché à partir des octets extraits et d'un arbre de Huffman.
pub fn decode_hidden_message(tree: &Tree, data: &[u8], bit_len: usize) -> Result<String, Error> {
    tree.decode(data, bit_len)
}

// Exemple d'utilisation (à adapter selon votre main) :
/*
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let positions = vec![...]; // positions des bits modifiés
    let bit_len = ...; // nombre de bits du message caché
    let tree = ...; // votre arbre Huffman (chargé ou reconstruit)
    let data = extract_bits_from_bmp("image.bmp", &positions, bit_len)?;
    let message = decode_hidden_message(&tree, &data, bit_len)?;
    println!("Message caché : {}", message);
    Ok(())
}
*/
