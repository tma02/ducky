use std::collections::HashMap;

use serde::Deserialize;

type Float = f64;
type Int = i64;

#[derive(Clone, Debug)]
pub struct Vector2 {
    pub x: Float,
    pub y: Float,
}

#[derive(Clone, Debug)]
pub struct Rect2 {
    pub end: Vector2,
    pub position: Vector2,
    pub size: Vector2,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Vector3 {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}

pub type Dictionary = HashMap<String, VariantValue>;

pub type Array = Vec<VariantValue>;

/// https://docs.godotengine.org/en/stable/tutorials/io/binary_serialization_api.html
#[derive(Clone, Debug)]
pub enum VariantValue {
    Nil,
    Bool(bool),
    Int(Int),
    Float(Float),
    String(String),
    Vector2(Vector2),
    Rect2(Rect2),
    Vector3(Vector3),
    Dictionary(Dictionary),
    Array(Array),
}

impl VariantValue {
    pub fn is_type_of(&self, other: VariantType) -> bool {
        match self {
            VariantValue::Nil => other == VariantType::Nil,
            VariantValue::Bool(_) => other == VariantType::Bool,
            VariantValue::Int(_) => other == VariantType::Int,
            VariantValue::Float(_) => other == VariantType::Float,
            VariantValue::String(_) => other == VariantType::String,
            VariantValue::Vector2(_) => other == VariantType::Vector2,
            VariantValue::Rect2(_) => other == VariantType::Rect2,
            VariantValue::Vector3(_) => other == VariantType::Vector3,
            VariantValue::Dictionary(_) => other == VariantType::Dictionary,
            VariantValue::Array(_) => other == VariantType::Array,
        }
    }

    pub fn get_type(&self) -> VariantType {
        match &self {
            VariantValue::Nil => VariantType::Nil,
            VariantValue::Bool(_) => VariantType::Bool,
            VariantValue::Int(_) => VariantType::Int,
            VariantValue::Float(_) => VariantType::Float,
            VariantValue::String(_) => VariantType::String,
            VariantValue::Vector2(_) => VariantType::Vector2,
            VariantValue::Rect2(_) => VariantType::Rect2,
            VariantValue::Vector3(_) => VariantType::Vector3,
            VariantValue::Dictionary(_) => VariantType::Dictionary,
            VariantValue::Array(_) => VariantType::Array,
        }
    }
}

impl TryInto<i64> for VariantValue {
    type Error = ();

    fn try_into(self) -> Result<i64, Self::Error> {
        if let VariantValue::Int(i) = self {
            Ok(i)
        } else {
            Err(())
        }
    }
}

impl TryInto<f64> for VariantValue {
    type Error = ();

    fn try_into(self) -> Result<f64, Self::Error> {
        if let VariantValue::Float(f) = self {
            Ok(f)
        } else {
            Err(())
        }
    }
}

impl TryInto<bool> for VariantValue {
    type Error = ();

    fn try_into(self) -> Result<bool, Self::Error> {
        if let VariantValue::Bool(b) = self {
            Ok(b)
        } else {
            Err(())
        }
    }
}

impl TryInto<String> for VariantValue {
    type Error = ();

    fn try_into(self) -> Result<String, Self::Error> {
        if let VariantValue::String(s) = self {
            Ok(s)
        } else {
            Err(())
        }
    }
}

impl<'a> TryInto<&'a String> for &'a VariantValue {
    type Error = ();

    fn try_into(self) -> Result<&'a String, Self::Error> {
        if let VariantValue::String(s) = self {
            Ok(s)
        } else {
            Err(())
        }
    }
}

impl TryInto<Vector2> for VariantValue {
    type Error = ();

    fn try_into(self) -> Result<Vector2, Self::Error> {
        if let VariantValue::Vector2(v) = self {
            Ok(v)
        } else {
            Err(())
        }
    }
}

impl TryInto<Rect2> for VariantValue {
    type Error = ();

    fn try_into(self) -> Result<Rect2, Self::Error> {
        if let VariantValue::Rect2(r) = self {
            Ok(r)
        } else {
            Err(())
        }
    }
}

impl TryInto<Vector3> for VariantValue {
    type Error = ();

    fn try_into(self) -> Result<Vector3, Self::Error> {
        if let VariantValue::Vector3(v) = self {
            Ok(v)
        } else {
            Err(())
        }
    }
}

impl<'a> TryInto<&'a Vector3> for &'a VariantValue {
    type Error = ();

    fn try_into(self) -> Result<&'a Vector3, Self::Error> {
        if let VariantValue::Vector3(v) = self {
            Ok(v)
        } else {
            Err(())
        }
    }
}

impl TryInto<Dictionary> for VariantValue {
    type Error = ();

    fn try_into(self) -> Result<Dictionary, Self::Error> {
        if let VariantValue::Dictionary(d) = self {
            Ok(d)
        } else {
            Err(())
        }
    }
}

impl TryInto<Array> for VariantValue {
    type Error = ();

    fn try_into(self) -> Result<Array, Self::Error> {
        if let VariantValue::Array(a) = self {
            Ok(a)
        } else {
            Err(())
        }
    }
}

/// https://docs.godotengine.org/en/stable/tutorials/io/binary_serialization_api.html
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VariantType {
    Nil = 0,
    Bool = 1,
    Int = 2,
    Float = 3,
    String = 4,
    Vector2 = 5,
    Rect2 = 6,
    Vector3 = 7,
    Dictionary = 18,
    Array = 19,
}
