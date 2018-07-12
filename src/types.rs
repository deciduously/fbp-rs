// see https://wiki.factorio.com/Blueprint_string_format for specification

pub type ItemCountType = u32;
pub type GraphicsVariation = u8;

// Top-level container
// How do I do this?  Json is of form { "blueprint": <#Blueprint> }
#[derive(Debug, Deserialize)]
pub struct Container {
    pub blueprint: Blueprint,
}

// TODO this is a placeholder I put in just to write a unti test
// which ensures the deserialization works - if we succeeded, this fn will exist which is good enough for me
// You should really write out an expected type
impl Container {
    pub fn ok(&self) -> bool {
        true
    }
}

#[derive(Debug, Deserialize)]
pub struct BlueprintBook {
    pub item: String,                      // always "blueprint-book"
    pub label: String,                     // user-defined name
    pub blueprints: Vec<(i32, Blueprint)>, // 0-based index, blueprint from below
    pub active_index: i32,                 // selected blueprint
    pub version: i64,                      // map version of the map the blueprint was created in
}

#[derive(Debug, Deserialize)]
pub struct Blueprint {
    pub item: String,             // always "blueprint"
    pub label: String,            // user-defined name
    pub entities: Vec<Entity>,    // actual content
    pub tiles: Option<Vec<Tile>>, // tiles included
    pub icons: Vec<Icon>,         // icons of the blueprint set by the user
    pub version: i64,             // map version of the map the blueprint was created in
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
pub struct Tile {
    pub name: String,       // prototype name of the tile (e.g. "concrete")
    pub position: Position, // within the blueprint
}

// 0,0 is the center
#[derive(Debug, Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Deserialize)]
pub struct Icon {
    pub index: i32,       // Index of the icon, 1-based
    pub signal: SignalID, // the icon that is displayed
}

#[derive(Debug, Deserialize)]
pub struct SignalID {
    pub name: String, // name of the signal prototype this signal is set to
    #[serde(rename = "type")]
    pub signal_type: String, // either "item", "fluid", or "virtual" - you should make an enum with FromStr/ToStr
}

// the spec has digits 1 and 2 as key names
#[derive(Debug, Deserialize)]
pub struct Connection {
    #[serde(rename = "1")]
    pub one: ConnectionPoint, // Default for everything that doens't have multiple connection points
    #[serde(rename = "2")]
    pub two: Option<ConnectionPoint>, // e.g. the "output" of an arithmetic combinator
}

#[derive(Debug, Deserialize)]
pub struct ConnectionPoint {
    pub red: Vec<ConnectionData>,   // all red wire connections
    pub green: Vec<ConnectionData>, // all green wire connections
}

#[derive(Debug, Deserialize)]
pub struct ConnectionData {
    pub entity_id: i32,
    pub circuit_id: i32,
}

// One or more key-val pairs - key is String of iitem name, value is amt requested
#[derive(Debug, Deserialize)]
pub struct ItemRequest {}

#[derive(Debug, Deserialize)]
pub struct ItemFilter {
    pub name: String, // name of prototype
    pub index: i32,   // index of filter, 1-based
}

#[derive(Debug, Deserialize)]
pub struct InfinitySettings {
    pub remove_unfiltered_items: bool, // this is a checkbox in the UI
    pub filters: Option<Vec<InfinityFilter>>,
}

#[derive(Debug, Deserialize)]
pub struct InfinityFilter {
    pub name: String, // name of the prototype
    pub count: ItemCountType,
    pub mode: String, // either "at-least", "at-most", "exactly" - TODO enum
    pub index: i32,   // 1-based
}

#[derive(Debug, Deserialize)]
pub struct LogisticFilter {
    pub name: String,         // name ofthe prototype
    pub index: i32,           // 1-based
    pub count: ItemCountType, // number filter is set to, 0 for storage chests
}

#[derive(Debug, Deserialize)]
pub struct SpeakerParameter {
    pub playback_volume: f64,    // volume of speaker
    pub playback_globally: bool, // global playback enabled
    pub allow_polyphony: bool,   // speaks for itself
}

#[derive(Debug, Deserialize)]
pub struct SpeakerAlertParameter {
    pub show_alert: bool,
    pub show_on_map: bool,        // is icon shown on map
    pub icon_signal_id: SignalID, //what to display with alert
    pub alert_message: String,    // body of the alert
}

#[derive(Debug, Deserialize)]
pub struct Color {
    pub r: i32,
    pub g: i32,
    pub b: i32,
    pub a: i32,
}
