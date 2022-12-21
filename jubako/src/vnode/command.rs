//! Commands sent to the web browser to update the DOM.

use typeshare::typeshare;

use crate::vnode::{EventHandleId, VNodeEventType};

#[typeshare]
#[derive(serde::Serialize, Clone, Debug)]
#[serde(tag = "type", content = "content")]
pub(crate) enum DifferenceProps {
    Add(String),
    Remove(String),
}

#[typeshare]
#[derive(serde::Serialize, Clone, Debug)]
#[serde(tag = "type", content = "content")]
pub(crate) enum DifferenceClass {
    Add(String),
    Remove(String),
}

#[typeshare]
#[derive(serde::Serialize, Clone, Debug)]
#[serde(tag = "type", content = "content")]
pub(crate) enum DifferenceEvent {
    None,
    Update(HandleEvent),
}

#[typeshare]
#[derive(serde::Serialize, Clone, Debug)]
pub(crate) struct HandleEvent {
    pub(crate) handle_id: EventHandleId,
    pub(crate) handle_events: Vec<VNodeEventType>,
    pub(crate) handle_prevent_default_events: Vec<VNodeEventType>,
}

#[typeshare]
#[derive(serde::Serialize, Clone, Debug)]
#[serde(tag = "type", content = "content")]
pub(crate) enum DifferenceCommand {
    // update props、class、children
    UpdateElement {
        index: u32,
        class_diff: Vec<DifferenceClass>,
        props_diff: Vec<DifferenceProps>,
        event_diff: DifferenceEvent,
        children: Vec<DifferenceCommand>,
    },
    // update TextNode inner
    UpdateText {
        index: u32,
        new_text: String,
    },
    // update tag type
    ReplaceToElement {
        index: u32,
        new_tag: String,
        classes: Vec<String>,
        props: Vec<String>,
        event: HandleEvent,
        children: Vec<DifferenceCommand>,
    },
    // update tag type to TextNode
    ReplaceToText {
        index: u32,
        text: String,
    },
    // Insert element
    InsertElement {
        index: u32,
        tag: String,
        classes: Vec<String>,
        props: Vec<String>,
        event: HandleEvent,
        children: Vec<DifferenceCommand>,
    },
    // Insert TextNode
    InsertText {
        index: u32,
        text: String,
    },
    // Remove element
    Remove {
        index: u32,
    },
}

#[typeshare]
#[derive(serde::Serialize, Clone, Debug)]
#[serde(tag = "type", content = "content")]
pub(crate) enum StyleDifferenceCommand {
    AddStyle { class_name: String, value: String },
    RemoveStyle { class_name: String },
}

#[typeshare]
#[derive(serde::Serialize, Clone, Debug)]
pub(crate) struct DifferenceCommands {
    pub(crate) main: Vec<DifferenceCommand>,
    pub(crate) portals: Vec<DifferenceCommand>,
    pub(crate) styles: Vec<StyleDifferenceCommand>,
}
