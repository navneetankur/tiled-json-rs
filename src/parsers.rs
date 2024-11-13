/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use crate::{Color, TiledValue};
use base64;
use serde::de::{self, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;
use std::fmt;
use std::path::{Path, PathBuf};

pub fn parse_tileset_tiles<'de, D>(
    de: D,
) -> Result<HashMap<u16, crate::Tile>, D::Error>
where
    D: Deserializer<'de>,
{
    todo!()
}

pub fn parse_property<'de, D>(
    de: D,
) -> Result<HashMap<String, TiledValue>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Debug, Deserialize)]
    struct Helper {
        name: String,
        #[serde(flatten)]
        value: TiledValue,
    }

    struct SomeVisitor;

    impl<'de> Visitor<'de> for SomeVisitor {
        type Value = HashMap<String, TiledValue>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a nonempty sequence of {name,type,value}")
        }

        fn visit_seq<S>(
            self,
            mut seq: S,
        ) -> Result<HashMap<String, TiledValue>, S::Error>
        where
            S: SeqAccess<'de>,
        {
            let mut map = HashMap::<String, TiledValue>::new();
            while let Some(pair) = seq.next_element::<Helper>()? {
                map.insert(pair.name, pair.value);
            }

            Ok(map)
        }
    }

    let visitor = SomeVisitor;
    de.deserialize_seq(visitor)
}

pub fn parse_data<'de, D>(de: D) -> Result<Vec<u32>, D::Error>
where
    D: Deserializer<'de>,
{
    struct SomeVisitor;

    impl<'de> Visitor<'de> for SomeVisitor {
        type Value = Vec<u32>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("enum of either array of u32 or string")
        }

        fn visit_str<E>(self, value: &str) -> Result<Vec<u32>, E>
        where
            E: de::Error,
        {
            // TODO: decompression
            let decoded: Vec<u8> =
                base64::decode(value).expect("Could not decode base64");
            let mut corrected = Vec::with_capacity(decoded.len() / 4);
            for chunk in decoded.chunks(4) {
                let res = (chunk[0] as u32)
                    + ((chunk[1] as u32) << 8)
                    + ((chunk[2] as u32) << 16)
                    + ((chunk[3] as u32) << 24);
                corrected.push(res);
            }
            Ok(corrected)
        }

        fn visit_seq<S>(self, mut seq: S) -> Result<Vec<u32>, S::Error>
        where
            S: SeqAccess<'de>,
        {
            let mut v = Vec::<u32>::new();
            while let Some(val) = seq.next_element::<u32>()? {
                v.push(val);
            }
            Ok(v)
        }
    }

    let visitor = SomeVisitor;

    de.deserialize_any(visitor)
}

pub fn parse_color<'de, D>(de: D) -> Result<Color, D::Error>
where
    D: Deserializer<'de>,
{
    struct SomeVisitor;

    impl<'de> Visitor<'de> for SomeVisitor {
        type Value = Color;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("color as a string of hex")
        }

        fn visit_str<E>(self, value: &str) -> Result<Color, E>
        where
            E: de::Error,
        {
            let value = if value.len() < 9 {
                format!("#ff{}", &value[1..])
            } else {
                value.to_string()
            };
            // Tiled puts alpha first
            let alpha = &value[1..=2];
            let red = &value[3..=4];
            let green = &value[5..=6];
            let blue = &value[7..=8];
            Ok(Color(
                parse_hex(red),
                parse_hex(green),
                parse_hex(blue),
                parse_hex(alpha),
            ))
        }
    }

    let visitor = SomeVisitor;
    de.deserialize_str(visitor)
}

fn parse_hex(hex: &str) -> u32 {
    let mut dec = 0;
    let mut weight = 1;
    for c in hex.chars() {
        dec += weight * c.to_digit(16).unwrap();
        weight *= 16;
    }
    dec
}

pub fn parse_path<'de, D>(de: D) -> Result<PathBuf, D::Error>
where
    D: Deserializer<'de>,
{
    struct SomeVisitor;

    impl<'de> Visitor<'de> for SomeVisitor {
        type Value = PathBuf;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string path to file")
        }

        fn visit_str<E>(self, value: &str) -> Result<PathBuf, E>
        where
            E: de::Error,
        {
            let p = Path::new(value);
            let p = p.to_path_buf();
            Ok(p)
        }
    }

    let visitor = SomeVisitor;
    de.deserialize_str(visitor)
}

#[cfg(test)]
mod tests {
    #[test]
    fn hex_check() {
        use crate::parsers::parse_hex;
        assert_eq!(parse_hex("ffffff"), 16777215);
    }
}
