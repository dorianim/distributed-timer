use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let buf = String::deserialize(deserializer)?;

        if buf.len() != 7 {
            return Err(serde::de::Error::custom("color must be 7 chars long"));
        }

        let mut chars = buf.chars().collect::<Vec<char>>();

        if chars.remove(0) != '#' {
            return Err(serde::de::Error::custom("color must start with #"));
        }

        let colors = chars
            .chunks(2)
            .map(|c| c.iter().collect::<String>())
            .map(|s| parse_color(&s))
            .collect::<Result<Vec<u8>, D::Error>>()?;

        Ok(Color {
            r: colors[0],
            g: colors[1],
            b: colors[2],
        })
    }
}

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let string = format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b);
        serializer.serialize_str(&string)
    }
}

fn parse_color<E>(hex_code: &str) -> Result<u8, E>
where
    E: serde::de::Error,
{
    let res = u8::from_str_radix(hex_code, 16);
    if res.is_err() {
        return Err(E::custom("could not deserialize color"));
    }

    Ok(res.unwrap())
}

#[cfg(test)]
#[test]
fn test_serialize_color() {
    let color = Color {
        r: 10,
        g: 20,
        b: 30,
    };

    let serialized_color = serde_json::to_string(&color).unwrap();
    assert_eq!(serialized_color, "\"#0A141E\"");
}

#[cfg(test)]
#[test]
fn test_deserialize_color() {
    let color = "\"#0A141E\"";

    let deserialized_color: Color = serde_json::from_str(color).unwrap();
    assert_eq!(deserialized_color.r, 10);
    assert_eq!(deserialized_color.g, 20);
    assert_eq!(deserialized_color.b, 30);

    assert!(serde_json::from_str::<Color>("\"000000\"").is_err());
    assert!(serde_json::from_str::<Color>("\"#00000\"").is_err());
    assert!(serde_json::from_str::<Color>("\"#0000000\"").is_err());
    assert!(serde_json::from_str::<Color>("\"#G00000\"").is_err());
}
