pub struct Ppm {
    pub width: u32,
    pub height: u32,
    pub range: u32,
    pub bytes: Vec<u8>,
}

pub fn read(bytes: impl IntoIterator<Item = u8>) -> Result<Ppm, &'static str> {
    let mut bytes = bytes.into_iter();

    // parse format from first line
    let mut magic = [0u8; 2];
    magic[0] = if let Some(byte) = bytes.next() {
        byte
    } else {
        return Err("ppm: not enough bytes");
    };
    magic[1] = if let Some(byte) = bytes.next() {
        byte
    } else {
        return Err("ppm: not enough bytes");
    };

    // is this a P6 PPM?
    if magic != *b"P6" {
        return Err("ppm: cannot handle magic");
    }

    fn real_bytes(iter: &mut impl Iterator<Item = u8>, limit: usize) -> Vec<u8> {
        let mut bytes = Vec::new();
        for byte in iter {
            if bytes.len() == limit {
                break;
            }

            if byte == b' ' || byte == b'\n' {
                if !bytes.is_empty() {
                    break;
                }
            } else {
                bytes.push(byte);
            }
        }
        bytes
    }

    // parse width
    let width_bytes = real_bytes(&mut bytes, 10);
    let width = std::str::from_utf8(&width_bytes)
        .expect("bytes should contain ASCII data")
        .parse::<usize>()
        .expect("value should be integer");

    // parse height
    let height_bytes = real_bytes(&mut bytes, 10);
    let height = std::str::from_utf8(&height_bytes)
        .expect("bytes should contain ASCII data")
        .parse::<usize>()
        .expect("value should be integer");

    // parse range
    let range_bytes = real_bytes(&mut bytes, 10);
    let range = std::str::from_utf8(&range_bytes)
        .expect("bytes should contain ASCII data")
        .parse::<usize>()
        .expect("value should be integer");

    if range > 255 {
        return Err("ppm: cannot handle range: {range}");
    }

    // take only as many bytes as we expect there to be in the image
    let ppm_len = width * height * 3;
    let bytes: Vec<u8> = bytes.take(ppm_len).collect();

    // verify buffer length
    if bytes.len() != width * height * 3 {
        return Err("ppm: invalid length");
    }

    Ok(Ppm {
        width: width as u32,
        height: height as u32,
        range: range as u32,
        bytes,
    })
}
