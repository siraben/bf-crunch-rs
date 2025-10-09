use anyhow::{anyhow, bail, Result};

pub fn unescape_regex_like(input: &str) -> Result<String> {
    let mut output = String::with_capacity(input.len());
    let mut chars = input.chars();

    while let Some(ch) = chars.next() {
        if ch != '\\' {
            output.push(ch);
            continue;
        }

        match chars.next() {
            Some('n') => output.push('\n'),
            Some('r') => output.push('\r'),
            Some('t') => output.push('\t'),
            Some('f') => output.push('\u{000C}'),
            Some('v') => output.push('\u{000B}'),
            Some('a') => output.push('\u{0007}'),
            Some('b') => output.push('\u{0008}'),
            Some('0') => output.push('\u{0000}'),
            Some('x') => {
                let hi = chars
                    .next()
                    .ok_or_else(|| anyhow!("\\x escape missing digits"))?;
                let lo = chars
                    .next()
                    .ok_or_else(|| anyhow!("\\x escape missing digits"))?;
                let value = hex_pair_to_u8(hi, lo)?;
                output.push(char::from(value));
            }
            Some('u') => {
                let mut value: u32 = 0;
                for _ in 0..4 {
                    let digit = chars
                        .next()
                        .ok_or_else(|| anyhow!("\\u escape missing digits"))?;
                    value = (value << 4) | (digit_to_value(digit)? as u32);
                }
                let ch = char::from_u32(value).ok_or_else(|| anyhow!("Invalid \\u escape"))?;
                output.push(ch);
            }
            Some('U') => {
                let mut value: u32 = 0;
                for _ in 0..8 {
                    let digit = chars
                        .next()
                        .ok_or_else(|| anyhow!("\\U escape missing digits"))?;
                    value = (value << 4) | (digit_to_value(digit)? as u32);
                }
                let ch = char::from_u32(value).ok_or_else(|| anyhow!("Invalid \\U escape"))?;
                output.push(ch);
            }
            Some('c') => {
                if let Some(ctrl) = chars.next() {
                    let value = (ctrl as u32) & 0x1F;
                    output.push(
                        char::from_u32(value).ok_or_else(|| anyhow!("Invalid control escape"))?,
                    );
                } else {
                    bail!("\\c escape missing control character");
                }
            }
            Some(other) => output.push(other),
            None => output.push('\\'),
        }
    }

    Ok(output)
}

pub fn to_iso_8859_1_bytes(input: &str) -> Result<Vec<u8>> {
    let mut bytes = Vec::with_capacity(input.len());
    for ch in input.chars() {
        if (ch as u32) > 0xFF {
            bail!("Character '{}' is not representable in ISO-8859-1", ch);
        }
        bytes.push(ch as u8);
    }
    Ok(bytes)
}

pub fn abs_diff(a: u8, b: u8) -> i32 {
    let a = a as i32;
    let b = b as i32;
    (a - b).abs()
}

pub fn add_byte(value: u8, delta: i32) -> u8 {
    value.wrapping_add(delta as u8)
}

pub fn negate_byte(value: u8) -> u8 {
    (-(value as i32)) as u8
}

fn hex_pair_to_u8(hi: char, lo: char) -> Result<u8> {
    let high = digit_to_value(hi)?;
    let low = digit_to_value(lo)?;
    Ok(((high << 4) | low) as u8)
}

fn digit_to_value(ch: char) -> Result<i32> {
    match ch {
        '0'..='9' => Ok((ch as i32) - ('0' as i32)),
        'a'..='f' => Ok((ch as i32) - ('a' as i32) + 10),
        'A'..='F' => Ok((ch as i32) - ('A' as i32) + 10),
        _ => bail!("Invalid hex digit '{}'", ch),
    }
}
