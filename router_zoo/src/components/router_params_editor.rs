use yew::prelude::*;
use crate::router_details::{RouterDetails, RouterParamType};
use std::rc::Rc;
use crate::components::scenario_grid::{ScenarioGrid, ScenarioGridMessage};
use web_sys::HtmlInputElement;
use ipn_sim_lib::router::Router;

pub struct RouterParamsEditor<R: Router + RouterDetails + Clone + 'static> {
    link: ComponentLink<Self>,
    props: RouterParamsEditorProps<R>,
    param_input_refs: Box<[NodeRef]>,
    params: Box<[String]>,
}

pub enum RouterParamsEditorMessage {
    Update
}

#[derive(Properties, Clone)]
pub struct RouterParamsEditorProps<R: Router + RouterDetails + Clone + 'static> {
    pub on_update: Callback<R>,
}

impl<R: Router + RouterDetails + Clone + 'static> Component for RouterParamsEditor<R> {
    type Message = RouterParamsEditorMessage;
    type Properties = RouterParamsEditorProps<R>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            param_input_refs: (0..R::params().len())
                .map(|_| NodeRef::default())
                .collect(),
            params: R::default_params(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Self::Message::Update => {
                self.props.on_update.emit(
                    R::build_instance(self.param_input_refs
                        .iter()
                        .map(|node_ref| {
                            node_ref.cast::<HtmlInputElement>().unwrap().value()
                        }).collect())
                );
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let editor_html = R::params()
            .iter()
            .zip(self.param_input_refs.iter())
            .zip(self.params.iter())
            .map(|(((name, param_type), node_ref), value)| {
                let input_html = match param_type {
                    RouterParamType::Number => html! {
                        <input ref=node_ref.clone() class="form-control" value=value.clone() type="number"/>
                    }
                };
                html! {
                    <div class="mb-3">
                        <label class="form-label">{name}</label>
                        {input_html}
                    </div>
                }
            }).collect::<Html>();

        html! {
            <div class="card m-3">
                <div class="card-body">
                    <h5 class="card-title">{"Router Params"}</h5>
                    {editor_html}
                    <button
                        onclick=self.link.callback(|_| Self::Message::Update)
                        class="btn btn-primary"
                    >{"Update"}</button>
                </div>
            </div>
        }
    }
}