use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::nav_menu::{NavItem, NavMenu};
use crate::components::router_page::{RouterPage, router_page_from_key};
use std::collections::HashMap;
use std::rc::Rc;

pub struct App {
    link: ComponentLink<Self>,
}

#[derive(Switch, Debug, Clone)]
pub enum Route {
    #[to = "/routers/{}"]
    Routers(String),
    #[to = "/"]
    Home,
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let nav_item = NavItem::SubMenu(vec![
            ("Home", NavItem::Link(Route::Home)),
            (
                "Routers",
                NavItem::SubMenu(vec![(
                    "Epidemic",
                    NavItem::SubMenu(vec![
                        ("Vanilla", NavItem::Link(Route::Routers("epidemic".into()))),
                        ("Acknowledged", NavItem::Link(Route::Routers("epidemic-ack".into()))),
                    ]),
                )]),
            ),
        ]);

        html! {
            <>
            <div class="mx-auto" style="width: var(--body-width)">
                <Router<Route> render={Router::render(switch)} />
            </div>
            <div class="position-fixed bottom-0 start-0 p-3" style="width: calc((100vw - var(--body-width)) / 2 - 100px)">
                <NavMenu title="IPN Router Zoo" item=nav_item/>
            </div>
            </>
        }
    }
}

fn switch(route: Route) -> Html {
    match route {
        Route::Routers(router_key) => router_page_from_key(router_key),
        Route::Home => Html::from("home"),
    }
}
