/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! The final parsed structure is in some ways not suitable
//! for use so this crate is designed to produce a structure
//! to be *consumed* by your own implementations.
//!
//! There are many parts of the Tiled JSON that are parsed,
//! and which may be of no use to you. Where possible, there
//! are helper functions to transform some types of data in
//! to more usable data, eg: HashMaps, if desired. In all
//! other cases non-copy types are passed by value.

use serde::Deserialize;
use std::collections::HashMap;

use crate::{
    layer::Layer,
    parsers::{parse_color, parse_property},
    tile_set::TileSet,
    Color, TiledValue,
};

/// The base structure which contains all data - as in the root of a tree
#[derive(Deserialize)]
pub struct Map {
    #[serde(
        rename(deserialize = "backgroundcolor"),
        deserialize_with = "parse_color",
        default
    )]
    /// The background colour is translated from the hex representation
    pub background_color: Color,
    /// Length of the side of a hex tile in pixels
    #[serde(rename(deserialize = "hexsidelength"))]
    pub hex_side_length: Option<u32>,
    /// Whether the map has infinite dimensions
    #[serde(default)]
    pub infinite: bool,
    /// Layers are generally stored *in order*. So that the first layer in
    /// the array will be drawn first and so on. Nested layers follow the
    /// same principle.
    pub layers: Vec<Layer>,
    /// `Orthogonal`, `Isometric`, `Staggered` or `Hexagonal`
    pub orientation: Orientation,
    /// Rendering direction (orthogonal maps only)
    #[serde(rename(deserialize = "renderorder"))]
    pub render_order: Option<RenderOrder>,
    /// `X` or `Y` (staggered / hexagonal maps only)
    #[serde(rename(deserialize = "staggeraxis"))]
    pub stagger_axis: Option<StaggerAxis>,
    /// `Odd` or `Even` (staggered / hexagonal maps only)
    #[serde(rename(deserialize = "staggerindex"))]
    pub stagger_index: Option<StaggerIndex>,
    /// Height in pixels for tiles in this map
    #[serde(rename(deserialize = "tileheight"))]
    pub tile_height: u32,
    /// Width in pixels for tiles in this map
    #[serde(rename(deserialize = "tilewidth"))]
    pub tile_width: u32,
    /// The number of tile rows for the map
    pub height: u32,
    /// The number of tile columns for the map
    pub width: u32,
    #[serde(rename(deserialize = "tilesets"))]
    pub tile_sets: Vec<TileSet>,
    #[serde(deserialize_with = "parse_property", default)]
    pub properties: HashMap<String, TiledValue>,
}

/// Rendering direction. Applies only to orthogonal maps
#[derive(Deserialize, Debug, PartialEq, Clone)]
pub enum RenderOrder {
    #[serde(rename(deserialize = "right-down"))]
    RightDown,
    #[serde(rename(deserialize = "right-up"))]
    RightUp,
    #[serde(rename(deserialize = "left-down"))]
    LeftDown,
    #[serde(rename(deserialize = "left-up"))]
    LeftUp,
}

/// Applies only to staggered or hexagonal maps
#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all(deserialize = "lowercase"))]
pub enum StaggerAxis {
    X,
    Y,
}

/// Applies only to staggered or hexagonal maps
#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all(deserialize = "lowercase"))]
pub enum StaggerIndex {
    Odd,
    Even,
}

/// The orientation of the `Map`
#[derive(Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all(deserialize = "lowercase"))]
pub enum Orientation {
    Orthogonal,
    Isometric,
    Staggered,
    Hexagonal,
}
