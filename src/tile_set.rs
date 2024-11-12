/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use crate::{
    layer::ObjectGroup,
    parsers::{parse_color, parse_path, parse_property},
    wangs::WangSet,
    Color, TiledValue, Vec2,
};
use serde::{Deserialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Deserialize)]
#[serde(untagged)]
pub enum TileSet {
    Internal(Internal),
    External(External),
}
impl TileSet {
    pub fn internal(&self) -> &Internal {
        match self {
           TileSet::Internal(i)  => i,
           _ => panic!("external tileset."),
        }
    }
 }
#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct External {
    #[serde(rename(deserialize = "firstgid"))]
    pub first_gid: u32,
    pub source: PathBuf,
}
#[derive(Deserialize)]
#[derive(Debug, PartialEq, Clone)]
pub struct Internal {
    /// The number of tile columns in the tileset. Eg; dividing the
    /// associated image in to columns where each column is the width
    /// of the tile.
    pub columns: u32,
    /// GID corresponding to the first tile in the set
    #[serde(rename(deserialize = "firstgid"), default)]
    pub first_gid: u32,
    /// Path to the image used for tiles in this set
    #[serde(deserialize_with = "parse_path")]
    pub image: PathBuf,
    #[serde(rename(deserialize = "imagewidth"))]
    pub image_width: u32,
    #[serde(rename(deserialize = "imageheight"))]
    pub image_height: u32,
    /// Buffer between image edge and first tile in pixels
    pub margin: u32,
    /// Spacing between adjacent tiles in image in pixels
    pub spacing: u32,
    pub name: String,
    #[serde(deserialize_with = "parse_property", default)]
    pub properties: HashMap<String, TiledValue>,
    pub terrains: Option<Vec<Terrain>>,
    /// The tile count + the first GID enable finding the tile location
    /// on the image
    #[serde(rename(deserialize = "tilecount"))]
    pub tile_count: u32,
    #[serde(rename(deserialize = "tileheight"))]
    pub tile_height: u32,
    #[serde(rename(deserialize = "tilewidth"))]
    pub tile_width: u32,
    /// used to specify an offset in pixels, to be applied
    /// when drawing a tile from this tileset
    #[serde(rename(deserialize = "tileoffset"))]
    pub tile_offset: Option<Vec2<i32>>,
    /// Holds *extra* information for tiles such as terrain or animation
    pub tiles: Option<Vec<Tile>>,
    #[serde(
        rename(deserialize = "transparentcolor"),
        deserialize_with = "parse_color",
        default
    )]
    /// Defaults to 0,0,0,0 (rgba)
    pub transparent_color: Color,
    #[serde(rename(deserialize = "wangsets"))]
    pub wang_sets: Option<Vec<WangSet>>,
}
/// Contains all possible data for a tile including an optional `ObjectGroup`
#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Tile {
    pub animation: Option<Vec<Frame>>,
    /// Unlike the ID used in the `TileLayer`, this ID is
    /// local to the `TileSet` only and so starts at 0 (the
    /// tile layer ID starts a 1 for tiles with 0 being no-tile).
    pub id: u32,
    /// Image representing this tile if it uses a separate image
    pub image: Option<String>,
    /// Width of the tile image in pixels
    #[serde(rename(deserialize = "imagewidth"), default)]
    pub image_width: u32,
    /// Height of the tile image in pixels
    #[serde(rename(deserialize = "imageheight"), default)]
    pub image_height: u32,
    #[serde(rename(deserialize = "objectgroup"))]
    pub object_group: Option<ObjectGroup>,
    #[serde(deserialize_with = "parse_property", default)]
    pub properties: HashMap<String, TiledValue>,
    /// The order of indices is: top-left, top-right, bottom-left, bottom-right
    ///
    /// Each entry is the index number in to the Terrain array to get the
    /// specific terrain type for this tile. Typically used in conjunction
    /// with the tileset structure as the terrain tiles are stored within
    /// the data there.
    pub terrain: Option<[i8; 4]>,
    /// An optional string for describing a type
    #[serde(rename(deserialize = "type"))]
    pub tile_type: Option<String>,
}

/// Data for an individual frame of animation
#[derive(Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Frame {
    /// Frame duration in milliseconds
    pub duration: u32,
    /// Local tile ID representing this frame
    #[serde(rename(deserialize = "tileid"))]
    pub tile_id: u32,
}

#[derive(Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Terrain {
    pub name: String,
    /// Local ID of the tile for this terrain within the tileset
    pub tile: u32,
}
