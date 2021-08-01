use yew::prelude::*;
use std::collections::HashMap;

pub struct RouterPage {
    link: ComponentLink<Self>,
    props: RouterPageProps,
}

#[derive(Properties, Clone)]
pub struct RouterPageProps {
    pub name: String,
}

impl Component for RouterPage {
    type Message = ();
    type Properties = RouterPageProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Mauris id dignissim lectus, et varius dui. Vivamus mattis in massa ac facilisis. Pellentesque vitae tincidunt odio. Donec vestibulum magna vel turpis semper finibus. Maecenas et aliquam massa, at pellentesque est. Pellentesque tellus felis, rutrum ac turpis et, posuere egestas nibh. Morbi sapien libero, consectetur a rhoncus a, venenatis tincidunt felis. Fusce aliquam dictum urna, efficitur aliquet dolor interdum id. Sed fringilla est ut felis sollicitudin, vel consequat orci ullamcorper.

Mauris dapibus posuere ipsum nec tempus. Nulla facilisi. Morbi sollicitudin imperdiet justo, nec consectetur augue rhoncus eu. Cras ultricies tincidunt turpis a vehicula. Nunc luctus fermentum quam nec dictum. Praesent lacinia mattis libero, a porta dolor placerat in. Integer mattis ipsum magna, vel placerat libero eleifend eu. Mauris dignissim tempus neque. Donec in quam vehicula, semper orci ac, mollis risus. In hac habitasse platea dictumst. Vestibulum metus purus, dignissim sed magna sit amet, malesuada vulputate nulla. Sed maximus ligula metus, sit amet rutrum erat fringilla id.

Duis quis dolor augue. Nunc a suscipit tortor. Praesent sed lorem dui. Phasellus sollicitudin dui nec consectetur elementum. Pellentesque congue metus consequat placerat egestas. Interdum et malesuada fames ac ante ipsum primis in faucibus. Quisque ac gravida orci. Nullam feugiat nec erat a hendrerit. Aliquam et ultricies orci. Cras mattis condimentum magna nec scelerisque. Nullam malesuada et velit ut egestas. Donec ut urna aliquam ipsum consectetur rhoncus. Donec vitae ultrices orci. Phasellus fringilla sapien in placerat venenatis. Nam pellentesque lacinia dignissim. Lorem ipsum dolor sit amet, consectetur adipiscing elit.

Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Pellentesque id odio non lorem lobortis porta sed non felis. In sed tincidunt justo, id aliquet diam. In et scelerisque ante. Nulla porttitor pulvinar tincidunt. Mauris lectus orci, elementum in purus in, vulputate sagittis neque. Suspendisse suscipit purus eu magna suscipit ultricies. Morbi hendrerit gravida leo in lobortis. Mauris ultrices pulvinar ante nec mattis. Sed eget eros cursus, euismod elit quis, tincidunt quam.

Interdum et malesuada fames ac ante ipsum primis in faucibus. Aenean id posuere sapien. Proin suscipit vel tellus eget scelerisque. Phasellus condimentum ligula ac neque blandit convallis. Nullam euismod sagittis est, sed laoreet tellus molestie eu. Morbi pretium, nisi sit amet fermentum commodo, odio ligula elementum libero, consectetur tristique libero nulla vestibulum velit. Donec tristique, velit blandit interdum placerat, arcu urna interdum elit, ut tristique ante massa vestibulum tortor. Nullam porta odio tortor. Vestibulum id diam felis. Duis ornare tempus tristique. Donec tortor nibh, semper quis auctor nec, tempus vel ipsum. Nunc gravida ut purus vel finibus. Vestibulum elementum velit ipsum, at varius tellus fermentum nec."
            .into()
    }
}
