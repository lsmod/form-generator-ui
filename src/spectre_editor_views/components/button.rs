use yew::{
    html, html::Renderable, Callback, Children, Component, ComponentLink, Html, Properties,
    ShouldRender,
};

// TODO adds icons props add primary, secondary (outline)

#[derive(Properties, Clone)]
pub struct Props {
    pub children: Children,
    pub text: String,
    pub primary: bool,
    pub id: Option<String>,
    // TODO make it optionnal by having an option
    // @see https://dev.to/deciduously/lets-build-a-rust-frontend-with-yew---part-2-1ech
    // @see https://github.com/deciduously/hunt-the-wumpus/blob/66938953772f75051658a222d2643ed881db694c/part2/src/components/controls.rs
    #[props(required)]
    pub onclick: Callback<()>,
}

pub enum Msg {
    Click,
}

pub struct Button {
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for Button {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Button { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => self.props.onclick.emit(()),
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let btn_class = if self.props.primary {
            "btn btn-primary"
        } else {
            "btn"
        };
        match &self.props.id {
            Some(id) => html! {
                <button id=id class=btn_class type="button" onclick=self.link.callback(|_|Msg::Click)>
                    {self.props.children.render()}
                </button>
            },
            None => html! {
                <button class=btn_class type="button" onclick=self.link.callback(|_|Msg::Click)>
                    {self.props.children.render()}
                </button>
            },
        }
    }
}
