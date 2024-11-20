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

#[derive(Clone, Debug)]
pub struct Transform2d {
    pub _origin: Vector2,
    pub _x: Vector2,
    pub _y: Vector2,
}

#[derive(Clone, Debug)]
pub struct Plane {
    pub _d: Float,
    pub _normal: Vector3,
    pub _x: Float,
    pub _y: Float,
    pub _z: Float,
}

#[derive(Clone, Debug)]
pub struct Quat {
    pub _w: Float,
    pub _x: Float,
    pub _y: Float,
    pub _z: Float,
}

#[derive(Clone, Debug)]
pub struct AABB {
    pub _end: Vector3,
    pub _position: Vector3,
    pub _size: Vector3,
}

#[derive(Clone, Debug)]
pub struct Basis {
    pub _x: Vector3,
    pub _y: Vector3,
    pub _z: Vector3,
}

#[derive(Clone, Debug)]
pub struct Transform {
    pub _basis: Basis,
    pub _origin: Vector3,
}

// These don't have impls and aren't used anyway.
#[derive(Clone, Debug)]
pub struct Color {}
#[derive(Clone, Debug)]
pub struct NodePath {}
#[derive(Clone, Debug)]
pub struct RID {}
#[derive(Clone, Debug)]
pub struct Object {}

pub type Dictionary = HashMap<String, VariantValue>;

pub type Array = Vec<VariantValue>;

/// https://docs.godotengine.org/en/3.5/classes/index.html#variant-types
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
    // START no impls
    _Transform2d(Transform2d),
    _Plane(Plane),
    _Quat(Quat),
    _AABB(AABB),
    _Basis(Basis),
    _Transform(Transform),
    _Color(Color),
    _NodePath(NodePath),
    _RID(RID),
    _Object(Object),
    // END no impls
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
            VariantValue::_Transform2d(_) => other == VariantType::_Transform2d,
            VariantValue::_Plane(_) => other == VariantType::_Plane,
            VariantValue::_Quat(_) => other == VariantType::_Quat,
            VariantValue::_AABB(_) => other == VariantType::_AABB,
            VariantValue::_Basis(_) => other == VariantType::_Basis,
            VariantValue::_Transform(_) => other == VariantType::_Transform,
            VariantValue::_Color(_) => other == VariantType::_Color,
            VariantValue::_NodePath(_) => other == VariantType::_NodePath,
            VariantValue::_RID(_) => other == VariantType::_RID,
            VariantValue::_Object(_) => other == VariantType::_Object,
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
            VariantValue::_Transform2d(_) => VariantType::_Transform2d,
            VariantValue::_Plane(_) => VariantType::_Plane,
            VariantValue::_Quat(_) => VariantType::_Quat,
            VariantValue::_AABB(_) => VariantType::_AABB,
            VariantValue::_Basis(_) => VariantType::_Basis,
            VariantValue::_Transform(_) => VariantType::_Transform,
            VariantValue::_Color(_) => VariantType::_Color,
            VariantValue::_NodePath(_) => VariantType::_NodePath,
            VariantValue::_RID(_) => VariantType::_RID,
            VariantValue::_Object(_) => VariantType::_Object,
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

/// https://docs.godotengine.org/en/3.5/classes/index.html#variant-types
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
    // START no impls
    _Transform2d = 8,
    _Plane = 9,
    _Quat = 10,
    _AABB = 11,
    _Basis = 12,
    _Transform = 13,
    _Color = 14,
    _NodePath = 15,
    _RID = 16,
    _Object = 17,
    // END no impls
    Dictionary = 18,
    Array = 19,
}
