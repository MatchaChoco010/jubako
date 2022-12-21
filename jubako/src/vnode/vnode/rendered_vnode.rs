use std::collections::HashMap;

use crate::vnode::{
    command::*,
    event::VNodeEventKind,
    vnode::{PortalExpandedVNode, PortalExpandedVNodes},
    EventHandleId, VNodeEventType,
};

#[derive(Debug)]
pub(crate) struct HandledEvents {
    id: EventHandleId,
    handle_events: Vec<VNodeEventType>,
    handle_prevent_default_events: Vec<VNodeEventType>,
}

#[derive(Debug)]
pub(crate) enum RenderedVNode {
    Text {
        text: String,
    },
    Element {
        tag: String,
        class: Vec<String>,
        props: Vec<String>,
        event: HandledEvents,
        children: Vec<RenderedVNode>,
    },
}
impl RenderedVNode {
    fn is_different(&self, other: &PortalExpandedVNode) -> bool {
        match (self, other) {
            (RenderedVNode::Text { text }, PortalExpandedVNode::Text { text: other_text }) => {
                text != other_text
            }
            (
                RenderedVNode::Element {
                    tag,
                    class,
                    props,
                    event,
                    children,
                },
                PortalExpandedVNode::Element {
                    tag: other_tag,
                    class: other_class,
                    props: other_props,
                    event: other_event,
                    children: other_children,
                },
            ) => {
                tag != other_tag
                    || class != other_class
                    || props != other_props
                    || event.handle_events != other_event.handle_events
                    || event.handle_prevent_default_events
                        != other_event.handle_prevent_default_events
                    || children.len() != other_children.len()
                    || children
                        .iter()
                        .zip(other_children.iter())
                        .any(|(a, b)| a.is_different(b))
            }
            _ => true,
        }
    }
}

// rendered vnode

#[derive(Debug)]
pub(crate) struct RenderedVNodes {
    main: Vec<RenderedVNode>,
    portals: Vec<RenderedVNode>,
}
impl RenderedVNodes {
    pub(crate) fn new() -> Self {
        Self {
            main: vec![],
            portals: vec![],
        }
    }

