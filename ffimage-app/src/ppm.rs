use crate::parser::*;

pub struct Ppm {
    pub width: u32,
    pub height: u32,
    pub range: u32,
    pub bytes: Vec<u8>,
}

pub fn read(bytes: impl IntoIterator<Item = u8>) -> Option<Result<Ppm, &'static str>> {
    let mut bytes = bytes.into_iter();

    // parse format from first line
    let mut magic = [0u8; 2];
    magic[0] = if let Some(byte) = bytes.next() {
        byte
    } else {
        return None;
    };
    magic[1] = if let Some(byte) = bytes.next() {
        byte
    } else {
        return None;
    };

    // is this a P6 PPM?
    if magic != *b"P6" {
        return None;
    }

    match bytes.next()? {
        b' ' | b'\n' => {}
        _ => return Some(Err("ppm: expected whitespace")),
    }

    // parse width
    let width = match parse_u32(&mut bytes)?.0 {
        Ok(val) => val,
        Err(_e) => return Some(Err("ppm: failed to parse width")),
    };

    // parse height
    let height = match parse_u32(&mut bytes)?.0 {
        Ok(val) => val,
        Err(_e) => return Some(Err("ppm: failed to parse height")),
    };

    // parse range
    let range = match parse_u32(&mut bytes)?.0 {
        Ok(val) => val,
        Err(_e) => return Some(Err("ppm: failed to parse range")),
    };

    if range > 255 {
        return Some(Err("ppm: cannot handle range: {range}"));
    }

    // take only as many bytes as we expect there to be in the image
    let ppm_len = (width * height * 3) as usize;
    let bytes: Vec<u8> = bytes.take(ppm_len).collect();

    // verify buffer length
    if bytes.len() != ppm_len {
        return Some(Err("ppm: invalid length"));
    }

    Some(Ok(Ppm {
        width: width as u32,
        height: height as u32,
        range: range as u32,
        bytes,
    }))
}
