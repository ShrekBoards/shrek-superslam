use std::cmp::Ordering;

const MAX_DISTANCE: usize = 0x1011D;

/// Compress data to be compatible with Shrek SuperSlam
///
/// # Parameters
///
/// - `decompressed`: The data to compress
///
/// # Returns
///
/// The compressed data
///
/// # Notes
///
/// This method does not actually compress the data, but instead inserts
/// markers throughout the file to allow the game to read the uncompressed
/// stream. The returned data is therefore slightly larger than the input.
pub fn compress(decompressed: &[u8]) -> Vec<u8> {
    // Adapted from code provided by zed0
    // <https://reverseengineering.stackexchange.com/questions/16021/>
    let mut compressed: Vec<u8> = vec![];
    let mut index = 0;

    // Repeat as many max-length blocks as we can
    let mut remaining = decompressed.len() - index;
    while remaining > MAX_DISTANCE {
        compressed.extend(&[0xF8, 0xFF, 0xFF]);
        compressed.extend(&decompressed[index..index + MAX_DISTANCE]);
        index += MAX_DISTANCE;
        remaining = decompressed.len() - index;
    }

    // Add a final block with the remaining data
    if remaining > 0x11D {
        compressed.push(0xF8);
        let header = remaining - 0x11E;
        compressed.push((header & 0x000000FF) as u8);
        compressed.push((header >> 8) as u8);
    } else if remaining > 0x1D {
        compressed.push(0xF0);
        let header = remaining - 0x1E;
        compressed.push(header as u8);
    } else {
        let header = remaining << 3;
        compressed.push(header as u8);
    }
    compressed.extend(&decompressed[index..]);

    // Add the special case 0 length back reference to end the stream
    compressed.extend(&[0x00, 0x00]);

    compressed
}

/// Decompress compressed Shrek SuperSlam data
///
/// # Parameters
///
/// - `compressed`: The compressed data to extract
///
/// # Returns
///
/// The extracted data
pub fn decompress(compressed: &[u8]) -> Vec<u8> {
    let mut decompressed: Vec<u8> = vec![];
    let mut index: usize = 0;

    loop {
        let mut current = compressed[index] as usize;
        index += 1;
        let mut length = (current & 7) + 1;
        let mut distance = current >> 3;

        match distance.cmp(&0x1E) {
            Ordering::Equal => {
                current = compressed[index] as usize;
                index += 1;
                distance = current + 0x1E;
            }
            Ordering::Greater => {
                distance += compressed[index] as usize;
                index += 1;
                current = compressed[index] as usize;
                index += 1;
                distance += (current << 8) + 0xFF;
                if distance == MAX_DISTANCE {
                    length -= 1;
                }
            }
            _ => (),
        };

        if distance != 0 {
            decompressed.extend(&compressed[index..(index + distance)]);
            index += distance;
        }

        let bound = length;
        for _ in 0..bound {
            current = compressed[index] as usize;
            index += 1;
            length = current & 7;
            distance = current >> 3;

            if length == 0 {
                length = compressed[index] as usize;
                index += 1;
                if length == 0 {
                    return decompressed;
                }
                length += 7;
            }

            match distance.cmp(&0x1E) {
                Ordering::Equal => {
                    current = compressed[index] as usize;
                    index += 1;
                    distance = current + 0x1E;
                }
                Ordering::Greater => {
                    current = compressed[index] as usize;
                    distance += current;
                    index += 1;
                    current = compressed[index] as usize;
                    distance += (current << 8) + 0xFF;
                    index += 1;
                }
                _ => (),
            };

            for _ in 0..length {
                let backwards = decompressed[decompressed.len() - 1 - distance];
                decompressed.push(backwards);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn compress_then_decompress() {
        let data = "The quick brown fox jumped over the lazy dog";
        let compressed = compress(&data.as_bytes());
        let decompressed = decompress(&compressed);
        assert_eq!(String::from_utf8(decompressed).unwrap(), data);
    }
}
