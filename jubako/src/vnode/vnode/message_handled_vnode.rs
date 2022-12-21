use tokio::sync::mpsc::UnboundedSender;

use crate::vnode::{BundledEventHandler, VNode};
use crate::Message;

// MessageHandledVNode is a VNode that message is sent to given sender when event is fired
pub(crate) enum MessageHandledVNode {
    Text {
        text: String,
    },
    Element {
        tag: String,
        class: Vec<String>,
        props: Vec<String>,
        event: BundledEventHandler,
        children: Vec<MessageHandledVNode>,
        style: Option<String>,
    },
    Portal {
        children: Vec<MessageHandledVNode>,
    },
}
impl MessageHandledVNode {
    // send message to given sender
    pub(crate) fn handle_message<T: Message>(vnode: VNode<T>, sender: UnboundedSender<T>) -> Self {
        match vnode {
            VNode::Text { text } => Self::Text { text },
            VNode::Element {
                tag,
                class,
                props,
                event,
                children,
                style,
            } => Self::Element {
                tag,
                class,
                props,
                event: BundledEventHandler::bundle(event, sender.clone()),
                children: children
                    .into_iter()
                    .map(|v| MessageHandledVNode::handle_message(v, sender.clone()))
                    .collect(),
                style,
            },
            VNode::Portal { children } => Self::Portal {
                children: children
                    .into_iter()
                    .map(|v| MessageHandledVNode::handle_message(v, sender.clone()))
                    .collect(),
            },
        }
    }
}
