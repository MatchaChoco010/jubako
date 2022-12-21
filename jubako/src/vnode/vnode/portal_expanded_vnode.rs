use crate::vnode::vnode::{BundledEventHandler, StyleExtractedVNode};

pub(crate) enum PortalExpandedVNode {
    Text {
        text: String,
    },
    Element {
        tag: String,
        class: Vec<String>,
        props: Vec<String>,
        event: BundledEventHandler,
        children: Vec<PortalExpandedVNode>,
    },
}

// vnode that expanded portals
pub(crate) struct PortalExpandedVNodes {
    pub(crate) main: Vec<PortalExpandedVNode>,
    pub(crate) portals: Vec<PortalExpandedVNode>,
}
impl PortalExpandedVNodes {
    pub(crate) fn expand(mut vnode: StyleExtractedVNode) -> Self {
        let mut portals: Vec<StyleExtractedVNode> = vec![];

        fn traverse_main(
            main: &mut Vec<StyleExtractedVNode>,
            portals: &mut Vec<StyleExtractedVNode>,
        ) {
            let mut i = 0;
            while i < main.len() {
                match &mut main[i] {
                    StyleExtractedVNode::Portal { .. } => {
                        let vnode = main.remove(i);
                        if let StyleExtractedVNode::Portal { mut children, .. } = vnode {
                            portals.append(&mut children);
                        }
                    }
                    StyleExtractedVNode::Element { children, .. } => {
                        traverse_main(children, portals);
                        i += 1;
                    }
                    _ => {
                        i += 1;
                    }
                }
            }
        }
        fn traverse_portals(portals: &mut Vec<StyleExtractedVNode>) {
            let mut i = 0;
            while i < portals.len() {
                let mut new_portals = vec![];

                match &mut portals[i] {
                    StyleExtractedVNode::Portal { .. } => {
                        let vnode = portals.remove(i);
                        if let StyleExtractedVNode::Portal { mut children, .. } = vnode {
                            new_portals.append(&mut children);
                        }
                    }
                    StyleExtractedVNode::Element { children, .. } => {
                        traverse_main(children, &mut new_portals);
                        i += 1;
                    }
                    _ => {
                        i += 1;
                    }
                }

                portals.append(&mut new_portals);
            }
        }

        if let StyleExtractedVNode::Element {
            ref mut children, ..
        } = vnode
        {
            traverse_main(children, &mut portals);
        } else {
            unreachable!();
        }
        traverse_portals(&mut portals);

        fn convert(vnodes: Vec<StyleExtractedVNode>) -> Vec<PortalExpandedVNode> {
            let mut ret_vnodes = vec![];
            for vnode in vnodes {
                match vnode {
                    StyleExtractedVNode::Text { text } => {
                        ret_vnodes.push(PortalExpandedVNode::Text { text })
                    }
                    StyleExtractedVNode::Element {
                        tag,
                        class,
                        props,
                        event,
                        children,
                    } => ret_vnodes.push(PortalExpandedVNode::Element {
                        tag,
                        class,
                        props,
                        event,
                        children: convert(children),
                    }),
                    StyleExtractedVNode::Portal { .. } => unreachable!(),
                }
            }
            ret_vnodes
        }

        PortalExpandedVNodes {
            main: convert(vec![vnode]),
            portals: convert(portals),
        }
    }
}
