use tiled_json_rs::{Color, TileSet};

#[test]
fn tileset() {
    let data = r##"
        {
         "columns":19,
         "firstgid":1,
         "image":"..\/image\/fishbaddie_parts.png",
         "imageheight":480,
         "imagewidth":640,
         "margin":3,
         "name":"",
         "properties":[
           {
             "name":"myProperty1",
             "type":"string",
             "value":"myProperty1_value"
           }],
         "spacing":1,
         "tilecount":266,
         "tileheight":32,
         "tilewidth":32
        }"##;
    assert!(serde_json::from_str::<TileSet>(data).is_ok());
}

#[test]
fn tile_set_tiles() {
    let data = r##"
     {
         "columns":19,
         "firstgid":1,
         "image":"..\/image\/fishbaddie_parts.png",
         "imageheight":480,
         "imagewidth":640,
         "margin":3,
         "name":"",
         "properties":[
           {
             "name":"myProperty1",
             "type":"string",
             "value":"myProperty1_value"
           }],
         "spacing":1,
         "tilecount":266,
         "tileheight":32,
         "tilewidth":32,
         "tiles":[
              {
                "id":0,
                "properties":[
                  {
                    "name":"myProperty1",
                    "type":"string",
                    "value":"myProperty1_value"
                  },
                  {
                    "name":"color",
                    "type":"color",
                    "value":"#ffff2a9c"
                  }],
                "terrain":[0, 0, 0, 0]
              },
              {
                "id":11,
                "properties":[
                  {
                    "name":"myProperty2",
                    "type":"string",
                    "value":"myProperty2_value"
                  }],
                "terrain":[0, 1, 0, 1]
              },
              {
                "id":12,
                "properties":[
                  {
                    "name":"myProperty3",
                    "type":"string",
                    "value":"myProperty3_value"
                  }],
                "terrain":[1, 1, 1, 1]
              }
         ]
      }"##;
    assert!(serde_json::from_str::<TileSet>(data).is_ok());
}

#[test]
fn tileset_wang() {
    let data = r##"
        {
         "columns":19,
         "firstgid":1,
         "image":"..\/image\/fishbaddie_parts.png",
         "imageheight":480,
         "imagewidth":640,
         "margin":3,
         "name":"",
         "properties":[
           {
             "name":"myProperty1",
             "type":"string",
             "value":"myProperty1_value"
           }],
         "spacing":1,
         "tilecount":266,
         "tileheight":32,
         "tilewidth":32,
         "wangsets":[
           {
             "cornercolors":[{
                  "color": "#d31313",
                  "name": "Rails",
                  "probability": 1,
                  "tile": 18
                }],
             "edgecolors":[{
                  "color": "#d31313",
                  "name": "Rails",
                  "probability": 1,
                  "tile": 18
                }],
             "name":"some_name",
             "tile":42,
             "wangtiles":[{
                  "dflip": false,
                  "hflip": false,
                  "tileid": 0,
                  "vflip": false,
                  "wangid": [2, 0, 1, 0, 1, 0, 2, 0]
                }]
           }]
        }"##;

    let m = serde_json::from_str::<TileSet>(data).unwrap();
    assert!(serde_json::from_str::<TileSet>(data).is_ok());

    assert_eq!(
        m.wang_sets.as_ref().unwrap()[0].corner_colors[0].color.r(),
        61
    );

    assert_eq!(
        m.wang_sets.as_ref().unwrap()[0].corner_colors[0].color,
        Color(61, 49, 49, 255)
    );

    assert_eq!(
        m.wang_sets.unwrap()[0].wang_tiles[0].wang_id,
        [2, 0, 1, 0, 1, 0, 2, 0]
    );
}
