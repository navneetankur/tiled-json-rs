use tiled_json_rs::{Object, TiledValue};

#[test]
fn object() {
    let data = r##"
        {
          "gid":5,
          "height":0,
          "id":1,
          "name":"villager",
          "properties":[
            {
              "name":"hp",
              "type":"int",
              "value":12
            }],
          "rotation":0,
          "type":"npc",
          "visible":true,
          "width":0,
          "x":32,
          "y":32
        }"##;
    let m: Object = serde_json::from_str(data).expect("fail");

    assert_eq!(m.properties["hp"], TiledValue::Int(12));
}

#[test]
fn ellipse() {
    let data = r##"
        {
          "ellipse":true,
          "height":152,
          "id":13,
          "name":"",
          "rotation":0,
          "type":"",
          "visible":true,
          "width":248,
          "x":560,
          "y":808
        }"##;
    assert!(serde_json::from_str::<Object>(data).is_ok());
}

#[test]
fn rectangle() {
    let data = r##"
        {
          "height":184,
          "id":14,
          "name":"",
          "rotation":0,
          "type":"",
          "visible":true,
          "width":368,
          "x":576,
          "y":584
        }"##;
    assert!(serde_json::from_str::<Object>(data).is_ok());
}

#[test]
fn point() {
    let data = r##"
        {
          "point":true,
          "height":0,
          "id":20,
          "name":"",
          "rotation":0,
          "type":"",
          "visible":true,
          "width":0,
          "x":220,
          "y":350
        }"##;
    assert!(serde_json::from_str::<Object>(data).is_ok());
}

#[test]
fn polygon() {
    let data = r##"
        {
          "height":0,
          "id":15,
          "name":"",
          "polygon":[
          {
            "x":0,
            "y":0
          },
          {
            "x":152,
            "y":88
          },
          {
            "x":136,
            "y":-128
          },
          {
            "x":80,
            "y":-280
          },
          {
            "x":16,
            "y":-288
          }],
          "rotation":0,
          "type":"",
          "visible":true,
          "width":0,
          "x":-176,
          "y":432
        }"##;
    assert!(serde_json::from_str::<Object>(data).is_ok());
}

#[test]
fn polyline() {
    let data = r##"
        {
          "height":0,
          "id":16,
          "name":"",
          "polyline":[
          {
            "x":0,
            "y":0
          },
          {
            "x":248,
            "y":-32
          },
          {
            "x":376,
            "y":72
          },
          {
            "x":544,
            "y":288
          },
          {
            "x":656,
            "y":120
          },
          {
            "x":512,
            "y":0
          }],
          "rotation":0,
          "type":"",
          "visible":true,
          "width":0,
          "x":240,
          "y":88
        }"##;
    assert!(serde_json::from_str::<Object>(data).is_ok());
}

#[test]
fn text() {
    let data = r##"
        {
          "height":19,
          "id":15,
          "name":"",
          "text":
          {
            "text":"Hello World",
            "wrap":true
          },
          "rotation":0,
          "type":"",
          "visible":true,
          "width":248,
          "x":48,
          "y":136
        }"##;
    assert!(serde_json::from_str::<Object>(data).is_ok());
}
