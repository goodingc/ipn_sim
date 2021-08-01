use std::rc::Rc;
use yew::prelude::*;

pub struct TabSwitcher {
    link: ComponentLink<Self>,
    props: TabSwitcherProps,
    tab_index: usize,
}

#[derive(Properties, Clone)]
pub struct TabSwitcherProps {
    pub tabs: Rc<Vec<(String, Html)>>,
}

pub enum TabSwitcherMessage {
    SetTab(usize),
}

impl Component for TabSwitcher {
    type Message = TabSwitcherMessage;
    type Properties = TabSwitcherProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            tab_index: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            TabSwitcherMessage::SetTab(index) => {
                if self.tab_index != index {
                    self.tab_index = index;
                    return true;
                }
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let titles = self
            .props
            .tabs
            .iter()
            .enumerate()
            .map(|(index, (title, _))| {
                let class = if self.tab_index == index {
                    "nav-link active"
                } else {
                    "nav-link"
                };
                html! {
                <li class="nav-item">
                    <a
                        class=class
                        onclick=self.link.callback(move |_| TabSwitcherMessage::SetTab(index))
                    >{ title }</a>
                </li>
                }
            })
            .collect::<Html>();
        let content = self.props.tabs[self.tab_index].1.clone();
        // let tabs = self.props.tabs
        //     .iter()
        //     .enumerate()
        //     .map(|(index, (_, html))| {
        //         let class = if self.tab_index == index {
        //             ""
        //         } else {
        //             "d-none"
        //         };
        //         html! {
        //         <div class=class>
        //             { html.clone() }
        //         </div>
        //         }
        //     })
        //     .collect::<Html>();
        html! {
        <>
        <div class="row mb-2">
            <div class="col">
                <ul class="nav nav-pills">
                    { titles }
                </ul>
            </div>
        </div>
        { content }
        </>
        }
    }
}
