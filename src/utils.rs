use crate::{layer::Layer, map::Map, TileLayer, TileRect, TileSet, Vec2};
use std::fs::File;
use std::io::Error;
use std::path::Path;

impl Map {
    pub fn load_from_file(path: &Path) -> Result<Self, Error> {
        let file = File::open(path)?;
        let set = serde_json::from_reader(file)?;
        Ok(set)
    }

    pub fn load_from_str(s: &str) -> Result<Self, Error> {
        let set = serde_json::from_str(s)?;
        Ok(set)
    }

    /// Returns the image path for the image the tile is using
    pub fn tileset_image_path(&self, tile_gid: u32) -> Option<&Path> {
        for set in &self.tile_sets {
            let set = set.internal();
            if tile_gid >= set.first_gid
                && tile_gid < set.tile_count + set.first_gid
            {
                return Some(set.image.as_path());
            }
        }
        None
    }

    /// Return the name of the tileset the tile is from
    pub fn tileset_name(&self, tile_gid: u32) -> Option<&str> {
        for set in &self.tile_sets {
            let set = set.internal();
            if tile_gid >= set.first_gid
                && tile_gid < set.tile_count + set.first_gid
            {
                return Some(&set.name);
            }
        }
        None
    }

    /// Returns the position and dimensions of the tile GID on its associated image.
    /// Used for drawing tiles, eg; using SDL2 to blit this tile from an image surface.
    pub fn tile_position_on_image(&self, mut tile_gid: u32) -> TileRect {
        let mut tileset = self.tile_sets[0].internal();
        for set in &self.tile_sets {
            let set = set.internal();
            if tile_gid >= set.first_gid
                && tile_gid < set.tile_count + set.first_gid
            {
                tileset = set;
                tile_gid -= set.first_gid;
                break;
            }
        }
        tileset.tile_position_on_image(tile_gid)
    }

    /// Returns the tile position in pixels on the current map
    pub fn tile_position_on_map(&self, count: u32, tile_gid: u32) -> Vec2<u32> {
        let mut tileset = self.tile_sets[0].internal();
        for set in &self.tile_sets {
            let set = set.internal();
            if tile_gid >= set.first_gid
                && tile_gid < set.tile_count + set.first_gid
            {
                tileset = set;
                break;
            }
        }
        let x = count % self.width * tileset.tile_width;
        let y = count / self.width * tileset.tile_width;
        Vec2 { x, y }
    }
}

impl crate::tile_set::Internal {
    /// Returns the tile position and extents for it's location
    /// on the source image. Useful for creating textures/blits.
    pub fn tile_position_on_image(&self, local_id: u32) -> TileRect {
        let min_x = local_id % self.columns * self.tile_width;
        let min_y = local_id / self.columns * self.tile_width;
        TileRect {
            x: min_x as i32,
            y: min_y as i32,
            width: self.tile_width,
            height: self.tile_height,
        }
    }
}

impl TileLayer {
    /// Returns the tiles position in tile column/row. To get a pixel dimension
    /// multiply this by the tile dimensions
    pub fn tile_position_on_layer(&self, count: u32) -> Vec2<u32> {
        let x = count % self.width;
        let y = count / self.width;
        Vec2 { x, y }
    }
}
