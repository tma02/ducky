use std::io;
use std::io::Cursor;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Write;

use super::variant::Dictionary;
use super::variant::Rect2;
use super::variant::VariantType;
use super::variant::VariantValue;
use super::variant::Vector2;
use super::variant::Vector3;

// TODO: consider implementing serde::Deserializer

pub fn encode_variant(value: VariantValue) -> io::Result<Vec<u8>> {
    let array = vec![];
    let mut cursor = Cursor::new(array);
    encode_variant_into_cursor(&mut cursor, value)?;

    Ok(cursor.into_inner())
}

fn write_bool(cursor: &mut Cursor<Vec<u8>>, value: bool) -> io::Result<usize> {
    cursor.write(&[value as u8, 0, 0, 0])
}

fn write_bool_variant_header(cursor: &mut Cursor<Vec<u8>>) -> io::Result<usize> {
    cursor.write(&[VariantType::Bool as u8, 0, 0, 0])
}

fn write_i32(cursor: &mut Cursor<Vec<u8>>, value: i32) -> io::Result<()> {
    let mut buffer = [0; 4];
    buffer[0..4].copy_from_slice(&value.to_le_bytes());
    cursor.write_all(&buffer)
}

fn _write_i32_variant_header(cursor: &mut Cursor<Vec<u8>>) -> io::Result<usize> {
    cursor.write(&[VariantType::Int as u8, 0, 0, 0])
}

fn write_i64(cursor: &mut Cursor<Vec<u8>>, value: i64) -> io::Result<()> {
    let mut buffer = [0; 8];
    buffer[0..8].copy_from_slice(&value.to_le_bytes());
    cursor.write_all(&buffer)
}

fn write_i64_variant_header(cursor: &mut Cursor<Vec<u8>>) -> io::Result<usize> {
    cursor.write(&[VariantType::Int as u8, 0, 1, 0])
}

fn write_f64(cursor: &mut Cursor<Vec<u8>>, value: f64) -> io::Result<()> {
    let mut buffer = [0; 8];
    buffer[0..8].copy_from_slice(&value.to_le_bytes());
    cursor.write_all(&buffer)
}

fn write_f64_variant_header(cursor: &mut Cursor<Vec<u8>>) -> io::Result<usize> {
    cursor.write(&[VariantType::Float as u8, 0, 1, 0])
}

fn write_f32(cursor: &mut Cursor<Vec<u8>>, value: f32) -> io::Result<()> {
    let mut buffer = [0; 4];
    buffer[0..4].copy_from_slice(&value.to_le_bytes());
    cursor.write_all(&buffer)
}

fn _write_f32_variant_header(cursor: &mut Cursor<Vec<u8>>) -> io::Result<usize> {
    cursor.write(&[VariantType::Float as u8, 0, 0, 0])
}

fn write_string(cursor: &mut Cursor<Vec<u8>>, value: String) -> io::Result<()> {
    let bytes = value.as_bytes();
    write_i32(cursor, bytes.len() as i32)?;
    cursor.write_all(bytes)?;
    // Pad to 4 bytes
    for _ in 0..(4 - (bytes.len() % 4)) % 4 {
        cursor.write_all(&[0])?
    }

    Ok(())
}

fn write_string_variant_header(cursor: &mut Cursor<Vec<u8>>) -> io::Result<usize> {
    cursor.write(&[VariantType::String as u8, 0, 0, 0])
}

fn write_vector2(cursor: &mut Cursor<Vec<u8>>, value: Vector2) -> io::Result<()> {
    write_f32(cursor, value.x as f32)?;
    write_f32(cursor, value.y as f32)?;

    Ok(())
}

fn write_vector2_variant_header(cursor: &mut Cursor<Vec<u8>>) -> io::Result<usize> {
    cursor
        .write(&[VariantType::Vector2 as u8, 0, 0, 0])
}

fn write_rect2(cursor: &mut Cursor<Vec<u8>>, value: Rect2) -> io::Result<()> {
    write_vector2(cursor, value.end)?;
    write_vector2(cursor, value.position)?;
    write_vector2(cursor, value.size)?;

    Ok(())
}

