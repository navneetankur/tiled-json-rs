/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! A handy crate for parsing the Tiled JSON data in to a usable structure.
//!
//! The crate includes a few small helper functions on `Map`, `TileSet`, and
//! `TileLayer`. These functions are for common tasks such as generating a
//! cloumn/row location (tiles are stored in a 1D array), a located box on an
//! image for helping with tile-to-tilesheet image picking, and loading files
//! (or strings).
//!
//! # Examples
//!
//! ```
//! # use std::path::PathBuf;
//! use tiled_json_rs as tiled;
//! let map = tiled::Map::load_from_file(&PathBuf::from("tests/data/csv.json"))
//!            .expect("Failed to load map");
//! ```
//!
//! ```
//! # use std::path::PathBuf;
//! # use tiled_json_rs as tiled;
//! # let map = tiled::Map::load_from_file(&PathBuf::from("tests/data/csv.json")).unwrap();
//! for tileset in &map.tile_sets {
//!     let name = tileset.name.clone();
//!     let mut path = PathBuf::from("assets");
//!     path.push(tileset.image.clone());
//!     // Do stuff
//! }
//! ```
//!
//! ```
//! # use std::path::PathBuf;
//! # use tiled_json_rs as tiled;
//! # let map = tiled::Map::load_from_file(&PathBuf::from("tests/data/csv.json")).unwrap();
//! use tiled::Layer;
//!
//! fn render_layers(layers: &Vec<Layer>) {
//!     for layer in layers {
//!         match &layer.layer_type {
//!             tiled::LayerType::TileLayer(tiles) => {
//!                 //do_something(tiles);
//!             }
//!             tiled::LayerType::Group { layers } => {
//!                 &mut render_layers(layers);
//!             }
//!             tiled::LayerType::ImageLayer(image) => {
//!                 //do_something_else(image);
//!             }
//!             tiled::LayerType::ObjectGroup(objects) => {
//!                 //and_another_thing(objects);
//!             }
//!         }
//!     }
//! }
//!
//! render_layers(&map.layers);
//! ```
//!
//! ## Info
//!
//! Tiled can export maps as JSON files. To do so, simply select “File > Export As”
//! and select the JSON file type. You can export json from the command line with
//! the `--export-map` option.
//!
//! Notes:
//! - GID for tiles starts at 1 with 0 reserved for *empty* tile
//! - Local Id starts at 0 for `TileSet`, and only applies to `TileSet`
//! - Doc comments are only provided where clarification may be useful. In general things
//!  should be named well enough that intention is self-describing.
//!

use std::path::Path;

use serde::Deserialize;

mod layer;
mod map;
mod object;
mod tile_set;
mod utils;
mod wangs;

pub use layer::*;
pub use map::*;
pub use object::*;
pub use tile_set::*;
pub use utils::*;
pub use wangs::*;

mod parsers;

use parsers::parse_color;

/// A `TiledValue` is similar to JSON values.
///
/// It contains the basic types that Tiled uses.
/// This is generally used in the properties of layers, tiles, and objects.
#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all(deserialize = "lowercase"), tag = "type", content = "value")]
pub enum TiledValue {
    Bool(bool),
    Float(f32),
    Int(u32),
    #[serde(deserialize_with = "parse_color")]
    Color(Color),
    String(String),
    File(String),
    Class(serde_json::Value),
}


/// A simple representation of a 2d Vector to pass coords around
#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

/// An RGBA representation of colours
///
/// Order of colours in the tuple follow the Red-Green-Blue-Alpha pattern
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Color(pub u32, pub u32, pub u32, pub u32);

impl Color {
    /// Red
    pub fn r(&self) -> u32 {
        self.0
    }

    /// Green
    pub fn g(&self) -> u32 {
        self.1
    }

    /// Blue
    pub fn b(&self) -> u32 {
        self.2
    }

    /// Alpha
    pub fn a(&self) -> u32 {
        self.3
    }
}

/// Used to provide the location and dimensions of the required
/// tile on the tiles tileset image.
///
/// Functionally similar to SDL2 Rect.
#[derive(Debug, Default, PartialEq, Clone)]
pub struct TileRect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}
