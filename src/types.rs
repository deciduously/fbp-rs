// see https://wiki.factorio.com/Blueprint_string_format for specification

pub type ItemCountType = u32;
pub type GraphicsVariation = u8;

#[derive(Debug)]
pub struct BlueprintBook {
    pub item: String, // always "blueprint-book"
    pub label: String, // user-defined name
    pub blueprints: Vec<(i32, Blueprint)>, // 0-based index, blueprint from below
    pub active_index: i32,
    pub version: i64, // map version of the map the blueprint was created in
}

#[derive(Debug)]
pub struct Blueprint {
    pub item: String, // always "blueprint"
    pub label: String, // user-defined name
    pub entities: Vec<Entity>, // actual content
    pub tiles: Vec<Tile>, // tiles included
    pub icons: Vec<Icon>, // icons of the blueprint set by the user
    pub version: i64, // map version of the map the blueprint was created in
}

// NOTE underground_type sepcified as "type" - might be an issue
#[derive(Debug)]
pub struct Entity {
    pub entity_number: i32, // 1-based index of entity
    pub name: String, // e.g. "offshore-pump"
    pub position: Position,
    pub direction: Option<u32>, //uint (optional) per spec
    pub connections: Vec<Connection>, // circuit connection
    //pub control_behavior: // TODO what is this??
    pub items: Option<ItemRequest>, // defines the item-request-proxy when blueprint is placed, optional
    pub recipe: Option<String>, // name of the recipe this machine is set to, optional
    pub bar: Option<i32>, // inex of first inaccessible item slot due to limiting with the red "bar"
    pub infinity_settings: Option<InfinitySettings>,
    pub underground_type: Option<String>, // either "input" or "output" - type of underground belt or loader
    pub input_priority: Option<String>, // input prio of splitter, "right" or "left" - "none" is omitted
    pub output_priority: Option<String>, // output prio of splitter, "right" or "left" - "none" is omitted
    pub filter: Option<String>, // splitter filter - name of the prototype
    pub filters: Option<Vec<ItemFilter>>, // for filter inserter or loader
    pub override_stack_size: Option<u8>, // stack size of the inserter
    pub drop_position: Option<Position>, // drop position of inserter
    pub pickup_position: Option<Position>, // pickup position of inserter
    pub request_filters: Option<LogisticFilter>,
    pub request_from_buffers: Bool, // whether chest can request from buffers
    pub parameters: Option<SpeakerParameter>, // Programmable speaker
    pub alert_parameters: Option<SpeakerAlertParameter>,
    pub auto_launch: Option<Bool>, // used by rocket silo
    pub variation: Option<GrapicsVariation>, // used by SimplyEntityWithOwner
    pub color: Option<Color>, // SimpleEntityWithFOrce, SimpleEntityWithOwner, or train station
    pub station: Option<String>, // Name of the train station
}

#[derive(Debug)]
pub struct Tile {
    pub name: String, // prototype name of the tile (e.g. "concrete")
    pub position: Position, // within the blueprint
}

// 0,0 is the center
#[derive(Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub struct Icon {
    pub index: i32, // Index of the icon, 1-based
    pub signal: SignalID, // the icon that is displayed
}


// This is supposed to be "name": string, "type": string but "type" is reserved by Rust
#[derive(Debug)]
pub struct SignalID {
    pub name: String, // name of the signal prototype this signal is set ot
    pub signal_type: String, // either "item", "fluid", or "virtual" - you should make an enum with FromStr/ToStr
}

#[derive(Debug)]
pub struct Connection {
    pub 1: ConnectionPoint, // Default for everything that doens't have multiple connection points
    pub 2: Option<ConnectionPoint>, // e.g. the "output" of an arithmetic combinator
}

#[derive(Debug)]
pub struct ConnectionPoint {
    pub red: Vec<ConnectionData>, // all red wire connections
    pub green: Vec<ConnectionData>, // all green wire connections
}

#[derive(Debug)]
pub struct ConnectionData {
    pub entity_id: i32,
    pub circuit_id: i32,
}

// One or more key-val pairs - key is String of iitem name, value is amt requested
#[derive(Debug)]
pub struct ItemRequest {}

#[derive(Debug)]
pub struct ItemFilter {
    pub name: String, // name of prototype
    pub index: i32, // index of filter, 1-based
}

#[derive(Debug)]
pub struct InfinitySettings {
    pub remove_unfiltered_items: Bool, // this is a checkbox in the UI
    pub filters: Option<Vec<InfinityFilter>>,
}

#[derive(Debug)]
pub struct InfinityFilter {
    pub name: String, // name of the prototype
    pub count: ItemCountType,
    pub mode: String, // either "at-least", "at-most", "exactly" - TODO enum
    pub index: i32, // 1-based
}

#[derive(Debug)]
pub struct LogisticFilter {
    pub name: String, // name ofthe prototype
    pub index: i32, // 1-based
    pub count: ItemCountType, // number filter is set to, 0 for storage chests
}

#[derive(Debug)]
pub struct SpeakerParameter {
    pub playback_volume: f64, // volume of speaker
    pub playback_globally: Bool, // global playback enabled
    pub allow_polyphony: Bool, // speaks for itself
}

#[derive(Debug)]
pub struct SpeakerAlertParameter {
    pub show_alert: Bool,
    pub show_on_map: Bool, // is icon shown on map
    pub icon_signal_id: SignalID, //what to display with alert
    pub alert_message: String, // body of the alert
}

#[derive(Debug)]
pub struct Color {
    pub r: i32,
    pub g: i32,
    pub b: i32,
    pub a: i32,
}
