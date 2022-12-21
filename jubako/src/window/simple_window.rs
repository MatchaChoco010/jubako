//! SimpleWindow trait and related types.
//!
//! [`SimpleWindowCreator<Message = T>`] is a trait that creates [`SimpleWindow<Message = T>`].
//! [`SimpleWindow<Message = T>`] is a trait that is used to create a virtual dom tree of
//! the simple window and handle their messages.
//!
//! # Example

use axum::extract::ws::{Message as WebSocketMessage, WebSocket};
use parking_lot::Mutex;
use std::{
    fmt::Debug,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};

use crate::vnode::{MessageHandledVNode, VNode};
use crate::{DirtyFlag, Message};

struct SimpleWindowContextInner<T: Message> {
    sender: UnboundedSender<T>,
}
#[derive(Clone)]
pub struct SimpleWindowContext<T: Message> {
    inner: Arc<Mutex<SimpleWindowContextInner<T>>>,
}
impl<T: Message> SimpleWindowContext<T> {
    fn new(sender: UnboundedSender<T>) -> Self {
        let inner = SimpleWindowContextInner { sender };
        Self {
            inner: Arc::new(Mutex::new(inner)),
        }
    }

    pub fn dispatch(&self, message: T) {
        let inner = self.inner.lock();
        inner.sender.send(message).unwrap();
    }
}
impl<T: Message> Debug for SimpleWindowContext<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SimpleWindowContext").finish()
    }
}

pub trait SimpleWindowCreator: 'static + Clone + Send + Sync {
    type Message: Message;
    fn create(
        &mut self,
        ctx: SimpleWindowContext<Self::Message>,
    ) -> Arc<dyn SimpleWindow<Message = Self::Message>>;
}

pub trait SimpleWindow: 'static + Send + Sync {
    type Message: Message;
    fn update(&self, message: Self::Message) -> DirtyFlag;
    fn view(&self) -> VNode<Self::Message>;
    fn disconnected(&self) {}
}
impl<T: SimpleWindow + ?Sized> SimpleWindow for Arc<T> {
    type Message = T::Message;
    fn update(&self, message: Self::Message) -> DirtyFlag {
        T::update(self, message)
    }
    fn view(&self) -> crate::vnode::VNode<Self::Message> {
        T::view(self)
    }
    fn disconnected(&self) {
        T::disconnected(self)
    }
}

trait SimpleWindowCallback: 'static + Send + Sync {
    fn view(&self) -> MessageHandledVNode;
    fn disconnected(&self) {}
}
struct SimpleWindowCallbackImpl<T: Message> {
    window: Arc<dyn SimpleWindow<Message = T>>,
    sender: UnboundedSender<T>,
}
impl<T: Message> SimpleWindowCallbackImpl<T> {
    fn new(window: Arc<dyn SimpleWindow<Message = T>>, sender: UnboundedSender<T>) -> Box<Self> {
        Box::new(Self { window, sender })
    }
}
impl<T: Message> SimpleWindowCallback for SimpleWindowCallbackImpl<T> {
    fn view(&self) -> MessageHandledVNode {
        MessageHandledVNode::handle_message(self.window.view(), self.sender.clone())
    }
    fn disconnected(&self) {
        self.window.disconnected()
    }
}

struct SimpleWindowRunnerInner {
    window: Box<dyn SimpleWindowCallback>,
    dirty: Arc<AtomicBool>,
    vnode_processor: crate::vnode::Processor,
}
impl Debug for SimpleWindowRunnerInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SimpleWindowRunnerInner")
            .field("dirty", &self.dirty)
            .field("vnode_processor", &self.vnode_processor)
            .finish()
    }
}

#[derive(Debug)]
pub(crate) struct SimpleWindowRunner {
    inner: Arc<Mutex<SimpleWindowRunnerInner>>,
}
impl SimpleWindowRunner {
    pub(crate) fn new<T: Message>(
        mut window_creator: impl SimpleWindowCreator<Message = T>,
    ) -> Self {
        let (message_sender, mut message_receiver) = unbounded_channel();

        let context = SimpleWindowContext::new(message_sender.clone());
        let window = window_creator.create(context);

        let dirty = Arc::new(AtomicBool::new(true));

        tokio::spawn({
            let window = window.clone();
            let dirty = dirty.clone();
            async move {
                loop {
                    if let Some(message) = message_receiver.recv().await {
                        let should_render = window.update(message);
                        if should_render == DirtyFlag::ShouldRender {
                            dirty.store(true, Ordering::Relaxed);
                        }
                    } else {
                        break;
                    }
                }
            }
        });

        let vnode_processor = crate::vnode::Processor::new();

        Self {
            inner: Arc::new(Mutex::new(SimpleWindowRunnerInner {
                window: SimpleWindowCallbackImpl::new(window, message_sender),
                dirty,
                vnode_processor,
            })),
        }
    }

    fn is_dirty(&self) -> bool {
        self.inner.lock().dirty.load(Ordering::Relaxed)
    }

    fn reset_dirty(&self) {
        self.inner.lock().dirty.store(false, Ordering::Relaxed)
    }

    pub(crate) async fn run(&self, mut websocket: WebSocket) {
        loop {
            match websocket.recv().await {
                Some(msg) => {
                    match msg {
                        Ok(WebSocketMessage::Text(text)) => {
                            if &text == r#""DRAW""# {
                                if let Some(commands) = self.draw() {
                                    websocket
                                        .send(WebSocketMessage::Text(
                                            serde_json::to_string(&commands).unwrap(),
                                        ))
                                        .await
                                        .unwrap();
                                }
                            } else if let Ok(event) =
                                serde_json::from_str::<crate::vnode::VNodeEvent>(&text)
                            {
                                self.handle_event(event);
                            }
                        }
                        Ok(WebSocketMessage::Close(_)) => {
                            // connection is ended
                            break;
                        }
                        _ => (),
                    }
                }
                None => {
                    // connection is ended
                    break;
                }
            }
        }
        self.inner.lock().window.disconnected();
    }

    fn draw(&self) -> Option<crate::vnode::DifferenceCommands> {
        if self.is_dirty() {
            let commands = {
                let mut inner = self.inner.lock();
                let vnode = inner.window.view();
                inner.vnode_processor.next(vnode)
            };
            self.reset_dirty();
            Some(commands)
        } else {
            None
        }
    }

    fn handle_event(&self, event: crate::vnode::VNodeEvent) {
        self.inner.lock().vnode_processor.handle_event(event);
    }
}
