use std::rc::Rc;
use std::cell::RefCell;
use yew::prelude::*;
use yew::virtual_dom::VNode;

pub struct Sidebar {
    link: ComponentLink<Self>,
    props: SidebarProps,
    expanded: bool,
}

#[derive(Properties, Clone)]
pub struct SidebarProps {
    pub side: SidebarSide,
    pub children: Children,
    pub title: String,
}

pub enum SidebarMessage {
    Toggle
}

#[derive(Clone)]
pub enum SidebarSide {
    Left,
    Right,
}

impl Component for Sidebar {
    type Message = SidebarMessage;
    type Properties = SidebarProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            expanded: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            SidebarMessage::Toggle => {
                self.expanded = !self.expanded;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let container_style = format!(
            "{}: {}",
            match self.props.side {
                SidebarSide::Left => "left",
                SidebarSide::Right => "right"
            },
            if self.expanded {
                "0"
            } else {
                "-25vw"
            }
        );
        let button_content = if self.expanded {
            html! {
                <i
                    class="bi bi-x-lg"
                ></i>
            }
        } else {
            VNode::from(&self.props.title)
        };
        let button_class = format!(
            "btn btn-outline-{}",
            if self.expanded {
                "danger"
            } else {
                "light"
            }
        );
        let container_class = format!(
            "sidebar-container d-flex {} align-items-start",
            match self.props.side {
                SidebarSide::Left => "flex-row",
                SidebarSide::Right => "flex-row-reverse"
            }
        );

        html! {
            <div class=container_class style=container_style>
                <div class="sidebar bg-light p-3">
                    { self.props.children.clone() }
                </div>
                <div class="m-3">
                    <button
                        type="button"
                        class=button_class
                        onclick=self.link.callback(|_| SidebarMessage::Toggle)
                    >{button_content}</button>
                </div>
            </div>
        }
    }
}