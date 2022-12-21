//! Virtual Dom Node.
//!
//! VNode is processed in the following order in processor:
//! VNode<T> -> MessageHandledVNode -> StyleExtractedVNode -> PortalExpendedVNodes -> RenderedVNodes

use tokio::sync::mpsc::UnboundedSender;
use typeshare::typeshare;
use uuid::Uuid;

use crate::vnode::event::{self, VNodeEventKind};
use crate::{event_list_macro, Message};

mod message_handled_vnode;
pub(crate) use message_handled_vnode::MessageHandledVNode;
mod style_extracted_vnode;
pub(crate) use style_extracted_vnode::StyleExtractedVNode;
mod portal_expanded_vnode;
pub(crate) use portal_expanded_vnode::{PortalExpandedVNode, PortalExpandedVNodes};
mod rendered_vnode;
pub(crate) use rendered_vnode::RenderedVNodes;

#[derive(Default)]
pub enum VNodeEventHandler<Event, T: Message> {
    Handle {
        handler: Box<dyn Fn(Event) -> T + Sync + Send>,
    },
    HandlePreventDefault {
        handler: Box<dyn Fn(Event) -> T + Sync + Send>,
    },
    #[default]
    None,
}
impl<Event, T: Message> VNodeEventHandler<Event, T> {
    pub fn handle<F>(handler: F) -> Self
    where
        F: Fn(Event) -> T + 'static + Sync + Send,
    {
        Self::Handle {
            handler: Box::new(handler),
        }
    }
    pub fn handle_prevent_default<F>(handler: F) -> Self
    where
        F: Fn(Event) -> T + 'static + Sync + Send,
    {
        Self::HandlePreventDefault {
            handler: Box::new(handler),
        }
    }
}

macro_rules! define_vnode_event_handlers {
    ( $( $event_name_snake:ident, $event_name_pascal:ident => $event_type:ident, )* ) => {
        pub struct VNodeEventHandlers<T: Message> {
            $(
                pub $event_name_snake: VNodeEventHandler<event::$event_type, T>,
            )*
        }
    };
}
event_list_macro!(define_vnode_event_handlers);
macro_rules! impl_default_vnode_event_handlers {
    ( $( $event_name_snake:ident, $event_name_pascal:ident => $event_type:ident, )* ) => {
        impl<T: Message> Default for VNodeEventHandlers<T> {
            fn default() -> Self {
                Self {
                    $(
                        $event_name_snake: VNodeEventHandler::None,
                    )*
                }
            }
        }
    };
}
event_list_macro!(impl_default_vnode_event_handlers);

pub enum VNode<T: Message> {
    Text {
        text: String,
    },
    Element {
        tag: String,
        class: Vec<String>,
        props: Vec<String>,
        event: VNodeEventHandlers<T>,
        children: Vec<VNode<T>>,
        style: Option<String>,
    },
    Portal {
        children: Vec<VNode<T>>,
    },
}

// Event handle ID that is used to identify event handlers.
#[typeshare]
#[derive(serde::Serialize, serde::Deserialize, Eq, PartialEq, Clone, Debug, Default, Hash)]
pub(crate) struct EventHandleId(String);
impl EventHandleId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}

macro_rules! define_vnode_event_type {
    ( $( $event_name_snake:ident, $event_name_pascal:ident => $event_type:ident, )* ) => {
        #[typeshare]
        #[derive(serde::Serialize, Eq, PartialEq, Clone, Copy, Debug)]
        pub(crate) enum VNodeEventType {
            $(
                $event_name_pascal,
            )*
        }
    };
}
event_list_macro!(define_vnode_event_type);

pub(crate) struct BundledEventHandler {
    pub(crate) handler: Box<dyn Fn(event::VNodeEventKind) + Sync + Send>,
    pub(crate) handle_events: Vec<VNodeEventType>,
    pub(crate) handle_prevent_default_events: Vec<VNodeEventType>,
}
impl BundledEventHandler {
    pub(crate) fn bundle<T: Message>(
        handlers: VNodeEventHandlers<T>,
        sender: UnboundedSender<T>,
    ) -> BundledEventHandler {
        let mut handle_events = vec![];
        let mut handle_prevent_default_events = vec![];

        macro_rules! handle_event {
            ($handler:ident, $ty:ident) => {
                if let VNodeEventHandler::Handle { .. } = &handlers.$handler {
                    handle_events.push(VNodeEventType::$ty);
                } else if let VNodeEventHandler::HandlePreventDefault { .. } = &handlers.$handler {
                    handle_prevent_default_events.push(VNodeEventType::$ty);
                }
            };
        }
        macro_rules! handle_events {
            ( $( $event_name_snake:ident, $event_name_pascal:ident => $event_type:ident, )* ) => {
                $(
                    handle_event!($event_name_snake, $event_name_pascal);
                )*
            };
        }
        event_list_macro!(handle_events);

        macro_rules! define_handler {
            ( $( $event_name_snake:ident, $event_name_pascal:ident => $event_type:ident, )* ) => {
                Box::new(move |event: VNodeEventKind| match event {
                    $(
                        VNodeEventKind::$event_name_pascal(evt) => {
                            if let VNodeEventHandler::Handle { handler } = &handlers.$event_name_snake {
                                sender.send(handler(evt)).unwrap();
                            } else if let VNodeEventHandler::HandlePreventDefault { handler } = &handlers.$event_name_snake {
                                sender.send(handler(evt)).unwrap();
                            }
                        }
                    )*
                }) as Box<dyn Fn(VNodeEventKind) + Sync + Send>
            };
        }
        let handler = event_list_macro!(define_handler);

        BundledEventHandler {
            handle_events,
            handle_prevent_default_events,
            handler,
        }
    }
}
