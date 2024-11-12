/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use crate::{
    parsers::{parse_color, parse_property},
    Color, TiledValue, Vec2,
};
use serde::de::Error;
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;

#[derive(Debug, PartialEq, Clone)]
pub struct Object {
    // GID, only if object comes from a Tilemap
    pub gid: Option<u32>,
    /// Incremental id - unique across all objects
    pub id: Option<u32>,
    pub name: String,
    pub custom_type: String,
    /// Angle in degrees clockwise
    pub rotation: f32,
    pub height: f32,
    pub width: f32,
    /// X coordinate in pixels
    pub x: f32,
    /// Y coordinate in pixels
    pub y: f32,
    pub properties: HashMap<String, TiledValue>,
    /// An *almost* concrete type. Some types aren't included in this, eg; a square
    /// which can be derived from the X/Y & Height/Width
    ///
    /// Types can be:
    /// - Point
    /// - Polygon
    /// - Polyline
    /// - Text
    /// - Ellipse
    /// - or None
    pub object_type: ObjectType,
}

impl<'de> Deserialize<'de> for Object {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize, Debug)]
        #[serde(rename_all = "lowercase")]
        struct ExternalBase {
            id: u32,
            template: String,
            #[serde(deserialize_with = "parse_property", default)]
            properties: HashMap<String, TiledValue>,
            #[serde(default)]
            x: f32,
            #[serde(default)]
            y: f32,
        }

        #[derive(Deserialize, Debug)]
        struct Intermediate {
            object: ExternalDetails,
        }

        #[derive(Deserialize, Debug)]
        #[serde(rename_all = "lowercase")]
        struct ExternalDetails {
            gid: Option<u32>,
            id: Option<u32>,
            name: String,
            #[serde(rename(deserialize = "type"), default)]
            custom_type: String,
            rotation: f32,
            height: f32,
            width: f32,
            #[serde(flatten)]
            object_type: ObjectType,
        }

        #[derive(Deserialize)]
        struct Internal {
            gid: Option<u32>,
            id: Option<u32>,
            name: String,
            #[serde(rename(deserialize = "type"), default)]
            custom_type: String,
            rotation: f32,
            height: f32,
            width: f32,
            #[serde(default)]
            x: f32,
            #[serde(default)]
            y: f32,
            #[serde(deserialize_with = "parse_property", default)]
            properties: HashMap<String, TiledValue>,
            #[serde(flatten)]
            object_type: ObjectType,
        }

        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Helper {
            Internal(Internal),
            External(ExternalBase),
        }

        let v = serde_json::Value::deserialize(deserializer)?;
        match Helper::deserialize(&v) {
            Ok(m) => {
                match m {
                    Helper::Internal(t) => {
                        return Ok(Object {
                            gid: t.gid,
                            id: t.id,
                            name: t.name,
                            custom_type: t.custom_type,
                            rotation: t.rotation,
                            height: t.height,
                            width: t.width,
                            x: t.x,
                            y: t.y,
                            properties: t.properties,
                            object_type: t.object_type,
                        });
                    }
                    Helper::External(object_base) => {
                        let path = PathBuf::from(&object_base.template);
                        let file = File::open(path)
                            .map_err(|e| Error::custom(format!("{:?}", e)))?;
                        let intermediate: Intermediate =
                            serde_json::from_reader(file).map_err(|e| {
                                Error::custom(format!("{:?}", e))
                            })?;

                        return Ok(Object {
                            gid: intermediate.object.gid,
                            id: Some(object_base.id),
                            name: intermediate.object.name,
                            custom_type: intermediate.object.custom_type,
                            rotation: intermediate.object.rotation,
                            height: intermediate.object.height,
                            width: intermediate.object.width,
                            x: object_base.x,
                            y: object_base.y,
                            properties: object_base.properties,
                            object_type: intermediate.object.object_type,
                        });
                    }
                };
            }
            Err(e) => return Err(Error::custom(format!("{:?}", e))),
        }
    }
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Text {
    /// 0000: off, 0001: bold, 0010:italic, 0100: wrap (1, 2, 4, 8)
    pub flags: u8,
    #[serde(deserialize_with = "parse_color", default)]
    pub color: Color,
    pub text: String,
}

/// Contains data for the object sub-types
#[derive(Debug, PartialEq, Clone)]
pub enum ObjectType {
    Ellipse,
    Point,
    Polygon(Vec<Vec2<i32>>),
    PolyLine(Vec<Vec2<i32>>),
    Text(Text),
    Template(String),
    None,
}

impl ObjectType {
    pub fn is_none(&self) -> bool {
        *self == ObjectType::None
    }

    pub fn is_some(&self) -> bool {
        *self != ObjectType::None
    }
}

impl<'de> Deserialize<'de> for ObjectType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "lowercase")]
        enum Helper {
            Ellipse(bool),
            Point(bool),
            Polygon(Vec<Vec2<i32>>),
            Polyline(Vec<Vec2<i32>>),
            Text {
                bold: bool,
                #[serde(deserialize_with = "parse_color", default)]
                color: Color,
                italic: bool,
                text: String,
                wrap: bool,
            },
        }

        let v = serde_json::Value::deserialize(deserializer)?;
        if let Ok(m) = Helper::deserialize(&v) {
            return match m {
                Helper::Ellipse(b) => {
                    if b {
                        Ok(ObjectType::Ellipse)
                    } else {
                        Ok(ObjectType::None)
                    }
                }
                Helper::Point(b) => {
                    if b {
                        Ok(ObjectType::Point)
                    } else {
                        Ok(ObjectType::None)
                    }
                }
                Helper::Polygon(data) => Ok(ObjectType::Polygon(data)),
                Helper::Polyline(data) => Ok(ObjectType::PolyLine(data)),
                Helper::Text {
                    bold,
                    color,
                    italic,
                    text,
                    wrap,
                } => {
                    let mut flags = 0u8;
                    if bold {
                        flags |= 1
                    }
                    if italic {
                        flags |= 2
                    }
                    if wrap {
                        flags |= 4
                    }
                    Ok(ObjectType::Text(Text { flags, color, text }))
                }
            };
        }
        Ok(ObjectType::None)
    }
}
