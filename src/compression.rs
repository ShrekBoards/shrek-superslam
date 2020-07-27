/// Decompress compressed Shrek SuperSlam data
///
/// \param compressed The compressed data to extract
///
/// \returns The extracted data
pub fn decompress(compressed : &[u8]) -> Vec<u8> {
    const MAX_DISTANCE : usize = 0x1011D;

    let mut decompressed : Vec<u8> = vec!() ;

    let mut distance : usize = 0;
    let mut length : usize = 0;
    let mut index : usize = 0;

    loop {
        let mut current = compressed[index] as usize;

        //println!("| first byte: 0x{:x}", current);
        //println!("[1] current: 0x{:x}, index: {}, bindex: {}, length: {}, distance: {}", compressed[index], index, decompressed.len(), length, distance);

        length = (current & 7) + 1;
        distance = current >> 3;
        index += 1;

        //println!("[2] current: 0x{:x}, index: {}, bindex: {}, length: {}, distance: {}", compressed[index], index, decompressed.len(), length, distance);

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

        //println!("[3] current: 0x{:x}, index: {}, bindex: {}, length: {}, distance: {}", compressed[index], index, decompressed.len(), length, distance);

        if distance != 0 {
            //println!("writing {} bytes: {:#?}", distance, &compressed[index..(index + distance)]);
            decompressed.extend(&compressed[index..(index + distance)]);
            index += distance;
        }

        //println!("[4] current: 0x{:x}, index: {}, bindex: {}, length: {}, distance: {}", compressed[index], index, decompressed.len(), length, distance);

        for _ in 0..length {
            current = compressed[index] as usize;
            index += 1;
            length = current & 7;
            distance = current >> 3;

            //println!("[5] current: 0x{:x}, index: {}, bindex: {}, length: {}, distance: {}", compressed[index], index, decompressed.len(), length, distance);

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

            //println!("[6] current: 0x{:x}, index: {}, bindex: {}, length: {}, distance: {}", compressed[index], index, decompressed.len(), length, distance);

            for _ in 0..length {
                let backwards = decompressed[decompressed.len() - 1 - distance];
                decompressed.push(backwards);
            }
            /*
            if (decompressed.len() + 1) > distance {
                let decomp_index = decompressed.len() - distance - 1;
                //println!("writing from backwards stream from index {}: [", decomp_index);
                //for i in decomp_index..(decomp_index + length) {
                //    let n = decompressed[i];
                //    println!("    {}", n);
                //    decompressed.push(n);
                //}
                //println!("]");
            }
            */
        }

	    //println!("---");
    }
}
