use extism_pdk::{FromBytes, ToBytes, Json};
use serde::{Deserialize, Serialize};

#[derive(ToBytes, FromBytes, Deserialize, Serialize, PartialEq, Debug, Clone, Copy)]
#[encoding(Json)]
pub struct Color {
    pub r: i64,
    pub g: i64,
    pub b: i64,
    pub a: Option<i64>,
}

impl Color {
    pub fn new(r: i64, g: i64, b: i64, a: Option<i64>) -> Self {
        Color { r, g, b, a }
    }
    pub fn new_monochrome(color: i64, a: Option<i64>) -> Self {
        Color { r: color, g: color, b: color, a }
    }
    pub fn from_existing(color: Self, a: Option<i64>) -> Self {
        Color { r: color.r, g: color.g, b: color.b, a }
    }
}

#[derive(ToBytes, FromBytes, Deserialize, Serialize, PartialEq, Debug, Clone, Copy)]
#[encoding(Json)]
pub struct TextColors {
    pub foreground: Color,
    pub background: Option<Color>,
}

#[derive(ToBytes, Serialize, PartialEq, Debug, Clone)]
#[encoding(Json)]
pub struct File {
    pub key: String,
    pub path: String,
}

#[derive(ToBytes, Serialize, PartialEq, Debug)]
#[encoding(Json)]
pub struct ExtensionInfo {
    pub files: Vec<File>,
}

#[derive(ToBytes, Serialize, PartialEq, Debug, Clone)]
#[encoding(Json)]
pub struct ContextButton {
    pub text: String,
    pub icon: String,
    pub context_icon: Option<String>,
    pub action: Option<String>,
    pub context_menu: Option<String>,
}

#[derive(ToBytes, Serialize, PartialEq, Debug, Clone)]
#[encoding(Json)]
pub struct ContextMenu {
    pub key: String,
    pub buttons: Vec<ContextButton>,
    pub pre_loop: Option<String>,
    pub post_loop: Option<String>,
    pub invert: bool,
}

#[derive(ToBytes, Serialize, PartialEq, Debug, Clone)]
#[encoding(Json)]
pub struct PygameExtraRectEdgeRounding {
    pub edge_rounding: Option<i64>,
    pub edge_rounding_topright: Option<i64>,
    pub edge_rounding_topleft: Option<i64>,
    pub edge_rounding_bottomright: Option<i64>,
    pub edge_rounding_bottomleft: Option<i64>
}

impl PygameExtraRectEdgeRounding {
    pub fn new(edge_rounding: Option<i64>, edge_rounding_topright: Option<i64>, edge_rounding_topleft: Option<i64>, edge_rounding_bottomright: Option<i64>, edge_rounding_bottomleft: Option<i64>) -> Self {
        Self {
            edge_rounding,
            edge_rounding_topright,
            edge_rounding_topleft,
            edge_rounding_bottomright,
            edge_rounding_bottomleft
        }
    }
    pub fn all(edge_rounding: i64) -> Self {
        Self {
            edge_rounding: Some(edge_rounding),
            edge_rounding_topright: None,
            edge_rounding_topleft: None,
            edge_rounding_bottomright: None,
            edge_rounding_bottomleft: None
        }
    }
}

#[derive(ToBytes, FromBytes, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[encoding(Json)]
pub struct Rect {
    pub x: i64,
    pub y: i64,
    pub width: i64,
    pub height: i64
}

impl Rect {
    pub fn new(x: i64, y: i64, width: i64, height: i64) -> Self {
        Self { x, y, width, height }
    }
    pub fn move_to(&mut self, x: i64, y: i64) {
        self.x = x;
        self.y = y;
    }
    pub fn set_center(&mut self, x: i64, y: i64) {
        self.x = x - self.width / 2;
        self.y = y - self.height / 2;
    }
    pub fn set_size(&mut self, width: i64, height: i64) {
        self.width = width;
        self.height = height;
    }
    pub fn set_topleft(&mut self, x: i64, y: i64) {
        self.x = x;
        self.y = y;
    }

    pub fn set_topright(&mut self, x: i64, y: i64) {
        self.x = x - self.width;
        self.y = y;
    }

    pub fn set_bottomleft(&mut self, x: i64, y: i64) {
        self.x = x;
        self.y = y - self.height;
    }

    pub fn set_bottomright(&mut self, x: i64, y: i64) {
        self.x = x - self.width;
        self.y = y - self.height;
    }
}

#[derive(FromBytes, Deserialize, ToBytes, Serialize, PartialEq, Debug)]
#[encoding(Json)]
pub struct PygameExtraRect {
    pub color: Color,
    pub rect: Rect,
    pub width: i64,
    pub edge_rounding: Option<PygameExtraRectEdgeRounding>
}


#[derive(FromBytes, Deserialize, PartialEq, Debug, Clone)]
#[encoding(Json)]
pub struct MossState {
    pub width: i32,
    pub height: i32,
    pub current_screen: String,
    pub opened_context_menus: Vec<String>,
    pub icons: Vec<String>,
}

#[derive(FromBytes, Deserialize, PartialEq, Debug)]
#[encoding(Json)]
pub struct ConfigGet<T> {
    pub value: T,
}

#[derive(ToBytes, Serialize, PartialEq, Debug)]
#[encoding(Json)]
pub struct ConfigSet<T> {
    pub key: String,
    pub value: T,
}

#[derive(ToBytes, Serialize, PartialEq, Debug)]
#[encoding(Json)]
pub struct MossScreen {
    pub key: String,
    pub screen_pre_loop: Option<String>,
    pub screen_loop: String,
    pub screen_post_loop: Option<String>,
    pub event_hook: Option<String>
}

impl MossScreen {
    pub fn basic(key: String, screen_loop: String) -> Self {
        Self {
            key,
            screen_pre_loop: None,
            screen_loop,
            screen_post_loop: None,
            event_hook: None
        }
    }
    pub fn basic_with_event_hook(key: String, screen_loop: String, event_hook: String) -> Self {
        Self {
            key,
            screen_pre_loop: None,
            screen_loop,
            screen_post_loop: None,
            event_hook: Some(event_hook)
        }
    }
}