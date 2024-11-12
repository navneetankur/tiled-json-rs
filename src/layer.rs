/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use crate::{
    object::Object,
    parsers::{parse_color, parse_data, parse_path, parse_property},
    Color, TiledValue,
};
use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct TileLayer {
    /// Array of chunks (optional, generally infinite maps)
    pub chunks: Option<Vec<Chunk>>,
    /// Data consists of the global ID's of tiles making up this
    /// layer of the map
    #[serde(deserialize_with = "parse_data", default)]
    pub data: Vec<u32>,
    /// Row count. Same as map height for fixed-size maps.
    pub height: u32,
    /// Column count. Same as map width for fixed-size maps.
    pub width: u32,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct ObjectGroup {
    /// `TopDown` (default) or `Index`
    #[serde(rename(deserialize = "draworder"), default)]
    pub draw_order: DrawOrder,
    /// The array of `Object` in this layer
    #[serde(default)]
    pub objects: Vec<Object>,
}

/// Contains a file path to an image plus a mix colour
#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct ImageLayer {
    /// Image used by this layer
    #[serde(deserialize_with = "parse_path", default)]
    pub image: PathBuf,
    #[serde(
        rename(deserialize = "transparentcolor"),
        deserialize_with = "parse_color",
        default
    )]
    /// Defaults to 0,0,0,0 (rgba)
    pub transparent_color: Color,
}

/// Used to group layers if required
#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Group {
    pub layers: Vec<Layer>,
}

/// Contains the data for this variant of layer
///
/// # Example
///
/// ``` no-run
/// match &layer_type {
///     LayerType::TileLayer(tiles) => {
///         //do_something(tiles);
///     }
///     LayerType::Group { layers } => {
///         &mut render_layers(layers);
///     }
///     LayerType::ImageLayer(image) => {
///         //do_something_else(image);
///     }
///     LayerType::ObjectGroup(objects) => {
///         //and_another_thing(objects);
///     }
/// }
/// ```
#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all(deserialize = "lowercase"), tag = "type")]
pub enum LayerType {
    TileLayer(TileLayer),
    ObjectGroup(ObjectGroup),
    ImageLayer(ImageLayer),
    Group { layers: Vec<Layer> },
}

/// A map can contain any number of layers.
///
/// Layers have sub-types such as (enum) `LayerType::TileLayer(TileLayer)`
/// which contains the data for that sub-type.
#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Layer {
    pub name: String,
    /// Horizontal layer offset in pixels (default: 0)
    #[serde(rename(deserialize = "offsetx"), default)]
    pub offset_x: f32,
    /// Vertical layer offset in pixels (default: 0)
    #[serde(rename(deserialize = "offsety"), default)]
    pub offset_y: f32,
    /// Value between 0 and 1
    pub opacity: f32,
    #[serde(deserialize_with = "parse_property", default)]
    pub properties: HashMap<String, TiledValue>,
    #[serde(flatten)]
    /// The `LayerType` object also contains the data relating to the type
    pub layer_type: LayerType,
    /// Horizontal layer offset in tiles. Always 0.
    pub x: i32,
    /// Vertical layer offset in tiles. Always 0.
    pub y: i32,

    pub visible: bool,
}

/// Chunks are used to store the tile layer data for infinite maps
#[derive(Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Chunk {
    /// Array of unsigned int (GIDs) or base64-encoded data
    #[serde(deserialize_with = "parse_data", default)]
    pub data: Vec<u32>,
    pub height: u32,
    pub width: u32,
    pub x: i32,
    pub y: i32,
}

/// Can be `TopDown` (default) or `Index`. Applies to `ObjectGroup` only.
#[derive(Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all(deserialize = "lowercase"))]
pub enum DrawOrder {
    TopDown,
    Index,
}

impl Default for DrawOrder {
    fn default() -> Self {
        DrawOrder::TopDown
    }
}
