use crate::moss_definitions::types::*;
use extism_pdk::*;
use extism_pdk::json::to_vec;
use serde::{Deserialize, Serialize};

#[host_fn]
extern "ExtismHost" {
    // GUI
    pub fn moss_gui_register_context_menu(menu: ContextMenu);
    pub fn moss_gui_invert_icon(key: String, result_key: String);

    // Defaults
    pub fn moss_defaults_set_color(key: String, color: Color);
    pub fn moss_defaults_get_color(key: String) -> Color;
    pub fn moss_defaults_set_text_color(key: String, colors: TextColors);
    pub fn moss_defaults_get_text_color(key: String) -> TextColors;

    pub fn moss_defaults_get<T: for<'de> Deserialize<'de>>(key: &str) -> ConfigGet<T>;
    #[link_name = "moss_defaults_set"]
    fn _moss_defaults_set<T: Serialize>(value: ConfigSet<T>);

    // Extension manager
    pub fn moss_em_config_get<T: for<'de> Deserialize<'de>>(key: &str) -> ConfigGet<T>;
    #[link_name = "moss_em_config_set"]
    fn _moss_em_config_set<T: Serialize>(value: ConfigSet<T>);
    pub fn moss_em_get_state() -> MossState;

    // PygameExtra
    #[link_name = "moss_pe_draw_rect"]
    fn _moss_pe_draw_rect(draw: PygameExtraRect);

    // Screens
    pub fn moss_pe_register_screen(screen: MossScreen);
    #[link_name = "moss_pe_open_screen"]
    fn _moss_pe_open_screen(key: String, initial_values: Vec<u8>);

    pub fn moss_pe_get_screen_value<T: for<'de> Deserialize<'de>>(key: &str) -> ConfigGet<T>;
    #[link_name = "moss_pe_set_screen_value"]
    fn _moss_pe_set_screen_value<T: Serialize>(value: ConfigSet<T>);
}

pub unsafe fn moss_em_config_set<T: Serialize>(key: &str, value: T) {
    let _ = _moss_em_config_set::<T>(ConfigSet::<T> {
        key: key.into(),
        value,
    });
}
pub unsafe fn moss_pe_set_screen_value<T: Serialize>(key: &str, value: T) {
    let _ = _moss_pe_set_screen_value::<T>(ConfigSet::<T> {
        key: key.into(),
        value,
    });
}

pub unsafe fn moss_pe_open_screen<T: Serialize>(key: &str, initial_values: T) -> Result<(), extism_pdk::Error> {
    let serialized_values = to_vec(&initial_values)?;
    _moss_pe_open_screen(key.into(), serialized_values)
}

pub unsafe fn moss_defaults_set<T: Serialize>(key: &str, value: T) {
    let _ = _moss_defaults_set::<T>(ConfigSet::<T> {
        key: key.into(),
        value,
    });
}

pub unsafe fn moss_pe_draw_rect(color: Color, rect: Rect, width: i64, edge_rounding: Option<PygameExtraRectEdgeRounding>) {
    let _ = _moss_pe_draw_rect(PygameExtraRect {
        color,
        rect,
        width,
        edge_rounding,
    });
}

#[link(wasm_import_module = "extism:host/user")]
extern "C" {
    #[link_name = "moss_gui_open_context_menu"]
    fn moss_gui_open_context_menu_impl(key: u64, x: i64, y: i64) -> ();
}

pub unsafe fn moss_gui_open_context_menu(
    key: &str,
    x: i64,
    y: i64,
) -> Result<(), extism_pdk::Error> {
    let res =
        moss_gui_open_context_menu_impl(extism_pdk::ToMemory::to_memory(&&key)?.offset(), x, y);
    Ok(res)
}