    // update current RenderedVNode and generate diff commands with update event handlers
    pub(crate) fn diff(
        &mut self,
        next: PortalExpandedVNodes,
        events: &mut HashMap<EventHandleId, Box<dyn Fn(VNodeEventKind) + Sync + Send>>,
    ) -> (Vec<DifferenceCommand>, Vec<DifferenceCommand>) {
        fn diff_vnodes(
            current: &mut Vec<RenderedVNode>,
            mut next: Vec<PortalExpandedVNode>,
            events: &mut HashMap<EventHandleId, Box<dyn Fn(VNodeEventKind) + Sync + Send>>,
        ) -> Vec<DifferenceCommand> {
            let mut commands = vec![];

            let mut i = 0;
            let mut remove_index = None;
            while i < current.len().max(next.len()) as u32 {
                if i >= current.len() as u32 && i < next.len() as u32 {
                    // new element

                    // dummy vnode
                    let mut next_vnode = PortalExpandedVNode::Text {
                        text: String::new(),
                    };
                    // take ownership of next[i]
                    std::mem::swap(&mut next_vnode, &mut next[i as usize]);

                    match next_vnode {
                        PortalExpandedVNode::Element {
                            tag,
                            class,
                            props,
                            event,
                            children,
                        } => {
                            // insert new event handler
                            let event_id = EventHandleId::new();
                            let handler = event.handler;
                            events.insert(event_id.clone(), handler);

                            let mut inserted_vnode_children = vec![];

                            // add insert command
                            commands.push(DifferenceCommand::InsertElement {
                                index: i,
                                tag: tag.clone(),
                                classes: class.clone(),
                                props: props.clone(),
                                event: HandleEvent {
                                    handle_id: event_id.clone(),
                                    handle_events: event.handle_events.clone(),
                                    handle_prevent_default_events: event
                                        .handle_prevent_default_events
                                        .clone(),
                                },
                                children: diff_vnodes(
                                    &mut inserted_vnode_children,
                                    children,
                                    events,
                                ),
                            });

                            // update current vnode
                            current.push(RenderedVNode::Element {
                                tag: tag.clone(),
                                class: class.clone(),
                                props: props.clone(),
                                event: HandledEvents {
                                    id: event_id,
                                    handle_events: event.handle_events.clone(),
                                    handle_prevent_default_events: event
                                        .handle_prevent_default_events
                                        .clone(),
                                },
                                children: inserted_vnode_children,
                            });
                        }
                        PortalExpandedVNode::Text { text } => {
                            // add insert command
                            commands.push(DifferenceCommand::InsertText {
                                index: i,
                                text: text.clone(),
                            });

                            // update current vnode
                            current.push(RenderedVNode::Text { text: text.clone() });
                        }
                    }
                    i += 1;
                    continue;
                } else if i < current.len() as u32 && i >= next.len() as u32 {
                    // removed element
                    if remove_index.is_none() {
                        // when removing multiple elements, the index is shifted,
                        // so record the index of the first element to be removed
                        remove_index = Some(i);
                    }

                    // add remove command
                    commands.push(DifferenceCommand::Remove {
                        index: remove_index.unwrap(),
                    });

                    // update current vnode
                    current.remove(remove_index.unwrap() as usize);

                    i += 1;
                    continue;
                } else if i >= current.len() as u32 && i >= next.len() as u32 {
                    // end of loop
                    break;
                }

                let current_vnode = &mut current[i as usize];
                // dummy vnode
                let mut next_vnode = PortalExpandedVNode::Text {
                    text: String::new(),
                };
                // take ownership of next[i]
                std::mem::swap(&mut next_vnode, &mut next[i as usize]);

                if !current_vnode.is_different(&next_vnode) {
                    // no difference in vnode
                    match (current_vnode, next_vnode) {
                        (
                            RenderedVNode::Element {
                                event: current_event,
                                ..
                            },
                            PortalExpandedVNode::Element {
                                event: updated_event,
                                ..
                            },
                        ) => {
                            // update event handler
                            let event_id = current_event.id.clone();
                            let handler = updated_event.handler;
                            events.insert(event_id, handler);
                        }
                        _ => (),
                    }
                    i += 1;
                    continue;
                }

                match (current_vnode, next_vnode) {
                    (
                        RenderedVNode::Element {
                            tag: current_tag,
                            class: current_class,
                            props: current_props,
                            event: current_event,
                            children: current_children,
                        },
                        PortalExpandedVNode::Element {
                            tag: next_tag,
                            class: next_class,
                            props: next_props,
                            event: next_event,
                            children: next_children,
                        },
                    ) => {
                        if current_tag != &next_tag {
                            // change element tag

                            // remove prev event handler
                            events.remove(&current_event.id);

                            // insert new event handler
                            let event_id = EventHandleId::new();
                            let handler = next_event.handler;
                            events.insert(event_id.clone(), handler);

                            current_children.clear();

                            // add replace command
                            commands.push(DifferenceCommand::ReplaceToElement {
                                index: i,
                                new_tag: next_tag.clone(),
                                classes: next_class.clone(),
                                props: next_props.clone(),
                                event: HandleEvent {
                                    handle_id: event_id.clone(),
                                    handle_events: next_event.handle_events.clone(),
                                    handle_prevent_default_events: next_event
                                        .handle_prevent_default_events
                                        .clone(),
                                },
                                children: diff_vnodes(current_children, next_children, events),
                            });

                            // update current vnode
                            *current_tag = next_tag.clone();
                            *current_class = next_class.clone();
                            *current_props = next_props.clone();
                            *current_event = HandledEvents {
                                id: event_id,
                                handle_events: next_event.handle_events.clone(),
                                handle_prevent_default_events: next_event
                                    .handle_prevent_default_events
                                    .clone(),
                            };
                        } else {
                            // difference of class
                            let mut class_diff = vec![];
                            if current_class != &next_class {
                                // change class
                                for class in next_class.iter() {
                                    if !current_class.contains(class) {
                                        // add add class command
                                        class_diff.push(DifferenceClass::Add(class.clone()));
                                        // update current vnode
                                        current_class.push(class.clone());
                                    }
                                }
                                for class in current_class.clone().iter() {
                                    if !next_class.contains(class) {
                                        // add remove class command
                                        class_diff.push(DifferenceClass::Remove(class.clone()));
                                        // update current vnode
                                        if let Some(index) =
                                            current_class.iter().position(|x| x == class)
                                        {
                                            current_class.remove(index);
                                        }
                                    }
                                }
                            }

                            // difference of props
                            let mut props_diff = vec![];
                            if current_props != &next_props {
                                // change props
                                for props in next_props.iter() {
                                    if !current_props.contains(props) {
                                        // add add props command
                                        props_diff.push(DifferenceProps::Add(props.clone()));
                                        // update current vnode
                                        current_props.push(props.clone());
                                    }
                                }
                                for props in current_props.clone().iter() {
                                    if !next_props.contains(props) {
                                        // add remove props command
                                        props_diff.push(DifferenceProps::Remove(props.clone()));
                                        // update current vnode
                                        if let Some(index) =
                                            current_props.iter().position(|x| x == props)
                                        {
                                            current_props.remove(index);
                                        }
                                    }
                                }
                            }

                            // difference of events
                            let mut event_diff = DifferenceEvent::None;
                            if current_event.handle_events != next_event.handle_events
                                && current_event.handle_prevent_default_events
                                    != next_event.handle_prevent_default_events
                            {
                                // change handled event types
                                event_diff = DifferenceEvent::Update(HandleEvent {
                                    handle_id: current_event.id.clone(),
                                    handle_events: next_event.handle_events.clone(),
                                    handle_prevent_default_events: next_event
                                        .handle_prevent_default_events
                                        .clone(),
                                });
                                // update current vnode
                                current_event.handle_events = next_event.handle_events.clone();
                                current_event.handle_prevent_default_events =
                                    next_event.handle_prevent_default_events.clone();
                            }

                            // add update element command
                            commands.push(DifferenceCommand::UpdateElement {
                                index: i,
                                class_diff,
                                props_diff,
                                event_diff,
                                children: diff_vnodes(current_children, next_children, events),
                            });
                        }
                    }
                    (
                        RenderedVNode::Text { text: current_text },
                        PortalExpandedVNode::Text { text: next_text },
                    ) => {
                        // add update text command
                        commands.push(DifferenceCommand::UpdateText {
                            index: i,
                            new_text: next_text.clone(),
                        });
                        // update current vnode
                        *current_text = next_text.clone();
                    }
                    (
                        RenderedVNode::Element { event, .. },
                        PortalExpandedVNode::Text { text: next_text },
                    ) => {
                        // remove event handler
                        events.remove(&event.id);

                        // add replace command
                        commands.push(DifferenceCommand::ReplaceToText {
                            index: i,
                            text: next_text.clone(),
                        });

                        // update current vnode
                        current[i as usize] = RenderedVNode::Text {
                            text: next_text.clone(),
                        };
                    }
                    (
                        RenderedVNode::Text { .. },
                        PortalExpandedVNode::Element {
                            tag: next_tag,
                            class: next_class,
                            props: next_props,
                            event: next_event,
                            children: next_children,
                        },
                    ) => {
                        // insert new event handler
                        let event_id = EventHandleId::new();
                        let handler = next_event.handler;
                        events.insert(event_id.clone(), handler);

                        let mut new_children = vec![];

                        // add replace command
                        commands.push(DifferenceCommand::ReplaceToElement {
                            index: i,
                            new_tag: next_tag.clone(),
                            classes: next_class.clone(),
                            props: next_props.clone(),
                            event: HandleEvent {
                                handle_id: event_id.clone(),
                                handle_events: next_event.handle_events.clone(),
                                handle_prevent_default_events: next_event
                                    .handle_prevent_default_events
                                    .clone(),
                            },
                            children: diff_vnodes(&mut new_children, next_children, events),
                        });

                        // update current vnode
                        current[i as usize] = RenderedVNode::Element {
                            tag: next_tag.clone(),
                            class: next_class.clone(),
                            props: next_props.clone(),
                            event: HandledEvents {
                                id: event_id,
                                handle_events: next_event.handle_events.clone(),
                                handle_prevent_default_events: next_event
                                    .handle_prevent_default_events
                                    .clone(),
                            },
                            children: new_children,
                        };
                    }
                }
                i += 1;
            }

            commands
        }

        let main_diff = diff_vnodes(&mut self.main, next.main, events);
        let portals_diff = diff_vnodes(&mut self.portals, next.portals, events);

        (main_diff, portals_diff)
    }
}
