[![Build Status](https://drone.systemscoder.nz/api/badges/luke/tiled-json/status.svg)](https://drone.systemscoder.nz/luke/tiled-json)

# Parser for Tiled Editor JSON

[**Documentation**](https://docs.rs/tiled-json-rs)

## Usage

**Cargo.toml**

```
[dependencies]
tiled-json-rs = "0.1.0"
```

**Load from a file**

```
let map = tiled::Map::load_from_file(PathBuf::from("assets/test.json"))
            .expect("Failed to load map");
```

**Iterating over a tileset**

```
for tileset in &map.tile_sets {
    let name = tileset.name.clone();
    let mut path = PathBuf::from("assets");
    path.push(tileset.image.clone());
    // Do stuff
}
```

**Iterating over layers**

```
fn render_layers(
        layers: &Vec<Layer>,
    ) {
    for layer in layers {
        match &layer.layer_type {
            tiled::LayerType::TileLayer(tiles) => {
                do_something(tiles);
            }
            tiled::LayerType::Group { layers } => {
                &mut render_layers(layers);
            }
            tiled::LayerType::ImageLayer(image) => {
                do_something_else(image);
            }
            tiled::LayerType::ObjectGroup(objects) => {
                and_another_thing(objects);
            }
        }
    }
}

render_layers(&map.layers);
```

### Notes:

- GID for tiles starts at 1 with 0 reserved for *empty* tile
- Local Id starts at 0 for `TileSet`, and only applies to `TileSet`
- The tileset must be included in the JSON (this is temporary until parsing the path is done)
- Doc comments are only provided where clarification may be useful. In general things
  should be named well enough that intention is self-describing.

## License

MPL v2