fn write_rect2_variant_header(cursor: &mut Cursor<Vec<u8>>) -> io::Result<usize> {
    cursor.write(&[VariantType::Rect2 as u8, 0, 0, 0])
}

fn write_vector3(cursor: &mut Cursor<Vec<u8>>, value: Vector3) -> io::Result<()> {
    write_f32(cursor, value.x as f32)?;
    write_f32(cursor, value.y as f32)?;
    write_f32(cursor, value.z as f32)?;

    Ok(())
}

fn write_vector3_variant_header(cursor: &mut Cursor<Vec<u8>>) -> io::Result<usize> {
    cursor
        .write(&[VariantType::Vector3 as u8, 0, 0, 0])
}

fn write_array(cursor: &mut Cursor<Vec<u8>>, value: Vec<VariantValue>) -> io::Result<()> {
    write_i32(cursor, value.len() as i32)?;
    for item in value {
        encode_variant_into_cursor(cursor, item)?;
    }

    Ok(())
}

fn write_array_variant_header(cursor: &mut Cursor<Vec<u8>>) -> io::Result<usize> {
    cursor.write(&[VariantType::Array as u8, 0, 0, 0])
}

fn write_dictionary(cursor: &mut Cursor<Vec<u8>>, value: Dictionary) -> io::Result<()> {
    write_i32(cursor, value.len() as i32)?;
    for (key, value) in value {
        write_string_variant_header(cursor)?;
        write_string(cursor, key)?;
        encode_variant_into_cursor(cursor, value)?;
    }

    Ok(())
}

fn write_dictionary_variant_header(cursor: &mut Cursor<Vec<u8>>) -> io::Result<usize> {
    cursor
        .write(&[VariantType::Dictionary as u8, 0, 0, 0])
}

fn encode_variant_into_cursor(cursor: &mut Cursor<Vec<u8>>, value: VariantValue) -> io::Result<()> {
    match value {
        VariantValue::Bool(value) => {
            write_bool_variant_header(cursor)?;
            write_bool(cursor, value)?;

            Ok(())
        }
        VariantValue::Int(value) => {
            write_i64_variant_header(cursor)?;
            write_i64(cursor, value)?;

            Ok(())
        }
        VariantValue::Float(value) => {
            write_f64_variant_header(cursor)?;
            write_f64(cursor, value)?;

            Ok(())
        }
        VariantValue::String(value) => {
            write_string_variant_header(cursor)?;
            write_string(cursor, value)?;

            Ok(())
        }
        VariantValue::Vector2(value) => {
            write_vector2_variant_header(cursor)?;
            write_vector2(cursor, value)?;

            Ok(())
        }
        VariantValue::Rect2(value) => {
            write_rect2_variant_header(cursor)?;
            write_rect2(cursor, value)?;

            Ok(())
        }
        VariantValue::Vector3(value) => {
            write_vector3_variant_header(cursor)?;
            write_vector3(cursor, value)?;

            Ok(())
        }
        // VariantValue::Transform2d(transform2d) => todo!(),
        // VariantValue::Plane(plane) => todo!(),
        // VariantValue::Quat(quat) => todo!(),
        // VariantValue::AABB(aabb) => todo!(),
        // VariantValue::Basis(basis) => todo!(),
        // VariantValue::Transform(transform) => todo!(),
        // VariantValue::Color(color) => todo!(),
        // VariantValue::NodePath(node_path) => todo!(),
        // VariantValue::RID(rid) => todo!(),
        // VariantValue::Object(object) => todo!(),
        VariantValue::Array(value) => {
            write_array_variant_header(cursor)?;
            write_array(cursor, value)?;

            Ok(())
        }
        VariantValue::Dictionary(value) => {
            write_dictionary_variant_header(cursor)?;
            write_dictionary(cursor, value)?;

            Ok(())
        }
        _ => Err(Error::new(ErrorKind::InvalidData, "Unsupported VariantValue type")),
    }
}
