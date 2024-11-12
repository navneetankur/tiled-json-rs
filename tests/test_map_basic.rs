use tiled_json_rs::{Color, Map, Orientation, RenderOrder, TiledValue};

#[test]
fn parse_map_basic() {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r##"
        {
          "backgroundcolor":"#656667",
          "height":4,
          "layers":[ ],
          "nextobjectid":1,
          "orientation":"orthogonal",
          "properties":[
            {
              "name":"mapProperty1",
              "type":"string",
              "value":"string"
            },
            {
              "name":"mapProperty2",
              "type":"string",
              "value":"string"
            }],
          "renderorder":"right-down",
          "tileheight":32,
          "tilesets":[ ],
          "tilewidth":32,
          "version":1,
          "tiledversion":"1.0.3",
          "width":4
        }"##;

    let m: Map = serde_json::from_str(data).expect("fail");

    assert_eq!(&m.background_color, &Color(86, 102, 118, 255));

    assert!(m.hex_side_length.is_none());

    assert!(!m.infinite);

    assert_eq!(m.layers.len(), 0);

    assert_eq!(&m.orientation, &Orientation::Orthogonal);

    assert_ne!(m.properties.len(), 0);
    assert_eq!(
        m.properties["mapProperty1"],
        TiledValue::String("string".to_string())
    );

    assert!(m.render_order.is_some());
    assert_eq!(&m.render_order.unwrap(), &RenderOrder::RightDown);

    assert!(m.stagger_axis.is_none());
    assert!(m.stagger_index.is_none());

    assert_eq!(m.tile_height, 32);
    assert_eq!(m.tile_width, 32);

    assert_eq!(m.tile_sets.len(), 0);

    assert_eq!(m.height, 4);
    assert_eq!(m.width, 4);
}
