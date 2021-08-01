use std::collections::HashMap;
use std::iter;
use std::rc::Rc;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::app::Route;

const HEADER_HEIGHTS: [u8; 4] = [48, 38, 33, 28];

pub struct NavMenu {
    link: ComponentLink<Self>,
    props: NavMenuProps,
    expanded: bool,
}

type NavItems = Vec<(&'static str, NavItem)>;

#[derive(Clone)]
pub enum NavItem {
    Link(Route),
    SubMenu(NavItems),
}

#[derive(Properties, Clone)]
pub struct NavMenuProps {
    pub title: &'static str,
    pub item: NavItem,
    #[prop_or(0)]
    pub depth: u8,
}

pub enum NavMenuMessage {
    SetExpended(bool),
}

impl Component for NavMenu {
    type Message = NavMenuMessage;
    type Properties = NavMenuProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            expanded: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            NavMenuMessage::SetExpended(expanded) => {
                let changed = expanded != self.expanded;
                if changed {
                    self.expanded = expanded;
                }
                changed
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let status_icon_class = match &self.props.item {
            NavItem::Link(_) => "bi-link",
            NavItem::SubMenu(_) => {
                if self.expanded {
                    "bi-caret-up-fill"
                } else {
                    "bi-caret-left-fill"
                }
            }
        };
        let title_element = html! {
            <div class="d-flex justify-content-between">
                <@{format!("h{}", self.props.depth + 1)} class="m-0">{ self.props.title }</@>
                <div class="d-flex justify-content-center" style="width:50px">
                    <@{format!("h{}", self.props.depth + 1)} class="m-0">
                        <i class=format!("bi {}", status_icon_class)></i>
                    </@>
                </div>
            </div>
        };

        match &self.props.item {
            NavItem::Link(route) => html! {
                <RouterAnchor<Route> route=route.clone() classes="text-decoration-none text-dark">
                    {title_element}
                </RouterAnchor<Route>>
            },
            NavItem::SubMenu(sub_items) => {
                let new_depth = self.props.depth + 1;
                let sub_elements = sub_items
                    .iter()
                    .map(|(sub_title, sub_item)| {
                        html! {
                            <Self title=sub_title.clone() item=sub_item.clone() depth=new_depth/>
                        }
                    })
                    .rev()
                    .collect::<Html>();

                let style = format!(
                    "transition: height 1s; overflow-y: hidden; height: {}",
                    if self.expanded {
                        // HEADER_HEIGHTS[new_depth as usize] as usize * sub_items.len()
                        "auto"
                    } else {
                        "0"
                    }
                );

                // elements.push(html! {
                //     <div class="ms-5" style=style>
                //         {sub_elements}
                //     </div>
                // });
                //
                // let expanded = self.expanded;
                // (
                //     self.link.callback(move |_| NavMenuMessage::SetExpended(!expanded)),
                //     if expanded {
                //         "bi-caret-up-fill"
                //     } else {
                //         "bi-caret-left-fill"
                //     }
                // )

                let expanded = self.expanded;
                html! {
                    <>
                    <div class="ms-5" style=style>
                        {sub_elements}
                    </div>
                    <div onclick=self.link.callback(move |_| NavMenuMessage::SetExpended(!expanded))>
                        {title_element}
                    </div>
                    </>
                }
            }
        }
    }
}
