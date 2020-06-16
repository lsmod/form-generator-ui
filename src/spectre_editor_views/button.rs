use yew::{html, Component, Callback, ComponentLink,Properties, Html, ShouldRender};

#[derive(Properties, Clone)]
pub struct Props {
    pub text: String,
    #[props(required)]
    pub onclick: Callback<()>,
}

pub enum Msg {
    Click
}

pub struct Button {
    props: Props,
    link: ComponentLink<Self>,
}


impl Component for Button
{
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Button {
            props,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => self.props.onclick.emit(())
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <button onclick=self.link.callback(|_|Msg::Click)>{self.props.text.as_str()}</button>
        }
    }
}
