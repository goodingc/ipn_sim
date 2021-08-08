use yew::prelude::*;
use std::collections::HashMap;
use crate::components::scenario_grid::ScenarioGrid;
use crate::router_details::{RouterDetails};
use crate::router_details::ack_epidemic::AckEpidemic;
use ipn_sim_lib::router::Router;
use ipn_sim_lib::routers::epidemic::epidemic::Epidemic;
use ipn_sim_lib::routers::epidemic::flavours::{vanilla::Vanilla, ack::Ack};

pub struct RouterPage<R: Router + RouterDetails + Clone + 'static> {
    link: ComponentLink<Self>,
}


impl<R: Router + RouterDetails + Clone + 'static> Component for RouterPage<R> {
    type Message = ();
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <h1>{R::name()}</h1>
                <ScenarioGrid<R>/>
            </>
        }
    }
}


pub fn router_page_from_key(key: impl ToString) -> Html {
    match key.to_string().as_str() {
        "epidemic" => html! {
                <RouterPage<Epidemic<Vanilla>>/>
            },
        "epidemic-ack" => html! {
                <RouterPage<Epidemic<Ack>>/>
            },
        _ => {
            unreachable!()
        }
    }
}