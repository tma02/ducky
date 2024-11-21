use std::{
    io::{self, Cursor, Read},
    string::FromUtf8Error,
};

use super::variant::*;

static TAG: &str = "decode";

// TODO: consider implementing serde::Serializer

/// Decodes a u8 slice into a Variant.
pub fn decode_variant(buffer: &[u8]) -> io::Result<VariantValue> {
    let mut cursor = Cursor::new(buffer);
    read_variant(&mut cursor)
}

fn read_i32(cursor: &mut Cursor<&[u8]>) -> io::Result<i32> {
    let mut buffer = [0; 4];
    cursor.read_exact(&mut buffer)?;
    Ok(i32::from_le_bytes(buffer))
}

fn read_u16(cursor: &mut Cursor<&[u8]>) -> io::Result<u16> {
    let mut buffer = [0; 2];
    cursor.read_exact(&mut buffer)?;
    Ok(u16::from_le_bytes(buffer))
}

fn read_variant(cursor: &mut Cursor<&[u8]>) -> io::Result<VariantValue> {
    let var_type = read_u16(cursor);
    let type_flags = read_u16(cursor)?;
    match var_type {
        Ok(0) => Ok(VariantValue::Nil),
        Ok(1) => Ok(VariantValue::Bool(read_bool(cursor)?)),
        Ok(2) => Ok(VariantValue::Int(if type_flags & 1 == 1 {
            read_i64(cursor)?
        } else {
            read_i32(cursor)? as i64
        })),
        Ok(3) => Ok(VariantValue::Float(if type_flags & 1 == 1 {
            read_f64(cursor)?
        } else {
            read_f32(cursor)? as f64
        })),
        Ok(4) => Ok(VariantValue::String(read_string(cursor)?)),
        Ok(5) => Ok(VariantValue::Vector2(read_vector2(cursor)?)),
        Ok(6) => Ok(VariantValue::Rect2(read_rect2(cursor)?)),
        Ok(7) => Ok(VariantValue::Vector3(read_vector3(cursor)?)),
        // TODO: Some values do not have impls right now, but also aren't used.
        Ok(18) => Ok(VariantValue::Dictionary(read_dictionary(cursor)?)),
        Ok(19) => Ok(VariantValue::Array(read_array(cursor)?)),
        invalid => {
            println!("Invalid variant type: {:?}", invalid);
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid variant type",
            ))
        }
    }
}

fn read_bool(cursor: &mut Cursor<&[u8]>) -> io::Result<bool> {
    let mut buf = [0; 4];
    cursor.read_exact(&mut buf)?;
    Ok(buf[0] != 0)
}

fn read_i64(cursor: &mut Cursor<&[u8]>) -> io::Result<i64> {
    let mut buf = [0 as u8; 8];
    cursor.read_exact(&mut buf)?;
    Ok(i64::from_le_bytes(buf))
}

fn read_f64(cursor: &mut Cursor<&[u8]>) -> io::Result<f64> {
    let mut buf = [0 as u8; 8];
    cursor.read_exact(&mut buf)?;
    Ok(f64::from_le_bytes(buf))
}

fn read_f32(cursor: &mut Cursor<&[u8]>) -> io::Result<f32> {
    let mut buf = [0 as u8; 4];
    cursor.read_exact(&mut buf)?;
    Ok(f32::from_le_bytes(buf))
}

fn read_string(cursor: &mut Cursor<&[u8]>) -> io::Result<String> {
    let str_length = read_i32(cursor)? as usize;
    // This field is padded to 4 bytes
    let buf_length = 4 * ((str_length + 3) / 4);
    let mut buf = vec![0; buf_length];
    cursor.read_exact(&mut buf)?;
    // Should be an in-place truncation of the pad bytes
    buf.resize(str_length, 0);
    Ok(String::from_utf8(buf)
        .map_err(|e: FromUtf8Error| io::Error::new(io::ErrorKind::InvalidData, e))?)
}

fn read_vector2(cursor: &mut Cursor<&[u8]>) -> io::Result<Vector2> {
    Ok(Vector2 {
        x: read_f32(cursor)? as f64,
        y: read_f32(cursor)? as f64,
    })
}

fn read_rect2(cursor: &mut Cursor<&[u8]>) -> io::Result<Rect2> {
    Ok(Rect2 {
        end: read_vector2(cursor)?,
        position: read_vector2(cursor)?,
        size: read_vector2(cursor)?,
    })
}

fn read_vector3(cursor: &mut Cursor<&[u8]>) -> io::Result<Vector3> {
    Ok(Vector3 {
        x: read_f32(cursor)? as f64,
        y: read_f32(cursor)? as f64,
        z: read_f32(cursor)? as f64,
    })
}

fn read_dictionary(cursor: &mut Cursor<&[u8]>) -> io::Result<Dictionary> {
    let count = read_i32(cursor)?;

    let mut dict = Dictionary::new();
    for _ in 0..count {
        let key_variant = read_variant(cursor);
        if let (Ok(VariantValue::String(key_string)), Ok(value_variant)) =
            (key_variant, read_variant(cursor))
        {
            dict.insert(key_string, value_variant);
        } else {
            println!("[{}] Got non-String Dictionary key", TAG);
        }
    }

    Ok(dict)
}

fn read_array(cursor: &mut Cursor<&[u8]>) -> io::Result<Array> {
    let count = read_i32(cursor)?;

    let mut array: Vec<VariantValue> = Array::new();
    for _ in 0..count {
        if let Ok(value_variant) = read_variant(cursor) {
            array.push(value_variant);
        }
    }

    Ok(array)
}
