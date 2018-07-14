// this is where I define the actual shapes and sizes

use geo::{LineString, Point, Polygon};
use std::str::FromStr;
use types::Entity;

// this maps the keys coming in from the blueprint string to Polygons
//pub static ENTITIES: HashMap<String, Polygon>;

pub enum EntityType {
    TransportBelt,
    Splitter,
    UndergroundBelt,
}

impl FromStr for EntityType {
    type Err = String; // for now

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "transport-belt" => Ok(EntityType::TransportBelt),
            "splitter" => Ok(EntityType::Splitter),
            "underground-belt" => Ok(EntityType::UndergroundBelt),
            _ => Err("I don't know that entity!".into()),
        }
    }
}

#[derive(Debug)]
pub struct EntityShape {
    pub shape: Polygon<f64>,
}

impl EntityShape {
    pub fn from(e: &Entity) -> Result<Self, String> {
        let entity_type = EntityType::from_str(&e.name)?;
        match entity_type {
            EntityType::TransportBelt => {
                let raw_pos = e.position;
                let corner = Point::new(raw_pos.x, raw_pos.y);
                let exterior = LineString(vec![
                    corner,
                    Point::new(raw_pos.x, raw_pos.y + 1.0),
                    Point::new(raw_pos.x + 1.0, raw_pos.y + 1.0),
                    Point::new(raw_pos.x + 1.0, raw_pos.y + 1.0),
                ]);
                Ok(EntityShape {
                    shape: Polygon::new(exterior.clone(), vec![]),
                })
            }
            // TODO this isn't a single unit square
            EntityType::Splitter => {
                let raw_pos = e.position;
                let corner = Point::new(raw_pos.x, raw_pos.y);
                let exterior = LineString(vec![
                    corner,
                    Point::new(raw_pos.x, raw_pos.y + 1.0),
                    Point::new(raw_pos.x + 1.0, raw_pos.y + 1.0),
                    Point::new(raw_pos.x + 1.0, raw_pos.y + 1.0),
                ]);
                Ok(EntityShape {
                    shape: Polygon::new(exterior.clone(), vec![]),
                })
            }
            EntityType::UndergroundBelt => {
                let raw_pos = e.position;
                let corner = Point::new(raw_pos.x, raw_pos.y);
                let exterior = LineString(vec![
                    corner,
                    Point::new(raw_pos.x, raw_pos.y + 1.0),
                    Point::new(raw_pos.x + 1.0, raw_pos.y + 1.0),
                    Point::new(raw_pos.x + 1.0, raw_pos.y + 1.0),
                ]);
                Ok(EntityShape {
                    shape: Polygon::new(exterior.clone(), vec![]),
                })
            }
        }
    }
}
