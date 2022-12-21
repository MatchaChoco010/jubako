use lightningcss::{
    stylesheet::{MinifyOptions, ParserOptions, PrinterOptions, StyleSheet},
    targets::Browsers,
};
use std::collections::HashMap;
use uuid::Uuid;

use crate::vnode::{
    command::StyleDifferenceCommand,
    vnode::{BundledEventHandler, MessageHandledVNode},
};

// StyleExtractedVNode is a vnode with style replaced to class
pub(crate) enum StyleExtractedVNode {
    Text {
        text: String,
    },
    Element {
        tag: String,
        class: Vec<String>,
        props: Vec<String>,
        event: BundledEventHandler,
        children: Vec<StyleExtractedVNode>,
    },
    Portal {
        children: Vec<StyleExtractedVNode>,
    },
}
impl StyleExtractedVNode {
    // extract style from vnode and replace to class, and generate style diff commands
    pub(crate) fn extract(
        vnode: MessageHandledVNode,
        styles: &mut HashMap<String, Uuid>,
    ) -> (Self, Vec<StyleDifferenceCommand>) {
        let mut style_commands = vec![];

        // use hashmap to detect removed styles
        let mut prev_styles = HashMap::new();
        for (_, uuid) in styles.iter() {
            prev_styles.insert(uuid.clone(), false);
        }

        // record new styles
        let mut new_styles = HashMap::<String, Uuid>::new();

        // replace style to uuid class
        let mut style_to_class = |vnode: &mut MessageHandledVNode| {
            if let MessageHandledVNode::Element {
                ref mut style,
                ref mut class,
                ..
            } = vnode
            {
                if let Some(style_text) = style.clone() {
                    if styles.contains_key(&style_text) {
                        // already registered style
                        let uuid = styles[&style_text].clone();
                        class.push(format!("style-{}", uuid.to_string()));
                        *style = None;
                        prev_styles.insert(uuid, true);
                    } else if new_styles.contains_key(&style_text) {
                        // already registered style
                        let uuid = new_styles[&style_text].clone();
                        class.push(format!("style-{}", uuid.to_string()));
                        *style = None;
                    } else {
                        // new style
                        let uuid = Uuid::new_v4();
                        class.push(format!("style-{}", uuid.to_string()));
                        new_styles.insert(style_text.clone(), uuid);
                    }
                }
            }
        };
        // extract style from vnode
        fn traverse(
            mut vnode: MessageHandledVNode,
            style_to_class: &mut dyn FnMut(&mut MessageHandledVNode),
        ) -> StyleExtractedVNode {
            style_to_class(&mut vnode);
            match vnode {
                MessageHandledVNode::Text { text } => StyleExtractedVNode::Text { text },
                MessageHandledVNode::Element {
                    tag,
                    class,
                    props,
                    children,
                    event,
                    ..
                } => {
                    let mut new_children = vec![];
                    for child in children {
                        new_children.push(traverse(child, style_to_class));
                    }
                    StyleExtractedVNode::Element {
                        tag,
                        class,
                        props,
                        children: new_children,
                        event,
                    }
                }
                MessageHandledVNode::Portal { children, .. } => {
                    let mut new_children = vec![];
                    for child in children {
                        new_children.push(traverse(child, style_to_class));
                    }
                    StyleExtractedVNode::Portal {
                        children: new_children,
                    }
                }
            }
        }
        let vnode = traverse(vnode, &mut style_to_class);

        // styles that are not used in the new vnode are removed
        for (uuid, is_used) in prev_styles {
            if !is_used {
                let style_text = styles.iter().find(|(_, v)| **v == uuid).unwrap().0.clone();
                styles.remove(&style_text);
                style_commands.push(StyleDifferenceCommand::RemoveStyle {
                    class_name: uuid.to_string(),
                });
            }
        }

        // register new styles
        for (style_text, uuid) in new_styles {
            styles.insert(style_text.clone(), uuid);

            let style_sheet = format!(".style-{} {{ {} }}", uuid, style_text);
            let mut style_sheet: StyleSheet = StyleSheet::parse(
                &style_sheet,
                ParserOptions {
                    nesting: true,
                    ..Default::default()
                },
            )
            .unwrap();
            style_sheet.minify(MinifyOptions::default()).unwrap();
            let style_sheet = style_sheet
                .to_css(PrinterOptions {
                    minify: true,
                    targets: Some(Browsers {
                        chrome: Some(108 << 16),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .unwrap()
                .code;

            style_commands.push(StyleDifferenceCommand::AddStyle {
                class_name: format!("style-{}", uuid.to_string()),
                value: style_sheet,
            });
        }

        // return style extracted vnode and style diff commands
        (vnode, style_commands)
    }
}
