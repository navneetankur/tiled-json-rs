use tiled_json_rs::{Chunk, Color, DrawOrder, Layer, LayerType, TiledValue};

#[test]
fn parse_tile_layer_no_encoding() {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r##"
        {
          "data":[1, 2, 1, 2, 3, 1, 3, 1, 2, 2, 3, 3, 4, 4, 4, 1],
          "height":4,
          "name":"ground",
          "opacity":1,
          "properties":[
            {
              "name":"tileLayerProp",
              "type":"int",
              "value":1
            },
            {
                "name": "test",
                "type": "bool",
                "value": false
            }],
          "type":"tilelayer",
          "visible":true,
          "width":4,
          "x":0,
          "y":0
        }"##;

    let m: Layer = serde_json::from_str(data).expect("fail");

    match m.layer_type {
        LayerType::TileLayer(data) => {
            assert!(data.chunks.is_none());
            assert_eq!(data.data.len(), 16);
        }
        _ => {}
    }

    assert_eq!(m.properties["test"], TiledValue::Bool(false));
    assert_eq!(m.properties["tileLayerProp"], TiledValue::Int(1));
}

#[test]
fn parse_tile_layer_encoding() {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r##"
        {
          "data":"UQAAAFMAAAAsAAAALAAAACwAAAAsAAAALAAAACwAAAAsAAAALAAAACwAAAA", 
          "encoding": "base64",
          "height":4,
          "name":"ground",
          "opacity":1,
          "properties":[
          ],
          "type":"tilelayer",
          "visible":true,
          "width":4,
          "x":0,
          "y":0
        }"##;

    let m: Layer = serde_json::from_str(data).expect("fail");

    match m.layer_type {
        LayerType::TileLayer(data) => {
            assert_eq!(data.data.len(), 11);
        }
        _ => {}
    }
}

#[test]
fn check_property_variants() {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r##"
        {
          "draworder":"topdown",
          "height":0,
          "name":"people",
          "objects":[ ],
          "opacity":1,
          "properties":[
            {
              "name":"layerProp1",
              "type":"bool",
              "value":true
            },
            {
              "name":"layerProp2",
              "type":"float",
              "value":1.6
            },
            {
              "name":"layerProp3",
              "type":"int",
              "value":1
            },
            {
              "name":"layerProp4",
              "type":"color",
              "value": "#fff22a9c"
            },
            {
              "name":"layerProp5",
              "type":"string",
              "value":"naff"
            }],
          "type":"objectgroup",
          "visible":true,
          "width":0,
          "x":0,
          "y":0
        }"##;

    let m: Layer = serde_json::from_str(data).expect("fail");

    match m.layer_type {
        LayerType::ObjectGroup(data) => {
            assert_eq!(&data.draw_order, &DrawOrder::TopDown);
        }
        _ => {}
    }

    assert_eq!(m.properties["layerProp1"], TiledValue::Bool(true));
    assert_eq!(m.properties["layerProp2"], TiledValue::Float(1.6));
    assert_eq!(m.properties["layerProp3"], TiledValue::Int(1));
    match &m.properties["layerProp4"] {
        TiledValue::Color(c) => {
            assert_eq!(c.r(), 47);
            assert_eq!(c.g(), 162);
            assert_eq!(c.b(), 201);
            assert_eq!(c.a(), 255);
        }
        _ => panic!("shouldn't be a color"),
    }
    assert_eq!(
        m.properties["layerProp5"],
        TiledValue::String("naff".to_string())
    );
}

#[test]
fn check_layer_type_variations() {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r##"
        {
          "height":4,
          "name":"ground",
          "opacity":1,
          "properties":[
          ],
          "type":"tilelayer",
          "visible":true,
          "width":4,
          "x":0,
          "y":0
        }"##;
    let m: Layer = serde_json::from_str(data).expect("fail");
    if let LayerType::TileLayer(data) = m.layer_type {
        assert_eq!(data.data.len(), 0)
    } else {
        panic!("Should have been a LayerType::TileLayer");
    }

    let data = r##"
        {
          "height":4,
          "name":"ground",
          "opacity":1,
          "properties":[
          ],
          "type":"objectgroup",
          "visible":true,
          "width":4,
          "x":0,
          "y":0
        }"##;
    let m: Layer = serde_json::from_str(data).expect("fail");
    if let LayerType::ObjectGroup(data) = &m.layer_type {
        assert_eq!(data.objects.len(), 0)
    } else {
        panic!("Should have been a LayerType::ObjectGroup");
    }

    let data = r##"
        {
          "height":4,
          "name":"ground",
          "opacity":1,
          "properties":[
          ],
          "type":"imagelayer",
          "visible":true,
          "width":4,
          "x":0,
          "y":0
        }"##;
    let m: Layer = serde_json::from_str(data).expect("fail");
    if let LayerType::ImageLayer(data) = &m.layer_type {
        assert_eq!(&data.transparent_color, &Color(0, 0, 0, 0))
    } else {
        panic!("Should have been a LayerType::ObjectGroup");
    }
}

#[test]
fn chunk() {
    let data = r##"{
          "data":[1, 2, 1, 2, 3, 1, 3, 1, 2, 2, 3, 3, 4, 4, 4, 1],
          "height":16,
          "width":16,
          "x":0,
          "y":-16
        }"##;
    let m: Chunk = serde_json::from_str(data).expect("fail");

    assert_eq!(m.data.len(), 16);
    assert_eq!(m.data[2], 1);
    assert_eq!(m.y, -16);
}
