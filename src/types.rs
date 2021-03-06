// see https://wiki.factorio.com/Blueprint_string_format for specification

use std::fmt;

pub static ENTITY_LEN: usize = 8;

pub type ItemCountType = u32;
pub type GraphicsVariation = u8;

// Top-level container
// I want to be able to show this - is a Display impl sufficient?
// How do we get the full size?
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Container {
    pub blueprint: Blueprint, // this should be a union of Blueprint and BlueprintBook
}

// You could have a completelete separate BookContainer, and when parsing, try both?

impl fmt::Display for Container {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.blueprint)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BlueprintBook {
    pub item: String,                      // always "blueprint-book"
    pub label: String,                     // user-defined name
    pub blueprints: Vec<(i32, Blueprint)>, // 0-based index, blueprint from below
    pub active_index: i32,                 // selected blueprint
    pub version: i64,                      // map version of the map the blueprint was created in
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Blueprint {
    pub item: String,             // always "blueprint"
    pub label: Option<String>,    // user-defined name
    pub entities: Vec<Entity>,    // actual content
    pub tiles: Option<Vec<Tile>>, // tiles included
    pub icons: Vec<Icon>,         // icons of the blueprint set by the user
    pub version: i64,             // map version of the map the blueprint was created in
}

impl Blueprint {
    // returns the dimension of a square which can fit all the entities
    // I hestitate to say smallest because I'm not mathy enough and this implementation feels lazy to me
    // I don't really care if its a size too large as long as it works.  Bite me
    pub fn size(&self) -> usize {
        // find the largest coord
        let mut largest_coord: f64 = 0.0;
        for e in &self.entities {
            if e.position.x.abs() > largest_coord {
                largest_coord = e.position.x;
            }

            if e.position.y.abs() > largest_coord {
                largest_coord = e.position.y;
            }
        }
        // get the next largest integer, increment it, and double it
        (largest_coord.ceil() as usize) * 2 + 1
    }
}

impl fmt::Display for Blueprint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut disp_entities = String::new();
        for e in &self.entities {
            disp_entities.push_str(&format!("{}\n", e))
        }
        write!(
            f,
            "{} (size: {} side square):\n{}map v. {}",
            self.label.clone().unwrap_or("Untitled".into()),
            self.size(),
            disp_entities,
            self.version
        )
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Entity {
    pub entity_number: i32, // 1-based index of entity
    pub name: String,       // e.g. "offshore-pump"
    pub position: Position,
    pub direction: Option<u32>,               //uint (optional) per spec
    pub connections: Option<Vec<Connection>>, // circuit connection
    //pub control_behavior:                                 // TODO what is this??
    pub items: Option<ItemRequest>, // defines the item-request-proxy when blueprint is placed, optional
    pub recipe: Option<String>,     // name of the recipe this machine is set to, optional
    pub bar: Option<i32>, // index of first inaccessible item slot due to limiting with the red "bar"
    pub infinity_settings: Option<InfinitySettings>,
    #[serde(rename = "type")]
    pub underground_type: Option<String>, // either "input" or "output" - type of underground belt or loader
    pub input_priority: Option<String>, // input prio of splitter, "right" or "left" - "none" is omitted
    pub output_priority: Option<String>, // output prio of splitter, "right" or "left" - "none" is omitted
    pub filter: Option<String>,          // splitter filter - name of the prototype
    pub filters: Option<Vec<ItemFilter>>, // for filter inserter or loader
    pub override_stack_size: Option<u8>, // stack size of the inserter
    pub drop_position: Option<Position>, // drop position of inserter
    pub pickup_position: Option<Position>, // pickup position of inserter
    pub request_filters: Option<LogisticFilter>,
    pub request_from_buffers: Option<bool>, // whether chest can request from buffers
    pub parameters: Option<SpeakerParameter>, // Programmable speaker
    pub alert_parameters: Option<SpeakerAlertParameter>,
    pub auto_launch: Option<bool>,            // used by rocket silo
    pub variation: Option<GraphicsVariation>, // used by SimplyEntityWithOwner
    pub color: Option<Color>, // SimpleEntityWithForce, SimpleEntityWithOwner, or train station
    pub station: Option<String>, // Name of the train station
}

impl fmt::Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO direction
        write!(f, "{}", self.name)
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Tile {
    pub name: String,       // prototype name of the tile (e.g. "concrete")
    pub position: Position, // within the blueprint
}

// 0,0 is the center
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl Position {
    pub fn new(x: f64, y: f64) -> Self {
        Position { x, y }
    }
    // returns delta_x, delta_y
    pub fn distance(&self, target: &Self) -> (f64, f64) {
        (self.x - target.x, self.y - target.y)
    }
    // bp has coords with 0,0 as the center, our Grid type uses 0,0 as the top left corner
    // translate grid returns indices in the cells vector in Grid
    // TODO this is supes buggy
    pub fn grid_coords(&self, size: usize) -> (usize, usize) {
        let shift = (size / 2) as f64;
        ((self.x + shift) as usize, (self.y + shift) as usize)
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Icon {
    pub index: i32,       // Index of the icon, 1-based
    pub signal: SignalID, // the icon that is displayed
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SignalID {
    pub name: String, // name of the signal prototype this signal is set to
    #[serde(rename = "type")]
    pub signal_type: String, // either "item", "fluid", or "virtual" - you should make an enum with FromStr/ToStr
}

// the spec has digits 1 and 2 as key names
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Connection {
    #[serde(rename = "1")]
    pub one: ConnectionPoint, // Default for everything that doens't have multiple connection points
    #[serde(rename = "2")]
    pub two: Option<ConnectionPoint>, // e.g. the "output" of an arithmetic combinator
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ConnectionPoint {
    pub red: Vec<ConnectionData>,   // all red wire connections
    pub green: Vec<ConnectionData>, // all green wire connections
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ConnectionData {
    pub entity_id: i32,
    pub circuit_id: i32,
}

// One or more key-val pairs - key is String of iitem name, value is amt requested
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ItemRequest {}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ItemFilter {
    pub name: String, // name of prototype
    pub index: i32,   // index of filter, 1-based
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InfinitySettings {
    pub remove_unfiltered_items: bool, // this is a checkbox in the UI
    pub filters: Option<Vec<InfinityFilter>>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InfinityFilter {
    pub name: String, // name of the prototype
    pub count: ItemCountType,
    pub mode: String, // either "at-least", "at-most", "exactly" - TODO enum
    pub index: i32,   // 1-based
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct LogisticFilter {
    pub name: String,         // name ofthe prototype
    pub index: i32,           // 1-based
    pub count: ItemCountType, // number filter is set to, 0 for storage chests
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SpeakerParameter {
    pub playback_volume: f64,    // volume of speaker
    pub playback_globally: bool, // global playback enabled
    pub allow_polyphony: bool,   // speaks for itself
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SpeakerAlertParameter {
    pub show_alert: bool,
    pub show_on_map: bool,        // is icon shown on map
    pub icon_signal_id: SignalID, //what to display with alert
    pub alert_message: String,    // body of the alert
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Color {
    pub r: i32,
    pub g: i32,
    pub b: i32,
    pub a: i32,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_translate_grid_origin() {
        use super::Position;

        let pos = Position::new(0.0, 0.0);
        let size: usize = 7;
        let (target_x, target_y) = (3, 3);
        assert_eq!(pos.grid_coords(size), (target_x, target_y))
    }
    #[test]
    fn test_translate_grid_positive_whole() {
        use super::Position;

        let pos = Position::new(1.0, 2.0);
        let size: usize = 7;
        let (target_x, target_y) = (4, 5);
        assert_eq!(pos.grid_coords(size), (target_x, target_y))
    }
    #[test]
    fn test_translate_grid_negative_whole() {
        use super::Position;

        let pos = Position::new(3.0, -2.0);
        let size: usize = 7;
        let (target_x, target_y) = (6, 1);
        assert_eq!(pos.grid_coords(size), (target_x, target_y))
    }
    #[test]
    fn test_translate_grid_fraction() {
        use super::Position;

        let pos = Position::new(-1.5, 0.5);
        let size: usize = 7;
        let (target_x, target_y) = (1, 3);
        assert_eq!(pos.grid_coords(size), (target_x, target_y))
    }
}
