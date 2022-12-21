use jubako::{
    simple_window::{SimpleWindow, SimpleWindowContext, SimpleWindowCreator},
    vnode::{VNode, VNodeEventHandler, VNodeEventHandlers},
    DirtyFlag,
};
use std::sync::{atomic::AtomicI32, Arc};

#[derive(Debug, Clone)]
pub enum TestMessage {
    Increment,
    Decrement,
}

#[derive(Debug, Clone)]
pub struct TestWindowCreator;
impl SimpleWindowCreator for TestWindowCreator {
    type Message = TestMessage;
    fn create(
        &mut self,
        ctx: SimpleWindowContext<Self::Message>,
    ) -> Arc<dyn SimpleWindow<Message = Self::Message>> {
        Arc::new(TestWindow::new(ctx))
    }
}

#[derive(Debug, Clone)]
pub struct TestWindow {
    _ctx: SimpleWindowContext<TestMessage>,
    count: Arc<AtomicI32>,
}
impl TestWindow {
    pub fn new(ctx: SimpleWindowContext<TestMessage>) -> Self {
        Self {
            _ctx: ctx,
            count: Arc::new(AtomicI32::new(0)),
        }
    }
}
impl SimpleWindow for TestWindow {
    type Message = TestMessage;

    fn update(&self, message: Self::Message) -> DirtyFlag {
        match message {
            TestMessage::Increment => {
                self.count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                DirtyFlag::ShouldRender
            }
            TestMessage::Decrement => {
                self.count.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
                DirtyFlag::ShouldRender
            }
        }
    }

    fn view(&self) -> VNode<Self::Message> {
        let count = self.count.load(std::sync::atomic::Ordering::SeqCst);
        let count = count.to_string();

        let count = VNode::Text { text: count };

        let increment = VNode::Element {
            tag: "button".into(),
            class: vec![],
            props: vec![],
            event: VNodeEventHandlers {
                click: VNodeEventHandler::handle(|_| TestMessage::Increment),
                ..Default::default()
            },
            children: vec![VNode::Text {
                text: "increment".into(),
            }],
            style: Some(
                "
                min-width: 120px;
                height: 32px;
                "
                .into(),
            ),
        };
        let decrement = VNode::Element {
            tag: "button".into(),
            class: vec![],
            props: vec![],
            event: VNodeEventHandlers {
                click: VNodeEventHandler::handle(|_| TestMessage::Decrement),
                ..Default::default()
            },
            children: vec![VNode::Text {
                text: "decrement".into(),
            }],
            style: Some(
                "
                min-width: 120px;
                height: 32px;
                "
                .into(),
            ),
        };
        let buttons = VNode::Element {
            tag: "div".into(),
            class: vec![],
            props: vec![],
            event: Default::default(),
            children: vec![increment, decrement],
            style: Some(
                "
                width: 80%;
                display: flex;
                justify-content: space-around;
                "
                .into(),
            ),
        };

        VNode::Element {
            tag: "div".into(),
            class: vec![],
            props: vec![],
            event: Default::default(),
            children: vec![VNode::Element {
                tag: "div".into(),
                class: vec![],
                props: vec![],
                event: Default::default(),
                children: vec![count, buttons],
                style: Some(
                    "
                    width: 400px;
                    height: 120px;
                    display: flex;
                    flex-direction: column;
                    align-items: center;
                    justify-content: space-around;
                    "
                    .into(),
                ),
            }],
            style: Some(
                "
                width: 100%;
                height: 100%;
                display: grid;
                place-items: center;
                "
                .into(),
            ),
        }
    }
}
