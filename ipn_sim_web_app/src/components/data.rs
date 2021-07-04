use std::iter::once;

use yew::prelude::*;
use ipn_sim_lib::bit_vec::BitVec;


pub struct Data {
    link: ComponentLink<Self>,
    props: DataProps,
}

#[derive(Properties, Clone)]
pub struct DataProps {
    pub data: BitVec,
}

impl Component for Data {
    type Message = ();
    type Properties = DataProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let inner = unsafe {
            String::from_utf8_unchecked(self.props.data.to_bytes())
                .chars()
                .flat_map(|char| if char.is_alphanumeric() || char.is_whitespace() {
                    vec![
                        html! {
                        <span class="text-success me-1">
                            { char }
                        </span>
                        }
                    ]
                } else {
                    let mut bytes = [0; 4];
                    char.encode_utf8(&mut bytes);

                    let mut used_bytes = 0;
                    while bytes[used_bytes] != 0 {
                        used_bytes += 1;
                    }

                    bytes
                        .iter()
                        .take(used_bytes.max(1))
                        .map(|byte| html! {
                            <span class="me-1">
                                { format!("{:02x?}", byte) }
                            </span>
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Html>()
        };

        html! {
        <>
        { "[" }
        { inner }
        <span class="ms-n1">
            { "]" }
        </span>
        </>
        }
    }
}