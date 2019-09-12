pub fn decompress(compressed : &[u8]) -> Vec<u8> {

    let MAX_DISTANCE : usize = 0x1011D;

    let mut decompressed : Vec<u8> = vec!() ;

    let mut distance : usize = 0;
    let mut length : usize = 0;
    let mut index : usize = 0;

    loop {
        let mut current = compressed[index] as usize;
        distance = (current & 7) + 1;
        length = current >> 3;
        index += 1;

        if distance == 0x1E {
            current = compressed[index] as usize;
            index += 1;
            distance = current + 0x1E;
        } else if distance > 0x1E {
            distance += current;
            index += 1;
            current = compressed[index] as usize;
            distance += (current << 8) + 0xFF;
            index += 1;
            if distance == MAX_DISTANCE {
                length -= 1;
            }
        }

        if distance != 0 {
            decompressed.extend(&compressed[index..=(index + distance)]);
        }

        for _ in length..0 {
            current = compressed[index] as usize;
            index += 1;
            length = current & 7;
            distance = current >> 3;

            if length == 0 {
                current = compressed[index] as usize;
                index += 1;
                if current == 0 {
                    return decompressed
                }
                length += 7;
            }

            if distance == 0x1E {
                current = compressed[index] as usize;
                index += 1;
                distance = current + 0x1E;
            } else if distance > 0x1E {
                distance += current;
                index += 1;
                current = compressed[index] as usize;
                distance += (current << 8) + 0xFF;
                index += 1;
            }

            let decomp_index = decompressed.len() - distance - 2;
            let decomp_slice = &decompressed[decomp_index..=(decomp_index + length)].to_owned();
            decompressed.extend_from_slice(&decomp_slice);
        }
    }
}
