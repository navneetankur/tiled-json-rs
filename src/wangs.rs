/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use crate::{parsers::parse_color, Color};
use serde::Deserialize;

/// Data set for `Wang` tiles
///
/// Wang tiles are similar in concept to Terrains. They are, however, more
/// focused on filling larger areas without repetition. One defines the edge
/// and corner colors of tiles in a tileset. This information can then be used
/// when filling, or brushing to allow for smooth, non-repetitive transitions
/// between tiles. In most cases this tiling is random, and based on color
/// probability.
#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct WangSet {
    #[serde(rename(deserialize = "cornercolors"))]
    pub corner_colors: Vec<WangColor>,
    #[serde(rename(deserialize = "edgecolors"))]
    pub edge_colors: Vec<WangColor>,
    pub name: String,
    /// Local ID of tile representing the Wang set
    pub tile: u32,
    #[serde(rename(deserialize = "wangtiles"))]
    pub wang_tiles: Vec<WangTile>,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct WangColor {
    #[serde(deserialize_with = "parse_color")]
    pub color: Color,
    pub name: String,
    /// Probability used when randomizing
    pub probability: f32,
    /// Local ID of tile representing the Wang color
    pub tile: u32,
}

#[derive(Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct WangTile {
    /// Tile is flipped diagonally
    #[serde(rename(deserialize = "dflip"))]
    pub d_flip: bool,
    /// Tile is flipped horizontally
    #[serde(rename(deserialize = "hflip"))]
    pub h_flip: bool,
    /// Tile is flipped vertically
    #[serde(rename(deserialize = "vflip"))]
    pub v_flip: bool,
    /// Local ID of tile
    #[serde(rename(deserialize = "tileid"))]
    pub tile_id: u32,
    /// Array of Wang color indexes
    #[serde(rename(deserialize = "wangid"))]
    pub wang_id: Vec<u8>,
}
