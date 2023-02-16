use crate::parser::*;

#[derive(Debug, Clone)]
pub struct Y4m {
    pub width: u32,
    pub height: u32,
    pub framerate: (u32, u32),
    pub interlacing: char,
    pub aspect_ratio: (u32, u32),
    pub color_space: ColorSpace,
    pub bytes: Vec<u8>,
}

pub fn read(bytes: impl IntoIterator<Item = u8>) -> Option<Result<Y4m, &'static str>> {
    let mut bytes = bytes.into_iter();

    // check signature
    let mut signature = [0u8; 10];
    for byte in &mut signature {
        *byte = match bytes.next() {
            Some(byte) => byte,
            None => return Some(Err("y4m: no signature")),
        }
    }
    if signature != *b"YUV4MPEG2 " {
        return Some(Err("y4m: invalid signature"));
    }

    let mut y4m = Y4m {
        width: 0,
        height: 0,
        framerate: (0, 0),
        interlacing: ' ',
        aspect_ratio: (0, 0),
        color_space: ColorSpace::C420jpeg,
        bytes: Vec::new(),
    };

    // parse parameters
    loop {
        let (res, other) = parse_params(&mut bytes)?;
        let param = match res {
            Ok(param) => param,
            Err(e) => return Some(Err(e)),
        };
        match param {
            Param::Width(width) => y4m.width = width,
            Param::Height(height) => y4m.height = height,
            Param::Framerate((num, denom)) => y4m.framerate = (num, denom),
            Param::Interlacing(interlacing) => y4m.interlacing = interlacing,
            Param::AspectRatio(ratio) => y4m.aspect_ratio = ratio,
            Param::ColorSpace(colorspace) => y4m.color_space = colorspace,
            Param::Unknown(word) => println!("y4m: unknown tag: {}", word),
        }

        if other == b'\n' {
            break;
        }
    }

    // parse frame
    let mut marker = [0u8; 5];
    for byte in &mut marker {
        *byte = match bytes.next() {
            Some(byte) => byte,
            None => return Some(Err("y4m: missing frame marker")),
        }
    }
    if marker != *b"FRAME" {
        return Some(Err("y4m: invalid frame marker"));
    }

    // check for frame parameters
    match bytes.next() {
        Some(b' ') => todo!("y4m: parse frame parameters"),
        Some(b'\n') => {}
        Some(_) => return Some(Err("y4m: unexpected byte after frame marker")),
        None => return Some(Err("y4m: missing frame marker")),
    };

    // take only as many bytes as we expect there to be in the image
    let y4m_len = (y4m.width * y4m.height * y4m.color_space.bpp() / 8) as usize;
    y4m.bytes = bytes.take(y4m_len).collect();

    // verify buffer length
    if y4m.bytes.len() != y4m_len {
        return Some(Err("y4m: unexpected EOF"));
    }

    Some(Ok(y4m))
}

#[derive(Debug, Clone)]
pub enum Param {
    Unknown(String),
    Width(u32),
    Height(u32),
    Framerate((u32, u32)),
    Interlacing(char),
    AspectRatio((u32, u32)),
    ColorSpace(ColorSpace),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ColorSpace {
    C420jpeg,
    C420paldv,
    C420,
    C422,
    C444,
    Cmono,
}

impl ColorSpace {
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "420jpeg" => Some(ColorSpace::C420jpeg),
            "420paldv" => Some(ColorSpace::C420paldv),
            "420" => Some(ColorSpace::C420),
            "422" => Some(ColorSpace::C422),
            "444" => Some(ColorSpace::C444),
            "mono" => Some(ColorSpace::Cmono),
            _ => None,
        }
    }

    pub fn bpp(&self) -> u32 {
        match self {
            ColorSpace::C420jpeg | ColorSpace::C420paldv | ColorSpace::C420 => 12,
            ColorSpace::C422 => 16,
            ColorSpace::C444 => 24,
            ColorSpace::Cmono => 8,
        }
    }
}

pub fn parse_params(
    bytes: &mut impl Iterator<Item = u8>,
) -> Option<(Result<Param, &'static str>, u8)> {
    fn parse_fraction(
        bytes: &mut impl Iterator<Item = u8>,
    ) -> Option<(Result<(u32, u32), &'static str>, u8)> {
        let (res, other) = parse_u32(bytes)?;
        let num = match res {
            Ok(num) => num,
            Err(_e) => return Some((Err("y4m: faild to parse integer"), other)),
        };
        if other != b':' {
            return Some((Err("y4m: expected fraction delimiter (:)"), other));
        }
        let (res, other) = parse_u32(bytes)?;
        let denom = match res {
            Ok(num) => num,
            Err(_e) => return Some((Err("y4m: faild to parse integer"), other)),
        };

        Some((Ok((num, denom)), other))
    }

    let (param, other) = match bytes.next()? {
        b'W' => {
            let (res, other) = parse_u32(bytes)?;
            let val = match res {
                Ok(val) => val,
                Err(_e) => return Some((Err("y4m: failed to parse integer"), other)),
            };
            (Param::Width(val), other)
        }
        b'H' => {
            let (res, other) = parse_u32(bytes)?;
            let val = match res {
                Ok(val) => val,
                Err(_e) => return Some((Err("y4m: failed to parse integer"), other)),
            };
            (Param::Height(val), other)
        }
        b'F' => {
            let (res, other) = parse_fraction(bytes)?;
            let val = match res {
                Ok(val) => val,
                Err(e) => return Some((Err(e), other)),
            };
            (Param::Framerate(val), other)
        }
        b'I' => {
            let res = parse_char(bytes)?;
            let val = match res {
                Ok(val) => val,
                Err(e) => return Some((Err("y4m: failed to parse char"), e)),
            };
            let other = bytes.next()?;
            (Param::Interlacing(val), other)
        }
        b'A' => {
            let (res, other) = parse_fraction(bytes)?;
            let val = match res {
                Ok(val) => val,
                Err(e) => return Some((Err(e), other)),
            };
            (Param::AspectRatio(val), other)
        }
        b'C' => {
            let (word, other) = parse_ascii(bytes)?;
            let res = ColorSpace::parse(&word).ok_or("y4m: failed to parse colorspace");
            let val = match res {
                Ok(val) => val,
                Err(e) => return Some((Err(e), other)),
            };
            (Param::ColorSpace(val), other)
        }
        _ => {
            let (word, other) = parse_ascii(bytes)?;
            (Param::Unknown(word), other)
        }
    };

    Some((Ok(param), other))
}
