/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use crate::{
    layer::ObjectGroup,
    parsers::{parse_color, parse_path, parse_property},
    wangs::WangSet,
    Color, TiledValue, Vec2,
};
use serde::de::Error;
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;

/// A tileset that associates information with each tile.
///
/// A tileset associates information with each tile such as
/// image path or terrain type, may include a tiles array property.
/// Each tile in the `tiles` member has a local id property which
/// specifies the local ID within the tileset.
///
/// Tile sets may be internal to the map, or external files.
#[derive(Debug, PartialEq, Clone)]
pub struct TileSet {
    /// The number of tile columns in the tileset. Eg; dividing the
    /// associated image in to columns where each column is the width
    /// of the tile.
    pub columns: u32,
    /// GID corresponding to the first tile in the set
    pub first_gid: u32,
    /// Path to the image used for tiles in this set
    pub image: PathBuf,
    pub image_width: u32,
    pub image_height: u32,
    /// Buffer between image edge and first tile in pixels
    pub margin: u32,
    /// Spacing between adjacent tiles in image in pixels
    pub spacing: u32,
    pub name: String,
    pub properties: HashMap<String, TiledValue>,
    pub terrains: Option<Vec<Terrain>>,
    /// The tile count + the first GID enable finding the tile location
    /// on the image
    pub tile_count: u32,
    pub tile_height: u32,
    pub tile_width: u32,
    /// used to specify an offset in pixels, to be applied
    /// when drawing a tile from this tileset
    pub tile_offset: Option<Vec2<i32>>,
    /// Holds *extra* information for tiles such as terrain or animation
    pub tiles: Option<Vec<Tile>>,
    /// Defaults to 0,0,0,0 (rgba)
    pub transparent_color: Color,
    pub wang_sets: Option<Vec<WangSet>>,
}

impl<'de> Deserialize<'de> for TileSet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "lowercase")]
        struct External {
            #[serde(rename(deserialize = "firstgid"))]
            first_gid: u32,
            source: String,
        }

        #[derive(Deserialize)]
        struct Internal {
            columns: u32,
            #[serde(rename(deserialize = "firstgid"), default)]
            first_gid: u32,
            #[serde(deserialize_with = "parse_path")]
            image: PathBuf,
            #[serde(rename(deserialize = "imagewidth"))]
            image_width: u32,
            #[serde(rename(deserialize = "imageheight"))]
            image_height: u32,
            margin: u32,
            spacing: u32,
            name: String,
            #[serde(deserialize_with = "parse_property", default)]
            properties: HashMap<String, TiledValue>,
            terrains: Option<Vec<Terrain>>,
            #[serde(rename(deserialize = "tilecount"))]
            tile_count: u32,
            #[serde(rename(deserialize = "tileheight"))]
            tile_height: u32,
            #[serde(rename(deserialize = "tilewidth"))]
            tile_width: u32,
            #[serde(rename(deserialize = "tileoffset"))]
            tile_offset: Option<Vec2<i32>>,
            tiles: Option<Vec<Tile>>,
            #[serde(
                rename(deserialize = "transparentcolor"),
                deserialize_with = "parse_color",
                default
            )]
            transparent_color: Color,
            #[serde(rename(deserialize = "wangsets"))]
            wang_sets: Option<Vec<WangSet>>,
        }

        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Helper {
            Internal(Internal),
            External(External),
        }

        let v = serde_json::Value::deserialize(deserializer)?;
        if let Ok(m) = Helper::deserialize(&v) {
            let t = match m {
                Helper::Internal(v) => v,
                Helper::External(v) => {
                    let path = PathBuf::from(v.source);
                    // let file = File::open(path)
                    //     .map_err(|e| Error::custom(format!("{:?}", e)))?;
                    let file = File::open(path).unwrap();
                    let mut set: Internal = serde_json::from_reader(file)
                        .map_err(|e| Error::custom(format!("{:?}", e)))?;
                    set.first_gid = v.first_gid;
                    set
                }
            };
            let tile_set = TileSet {
                columns: t.columns,
                first_gid: t.first_gid,
                image: t.image,
                image_width: t.image_width,
                image_height: t.image_height,
                margin: t.margin,
                spacing: t.spacing,
                name: t.name,
                properties: t.properties,
                terrains: t.terrains,
                tile_count: t.tile_count,
                tile_height: t.tile_height,
                tile_width: t.tile_width,
                tile_offset: t.tile_offset,
                tiles: t.tiles,
                transparent_color: t.transparent_color,
                wang_sets: t.wang_sets,
            };
            return Ok(tile_set);
        } else {
            Err(Error::custom("could not parse tile-set"))
        }
    }
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
