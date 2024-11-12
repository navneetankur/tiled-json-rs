use std::path::Path;
use tiled_json_rs::{Color, LayerType, Map, TiledValue};

#[test]
fn test_load_file_small_base64_uncompressed() {
    let p = Path::new("tests/data/base64.json");
    let level = Map::load_from_file(&p).unwrap();

    let image_path = &level.tile_sets[0].image;
    assert!(image_path.ends_with("numbers.png"));

    // Group layer == 0
    if let LayerType::Group { layers } = &level.layers[0].layer_type {
        if let LayerType::ImageLayer(image) = &layers[0].layer_type {
            assert!(image.image.ends_with("bg.jpg"));
            assert_eq!(&image.transparent_color, &Color(0, 0, 0, 0));
        } else {
            panic!("First layer in group should be image layer")
        }
        if let LayerType::TileLayer(tiles) = &layers[1].layer_type {
            assert_eq!(tiles.data.len(), 504);
        } else {
            panic!("Second layer in group should be tile layer")
        }
    } else {
        panic!("First layer should be Group");
    }

    // Check tile object groups
    let tile = &level.tile_sets[0].tiles.as_ref().unwrap()[0];
    let objgrp = &tile.object_group.as_ref().unwrap().objects;
    assert_eq!(objgrp[0].height, 25.25);

    assert_eq!(level.layers.len(), 2);
}

#[test]
fn test_load_file_polys_csv() {
    let p = Path::new("tests/data/csv.json");
    let level = Map::load_from_file(&p).unwrap();

    let image_path = &level.tile_sets[0].image;
    assert!(image_path.ends_with("numbers.png"));

    assert_eq!(level.layers.len(), 2);
    assert_eq!(level.layers[0].name, "Group");

    // Check properties
    assert_eq!(level.properties["pi"], TiledValue::Float(3.14));
    assert_eq!(level.properties["bool_true"], TiledValue::Bool(true));
    assert_eq!(level.properties["bool_false"], TiledValue::Bool(false));
    assert_eq!(
        level.properties["colour"],
        TiledValue::Color(Color(161, 161, 161, 204))
    );

    // Group layer == 0
    if let LayerType::Group { layers } = &level.layers[0].layer_type {
        if let LayerType::ImageLayer(image) = &layers[0].layer_type {
            assert!(image.image.ends_with("bg.jpg"));
            assert_eq!(&image.transparent_color, &Color(0, 0, 0, 0));
        } else {
            panic!("First layer in group should be image layer")
        }
        if let LayerType::TileLayer(tiles) = &layers[1].layer_type {
            assert_eq!(tiles.data.len(), 504);
        } else {
            panic!("Second layer in group should be tile layer")
        }
    } else {
        panic!("First layer should be Group");
    }

    // Polygons etc, ObjectGroup == 1
    if let LayerType::ObjectGroup(object_layer) = &level.layers[1].layer_type {
        assert_eq!(object_layer.objects.len(), 5);
    }
    assert_eq!(level.properties["pi"], TiledValue::Float(3.14));
}

#[test]
fn test_load_file_external_tileset() {
    let p = Path::new("tests/data/external_tileset.json");
    let level = Map::load_from_file(&p).unwrap();

    let image_path = &level.tile_sets[0].image;
    assert!(image_path.ends_with("numbers.png"));

    // Group layer == 0
    if let LayerType::Group { layers } = &level.layers[0].layer_type {
        if let LayerType::ImageLayer(image) = &layers[0].layer_type {
            assert!(image.image.ends_with("bg.jpg"));
            assert_eq!(&image.transparent_color, &Color(0, 0, 0, 0));
        } else {
            panic!("First layer in group should be image layer")
        }
        if let LayerType::TileLayer(tiles) = &layers[1].layer_type {
            assert_eq!(tiles.data.len(), 504);
        } else {
            panic!("Second layer in group should be tile layer")
        }
    } else {
        panic!("First layer should be Group");
    }

    // Check tile object groups
    let tile = &level.tile_sets[0].tiles.as_ref().unwrap()[0];
    let objgrp = &tile.object_group.as_ref().unwrap().objects;
    assert_eq!(objgrp[0].height, 25.25);

    assert_eq!(level.layers.len(), 2);
}

#[test]
fn test_load_file_with_object_templates() {
    let p = Path::new("tests/data/external_objects.json");
    let level = Map::load_from_file(&p).unwrap();

    let image_path = &level.tile_sets[0].image;
    assert!(image_path.ends_with("numbers.png"));

    // Group layer == 0
    if let LayerType::Group { layers } = &level.layers[0].layer_type {
        if let LayerType::ImageLayer(image) = &layers[0].layer_type {
            assert!(image.image.ends_with("bg.jpg"));
            assert_eq!(&image.transparent_color, &Color(0, 0, 0, 0));
        } else {
            panic!("First layer in group should be image layer")
        }
        if let LayerType::TileLayer(tiles) = &layers[1].layer_type {
            assert_eq!(tiles.data.len(), 504);
        } else {
            panic!("Second layer in group should be tile layer")
        }
    } else {
        panic!("First layer should be Group");
    }

    // Check tile object groups
    let tile = &level.tile_sets[0].tiles.as_ref().unwrap()[0];
    let objgrp = &tile.object_group.as_ref().unwrap().objects;
    assert_eq!(objgrp[0].height, 25.25);

    assert_eq!(level.layers.len(), 2);
}
