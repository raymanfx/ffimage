use std::str::FromStr;

pub fn parse_char(bytes: &mut impl Iterator<Item = u8>) -> Option<Result<char, u8>> {
    let byte = bytes.next()?;
    match byte {
        b'A'..=b'Z' | b'a'..=b'z' => Some(Ok(byte as char)),
        _ => Some(Err(byte)),
    }
}

pub fn parse_digit(bytes: &mut impl Iterator<Item = u8>) -> Option<Result<char, u8>> {
    let byte = bytes.next()?;
    match byte {
        b'0'..=b'9' => Some(Ok(byte as char)),
        _ => Some(Err(byte)),
    }
}

pub fn parse_seq<I>(
    bytes: &mut I,
    predicate: impl Fn(&mut I) -> Option<Result<char, u8>>,
) -> Option<(String, u8)>
where
    I: Iterator<Item = u8>,
{
    let mut seq = String::new();

    loop {
        let res = predicate(bytes)?;
        match res {
            Ok(val) => seq.push(val),
            Err(b) => return Some((seq, b)),
        }
    }
}

pub fn parse_ascii(bytes: &mut impl Iterator<Item = u8>) -> Option<(String, u8)> {
    parse_seq(bytes, |iter| {
        let b = iter.next()?;
        match b {
            b' ' => Some(Err(b)),
            b'\n' => Some(Err(b)),
            _ => Some(Ok(b as char)),
        }
    })
}

pub fn parse_u32(
    bytes: &mut impl Iterator<Item = u8>,
) -> Option<(Result<u32, <u32 as FromStr>::Err>, u8)> {
    let (word, other) = parse_seq(bytes, parse_digit)?;
    match word.parse::<u32>() {
        Ok(number) => Some((Ok(number), other)),
        Err(e) => Some((Err(e), other)),
    }
}
