pub mod parsed_byte_sections;

use ipn_sim_lib::bit_vec::BitVec;
use yew::prelude::*;
use ipn_sim_lib::utils::{Data as DataType};
use std::ascii;
use std::char::decode_utf16;
use std::str;
use crate::components::data::parsed_byte_sections::{ParsedBytesSections, ParsedBytesSection};
use itertools::Itertools;

pub struct Data {
    link: ComponentLink<Self>,
    props: DataProps,
}

#[derive(Properties, Clone)]
pub struct DataProps {
    pub data: DataType,
}

impl Component for Data {
    type Message = ();
    type Properties = DataProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let content_html = ParsedBytesSections::from_bytes(self.props.data.as_ref())
            .into_iter()
            .map(|section| match section {
                ParsedBytesSection::Chars(chars) => {
                    let string = chars.into_iter().collect::<String>();
                    html! {
                        <div class="ms-1 text-success">
                            {string}
                        </div>
                    }
                },
                ParsedBytesSection::Bytes(bytes) => {
                    let tooltip = bytes
                        .into_iter()
                        .map(|byte| format!("{:02x}", byte))
                        .join(" ");
                    html!{
                        <div class="ms-1" title=tooltip>
                            {"..."}
                        </div>
                    }
                }
            }).collect::<Html>();

        html! {
            <div class="d-flex ms-n1">
                {content_html}
            </div>
        }
    }
}
