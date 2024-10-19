use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Window {
    pub id: u32,
    pub pid: u32,
    pub app: String,
    pub title: String,
    pub scratchpad: String,
    pub frame: Frame,
    pub role: String,
    pub subrole: String,
    #[serde(rename = "root-window")]
    pub root_window: bool,
    pub display: u32,
    pub space: u32,
    pub level: i32,
    #[serde(rename = "sub-level")]
    pub sub_level: i32,
    pub layer: String,
    #[serde(rename = "sub-layer")]
    pub sub_layer: String,
    pub opacity: f64,
    #[serde(rename = "split-type")]
    pub split_type: String,
    #[serde(rename = "split-child")]
    pub split_child: String,
    #[serde(rename = "stack-index")]
    pub stack_index: i32,
    #[serde(rename = "can-move")]
    pub can_move: bool,
    #[serde(rename = "can-resize")]
    pub can_resize: bool,
    #[serde(rename = "has-focus")]
    pub has_focus: bool,
    #[serde(rename = "has-shadow")]
    pub has_shadow: bool,
    #[serde(rename = "has-parent-zoom")]
    pub has_parent_zoom: bool,
    #[serde(rename = "has-fullscreen-zoom")]
    pub has_fullscreen_zoom: bool,
    #[serde(rename = "has-ax-reference")]
    pub has_ax_reference: bool,
    #[serde(rename = "is-native-fullscreen")]
    pub is_native_fullscreen: bool,
    #[serde(rename = "is-visible")]
    pub is_visible: bool,
    #[serde(rename = "is-minimized")]
    pub is_minimized: bool,
    #[serde(rename = "is-hidden")]
    pub is_hidden: bool,
    #[serde(rename = "is-floating")]
    pub is_floating: bool,
    #[serde(rename = "is-sticky")]
    pub is_sticky: bool,
    #[serde(rename = "is-grabbed")]
    pub is_grabbed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Frame {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SpaceType {
    Bsp,
    Stack,
    Float,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Space {
    pub id: u32,
    pub uuid: String,
    pub index: u32,
    pub label: String,
    #[serde(rename = "type")]
    pub space_type: SpaceType,
    pub display: u32,
    pub windows: Vec<u32>,
    #[serde(rename = "first-window")]
    pub first_window: u32,
    #[serde(rename = "last-window")]
    pub last_window: u32,
    #[serde(rename = "has-focus")]
    pub has_focus: bool,
    #[serde(rename = "is-visible")]
    pub is_visible: bool,
    #[serde(rename = "is-native-fullscreen")]
    pub is_native_fullscreen: bool,
}
