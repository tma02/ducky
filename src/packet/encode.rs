use std::io::Cursor;
use std::io::Write;

use super::variant::Dictionary;
use super::variant::Rect2;
use super::variant::VariantType;
use super::variant::VariantValue;
use super::variant::Vector2;
use super::variant::Vector3;

// TODO: consider implementing serde::Deserializer

pub fn encode_variant(value: VariantValue) -> Vec<u8> {
    let array = vec![];
    let mut cursor = Cursor::new(array);
    encode_variant_into_cursor(&mut cursor, value);

    cursor.into_inner()
}

fn write_bool(cursor: &mut Cursor<Vec<u8>>, value: bool) {
    cursor.write(&[value as u8, 0, 0, 0]).unwrap();
}

fn write_bool_variant_header(cursor: &mut Cursor<Vec<u8>>) {
    cursor.write(&[VariantType::Bool as u8, 0, 0, 0]).unwrap();
}

fn write_i32(cursor: &mut Cursor<Vec<u8>>, value: i32) {
    let mut buffer = [0; 4];
    buffer[0..4].copy_from_slice(&value.to_le_bytes());
    cursor.write_all(&buffer).unwrap();
}

fn _write_i32_variant_header(cursor: &mut Cursor<Vec<u8>>) {
    cursor.write(&[VariantType::Int as u8, 0, 0, 0]).unwrap();
}

fn write_i64(cursor: &mut Cursor<Vec<u8>>, value: i64) {
    let mut buffer = [0; 8];
    buffer[0..8].copy_from_slice(&value.to_le_bytes());
    cursor.write_all(&buffer).unwrap();
}

fn write_i64_variant_header(cursor: &mut Cursor<Vec<u8>>) {
    cursor.write(&[VariantType::Int as u8, 0, 1, 0]).unwrap();
}

fn write_f64(cursor: &mut Cursor<Vec<u8>>, value: f64) {
    let mut buffer = [0; 8];
    buffer[0..8].copy_from_slice(&value.to_le_bytes());
    cursor.write_all(&buffer).unwrap();
}

fn write_f64_variant_header(cursor: &mut Cursor<Vec<u8>>) {
    cursor.write(&[VariantType::Float as u8, 0, 1, 0]).unwrap();
}

fn write_f32(cursor: &mut Cursor<Vec<u8>>, value: f32) {
    let mut buffer = [0; 4];
    buffer[0..4].copy_from_slice(&value.to_le_bytes());
    cursor.write_all(&buffer).unwrap();
}

fn _write_f32_variant_header(cursor: &mut Cursor<Vec<u8>>) {
    cursor.write(&[VariantType::Float as u8, 0, 0, 0]).unwrap();
}

fn write_string(cursor: &mut Cursor<Vec<u8>>, value: String) {
    let bytes = value.as_bytes();
    write_i32(cursor, bytes.len() as i32);
    cursor.write_all(bytes).unwrap();
    // Pad to 4 bytes
    (0..(4 - (bytes.len() % 4)) % 4).for_each(|_| cursor.write_all(&[0]).unwrap());
}

fn write_string_variant_header(cursor: &mut Cursor<Vec<u8>>) {
    cursor.write(&[VariantType::String as u8, 0, 0, 0]).unwrap();
}

fn write_vector2(cursor: &mut Cursor<Vec<u8>>, value: Vector2) {
    write_f32(cursor, value.x as f32);
    write_f32(cursor, value.y as f32);
}

fn write_vector2_variant_header(cursor: &mut Cursor<Vec<u8>>) {
    cursor
        .write(&[VariantType::Vector2 as u8, 0, 0, 0])
        .unwrap();
}

fn write_rect2(cursor: &mut Cursor<Vec<u8>>, value: Rect2) {
    write_vector2(cursor, value.end);
    write_vector2(cursor, value.position);
    write_vector2(cursor, value.size);
}

fn write_rect2_variant_header(cursor: &mut Cursor<Vec<u8>>) {
    cursor.write(&[VariantType::Rect2 as u8, 0, 0, 0]).unwrap();
}

fn write_vector3(cursor: &mut Cursor<Vec<u8>>, value: Vector3) {
    write_f32(cursor, value.x as f32);
    write_f32(cursor, value.y as f32);
    write_f32(cursor, value.z as f32);
}

fn write_vector3_variant_header(cursor: &mut Cursor<Vec<u8>>) {
    cursor
        .write(&[VariantType::Vector3 as u8, 0, 0, 0])
        .unwrap();
}

fn write_array(cursor: &mut Cursor<Vec<u8>>, value: Vec<VariantValue>) {
    write_i32(cursor, value.len() as i32);
    for item in value {
        encode_variant_into_cursor(cursor, item);
    }
}

fn write_array_variant_header(cursor: &mut Cursor<Vec<u8>>) {
    cursor.write(&[VariantType::Array as u8, 0, 0, 0]).unwrap();
}

fn write_dictionary(cursor: &mut Cursor<Vec<u8>>, value: Dictionary) {
    write_i32(cursor, value.len() as i32);
    for (key, value) in value {
        write_string_variant_header(cursor);
        write_string(cursor, key);
        encode_variant_into_cursor(cursor, value);
    }
}

fn write_dictionary_variant_header(cursor: &mut Cursor<Vec<u8>>) {
    cursor
        .write(&[VariantType::Dictionary as u8, 0, 0, 0])
        .unwrap();
}

fn encode_variant_into_cursor(cursor: &mut Cursor<Vec<u8>>, value: VariantValue) {
    match value {
        VariantValue::Bool(value) => {
            write_bool_variant_header(cursor);
            write_bool(cursor, value);
        }
        VariantValue::Int(value) => {
            write_i64_variant_header(cursor);
            write_i64(cursor, value);
        }
        VariantValue::Float(value) => {
            write_f64_variant_header(cursor);
            write_f64(cursor, value);
        }
        VariantValue::String(value) => {
            write_string_variant_header(cursor);
            write_string(cursor, value);
        }
        VariantValue::Vector2(value) => {
            write_vector2_variant_header(cursor);
            write_vector2(cursor, value);
        }
        VariantValue::Rect2(value) => {
            write_rect2_variant_header(cursor);
            write_rect2(cursor, value);
        }
        VariantValue::Vector3(value) => {
            write_vector3_variant_header(cursor);
            write_vector3(cursor, value);
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
            write_array_variant_header(cursor);
            write_array(cursor, value);
        }
        VariantValue::Dictionary(value) => {
            write_dictionary_variant_header(cursor);
            write_dictionary(cursor, value);
        }
        _ => panic!("Unsupported variant type"),
    }
}
