//! Virtual Dom Processor has 2 main functions:
//! 1. generate next vnode difference commands
//! 2. handle event and send messages

use std::{collections::HashMap, fmt::Debug};
use uuid::Uuid;

use crate::vnode::{
    command::*,
    event::{VNodeEvent, VNodeEventKind},
    vnode::{MessageHandledVNode, PortalExpandedVNodes, RenderedVNodes, StyleExtractedVNode},
    EventHandleId,
};

pub(crate) struct Processor {
    current_vnode: RenderedVNodes,
    styles: HashMap<String, Uuid>,
    events: HashMap<EventHandleId, Box<dyn Fn(VNodeEventKind) + Sync + Send>>,
}
impl Processor {
    pub(crate) fn new() -> Self {
        Self {
            current_vnode: RenderedVNodes::new(),
            styles: HashMap::new(),
            events: HashMap::new(),
        }
    }

    // generate next vnode difference commands
    pub(crate) fn next(&mut self, vnode: MessageHandledVNode) -> DifferenceCommands {
        let (vnode, styles) = StyleExtractedVNode::extract(vnode, &mut self.styles);
        let vnode = PortalExpandedVNodes::expand(vnode);
        let (main, portals) = self.current_vnode.diff(vnode, &mut self.events);
        DifferenceCommands {
            main,
            portals,
            styles,
        }
    }

    pub(crate) fn handle_event(&self, evt: VNodeEvent) {
        if let Some(handler) = self.events.get(&evt.handle_id) {
            handler(evt.kind);
        }
    }
}
impl Debug for Processor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Processor")
            .field("current_vnode", &self.current_vnode)
            .field("styles", &self.styles)
            .finish()
    }
}
