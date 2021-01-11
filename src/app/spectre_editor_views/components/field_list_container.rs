use yew::{
    html, html::Renderable, Children, Component, ComponentLink, Html, Properties, ShouldRender,
};

#[derive(Properties, Clone)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

pub struct FieldListContainer {
    props: Props,
}

impl Component for FieldListContainer {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        FieldListContainer { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="model-field-container">
                <h2>{"Fields"}</h2>
                <table class="table table-striped table-hover">
                  <thead>
                    <tr>
                      <th>{"Name"}</th>
                      <th>{"Type"}</th>
                      <th>{"Label"}</th>
                      <th>{"Placeholder"}</th>
                      <th>{"Required"}</th>
                      <th>{"Order"}</th>
                      <th>{"Action"}</th>
                    </tr>
                  </thead>
                  <tbody>
                  { for self.props.children.iter() }
                  </tbody>
                </table>
            </div>
        }
    }
}
