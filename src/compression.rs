/// Decompress compressed Shrek SuperSlam data
///
/// \param compressed The compressed data to extract
///
/// \returns The extracted data
pub fn decompress(compressed : &[u8]) -> Vec<u8> {
    const MAX_DISTANCE : usize = 0x1011D;

    let mut decompressed : Vec<u8> = vec!() ;
    let mut index : usize = 0;

    loop {
        let mut current = compressed[index] as usize;
        index += 1;
        let mut length = (current & 7) + 1;
        let mut distance = current >> 3;

        if distance == 0x1E {
            current = compressed[index] as usize;
            index += 1;
            distance = current + 0x1E;
        } else if distance > 0x1E {
            distance += compressed[index] as usize;
            index += 1;
            current = compressed[index] as usize;
            index += 1;
            distance += (current << 8) + 0xFF;
            if distance == MAX_DISTANCE {
                length -= 1;
            }
        }

        if distance != 0 {
            decompressed.extend(&compressed[index..(index + distance)]);
            index += distance;
        }

        for _ in 0..length {
            current = compressed[index] as usize;
            index += 1;
            length = current & 7;
            distance = current >> 3;

            if length == 0 {
                length = compressed[index] as usize;
                index += 1;
                if length == 0 {
                    return decompressed
                }
                length += 7;
            }

            if distance == 0x1E {
                current = compressed[index] as usize;
                index += 1;
                distance = current + 0x1E;
            } else if distance > 0x1E {
                current = compressed[index] as usize;
                distance += current;
                index += 1;
                current = compressed[index] as usize;
                distance += (current << 8) + 0xFF;
                index += 1;
            }

            for _ in 0..length {
                let backwards = decompressed[decompressed.len() - 1 - distance];
                decompressed.push(backwards);
            }
        }
    }
}
