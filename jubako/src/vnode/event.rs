//! Events sent from the web browser to the jubako server.

use typeshare::typeshare;

use crate::{event_list_macro, vnode::EventHandleId};

#[typeshare]
#[derive(serde::Deserialize, PartialEq, Clone, Debug, Default)]
pub struct Element {
    pub tag_name: String,
    pub client_height: u32,
    pub client_width: u32,
    pub client_left: u32,
    pub client_top: u32,
    pub scroll_height: u32,
    pub scroll_width: u32,
    pub scroll_left: u32,
    pub scroll_top: u32,
}

#[typeshare]
#[derive(serde::Deserialize, PartialEq, Clone, Debug, Default)]
pub struct Event {
    pub target: Option<Element>,
}

#[typeshare]
#[derive(serde::Deserialize, PartialEq, Clone, Debug, Default)]
pub struct MouseEvent {
    pub alt_key: bool,
    pub button: u8,
    pub buttons: u16,
    pub client_x: i32,
    pub client_y: i32,
    pub ctrl_key: bool,
    pub meta_key: bool,
    pub movement_x: i32,
    pub movement_y: i32,
    pub offset_x: i32,
    pub offset_y: i32,
    pub page_x: i32,
    pub page_y: i32,
    pub related_target: Option<Element>,
    pub screen_x: i32,
    pub screen_y: i32,
    pub shift_key: bool,
    pub target: Option<Element>,
    pub x: i32,
    pub y: i32,
}

#[typeshare]
#[derive(serde::Deserialize, PartialEq, Clone, Debug, Default)]
pub struct FocusEvent {
    pub related_target: Option<Element>,
    pub target: Option<Element>,
}

#[typeshare]
#[derive(serde::Deserialize, PartialEq, Clone, Debug, Default)]
pub struct DragEvent {
    pub alt_key: bool,
    pub button: u8,
    pub buttons: u16,
    pub client_x: i32,
    pub client_y: i32,
    pub ctrl_key: bool,
    pub meta_key: bool,
    pub movement_x: i32,
    pub movement_y: i32,
    pub offset_x: i32,
    pub offset_y: i32,
    pub page_x: i32,
    pub page_y: i32,
    pub related_target: Option<Element>,
    pub screen_x: i32,
    pub screen_y: i32,
    pub shift_key: bool,
    pub target: Option<Element>,
    pub x: i32,
    pub y: i32,
}

#[typeshare]
#[derive(serde::Deserialize, PartialEq, Clone, Debug, Default)]
pub struct InputEvent {
    pub data: String,
    pub input_type: String,
    pub is_composing: bool,
    pub target: Option<Element>,
}

#[typeshare]
#[derive(serde::Deserialize, PartialEq, Clone, Debug, Default)]
pub struct KeyboardEvent {
    pub alt_key: bool,
    pub char_code: u32,
    pub code: String,
    pub ctrl_key: bool,
    pub key: String,
    pub key_code: u32,
    pub location: u32,
    pub meta_key: bool,
    pub repeat: bool,
    pub shift_key: bool,
    pub target: Option<Element>,
}

#[typeshare]
#[derive(serde::Deserialize, PartialEq, Clone, Debug, Default)]
pub struct ProgressEvent {
    pub length_computable: bool,
    pub loaded: u32,
    pub target: Option<Element>,
    pub total: u32,
}

#[typeshare]
#[derive(serde::Deserialize, PartialEq, Clone, Debug, Default)]
pub struct SubmitEvent {
    pub submitter: Option<Element>,
    pub target: Option<Element>,
}

#[typeshare]
#[derive(serde::Deserialize, PartialEq, Clone, Debug, Default)]
pub struct WheelEvent {
    pub alt_key: bool,
    pub button: u8,
    pub buttons: u16,
    pub client_x: i32,
    pub client_y: i32,
    pub ctrl_key: bool,
    pub delta_mode: u32,
    pub delta_x: f64,
    pub delta_y: f64,
    pub delta_z: f64,
    pub meta_key: bool,
    pub movement_x: i32,
    pub movement_y: i32,
    pub offset_x: i32,
    pub offset_y: i32,
    pub page_x: i32,
    pub page_y: i32,
    pub screen_x: i32,
    pub screen_y: i32,
    pub shift_key: bool,
    pub x: i32,
    pub y: i32,
    pub target: Option<Element>,
}

#[typeshare]
#[derive(serde::Deserialize, PartialEq, Clone, Debug, Default)]
pub struct AnimationEvent {
    pub animation_name: String,
    pub elapsed_time: f64,
    pub pseudo_element: String,
    pub target: Option<Element>,
}

#[typeshare]
#[derive(serde::Deserialize, PartialEq, Clone, Debug, Default)]
pub struct PointerEvent {
    pub alt_key: bool,
    pub button: u8,
    pub buttons: u16,
    pub client_x: i32,
    pub client_y: i32,
    pub ctrl_key: bool,
    pub height: f64,
    pub is_primary: bool,
    pub meta_key: bool,
    pub movement_x: i32,
    pub movement_y: i32,
    pub offset_x: i32,
    pub offset_y: i32,
    pub page_x: i32,
    pub page_y: i32,
    pub pointer_id: u32,
    pub pointer_type: String,
    pub pressure: f64,
    pub related_target: Option<Element>,
    pub screen_x: i32,
    pub screen_y: i32,
    pub shift_key: bool,
    pub target: Option<Element>,
    pub tilt_x: i16,
    pub tilt_y: i16,
    pub twist: i32,
    pub width: f64,
    pub x: i32,
    pub y: i32,
}

#[typeshare]
#[derive(serde::Deserialize, PartialEq, Clone, Debug, Default)]
pub struct Touch {
    pub identifier: u32,
    pub client_x: i32,
    pub client_y: i32,
    pub page_x: i32,
    pub page_y: i32,
    pub radius_x: f64,
    pub radius_y: f64,
    pub rotation_angle: f64,
    pub screen_x: i32,
    pub screen_y: i32,
    pub target: Option<Element>,
}

#[typeshare]
#[derive(serde::Deserialize, PartialEq, Clone, Debug, Default)]
pub struct TouchEvent {
    pub alt_key: bool,
    pub changed_touches: Vec<Touch>,
    pub ctrl_key: bool,
    pub meta_key: bool,
    pub shift_key: bool,
    pub target_touches: Vec<Touch>,
    pub touches: Vec<Touch>,
    pub target: Option<Element>,
}

#[typeshare]
#[derive(serde::Deserialize, PartialEq, Clone, Debug, Default)]
pub struct TransitionEvent {
    pub elapsed_time: f64,
    pub pseudo_element: String,
    pub property_name: String,
    pub target: Option<Element>,
}

macro_rules! define_vnode_event_kind {
    ( $( $event_name_snake:ident, $event_name_pascal:ident => $event_type:ident, )* ) => {
        #[typeshare::typeshare]
        #[derive(serde::Deserialize, PartialEq, Clone, Debug)]
        #[serde(tag = "type", content = "content")]
        pub(crate) enum VNodeEventKind {
            $(
                $event_name_pascal($event_type),
            )*
        }
    };
}
event_list_macro!(define_vnode_event_kind);

#[typeshare]
#[derive(serde::Deserialize, PartialEq, Clone, Debug)]
pub(crate) struct VNodeEvent {
    pub(crate) handle_id: EventHandleId,
    pub(crate) kind: VNodeEventKind,
}
