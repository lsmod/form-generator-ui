use crate::app::form_model::form_model::Model;
use yew::html::InputData;
use yew::{
    html, html::Renderable, Callback, Children, Component, ComponentLink, Html, Properties,
    ShouldRender,
};
// TODO adds icons props add primary, secondary (outline)

#[derive(Properties, Clone)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    pub model: Model,
    // TODO make it optionnal by having an option
    // @see https://dev.to/deciduously/lets-build-a-rust-frontend-with-yew---part-2-1ech
    // @see https://github.com/deciduously/hunt-the-wumpus/blob/66938953772f75051658a222d2643ed881db694c/part2/src/components/controls.rs
    pub update_name: Callback<String>,
    pub update_title: Callback<String>,
    pub update_subtitle: Callback<String>,
    pub update_submit_label: Callback<String>,
}

pub enum Msg {
    UpdateName(String),
    UpdateTitle(String),
    UpdateSubtitle(String),
    UpdateSubmitLabel(String),
}

pub struct ModelForm {
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for ModelForm {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ModelForm { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateName(value) => self.props.update_name.emit(value),
            Msg::UpdateTitle(value) => self.props.update_title.emit(value),
            Msg::UpdateSubtitle(value) => self.props.update_subtitle.emit(value),
            Msg::UpdateSubmitLabel(value) => self.props.update_submit_label.emit(value),
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let model_subtitle: String = if let Some(model_subtitle) = &self.props.model.subtitle {
            model_subtitle.clone()
        } else {
            "".to_string()
        };
        let model_title: String = if let Some(model_title) = &self.props.model.title {
            model_title.clone()
        } else {
            "".to_string()
        };

        html! {
            <div>
                <h1>{"Model: "}{&self.props.model.name}</h1>
                <div>
                    <div class="form-horizontal">
                        <div class="form-group">
                            <div class="col-3 col-sm-12">
                                <label class="form-label" for="input_model-name">{"Name: "}</label>
                            </div>
                            <div class="col-9 col-sm-12">
                                <input
                                    type="text"
                                    required=true
                                    class="form-input"
                                    id="input_model-name"
                                    placeholder="Name"
                                    value={&self.props.model.name}
                                    oninput=self.link.callback(move |input: InputData|
                                    {
                                        Msg::UpdateName(input.value)
                                    }) />
                            </div>
                        </div>
                        <div class="form-group">
                            <div class="col-3 col-sm-12">
                                <label for="input_model-title" class="form-label">{"Title: "}</label>
                            </div>
                            <div class="col-9 col-sm-12">
                                <input
                                    type="text"
                                    required=true
                                    class="form-input"
                                    id="input_model-title"
                                    placeholder="Title"
                                    value={model_title}
                                    oninput=self.link.callback(move |input: InputData|
                                    {
                                        Msg::UpdateTitle(input.value)
                                    }) />
                            </div>
                        </div>

                        <div class="form-group">
                            <div class="col-3 col-sm-12">
                                <label class="form-label" for="input_model-subtitle">{"Sub-Title: "}</label>
                            </div>
                            <div class="col-9 col-sm-12">
                                <input
                                    type="text"
                                    class="form-input"
                                    id="input_model-subtitle"
                                    value={model_subtitle}
                                    oninput=self.link.callback(move |input: InputData|
                                    {
                                        Msg::UpdateSubtitle(input.value)
                                    })
                                />
                            </div>
                        </div>

                        <div class="form-group">
                            <div class="col-3 col-sm-12">
                                <label class="form-label" for="input_model-submit_label">{"Submit button label: "}</label>
                            </div>
                            <div class="col-9 col-sm-12">
                                <input
                                    type="text"
                                    class="form-input"
                                    id="input_model-submit_label"
                                    value={&self.props.model.submit_label}
                                    oninput=self.link.callback(move |input: InputData|
                                    {
                                        Msg::UpdateSubmitLabel(input.value)
                                    })
                                    />
                            </div>
                        </div>
                        { for self.props.children.iter() }
                    </div>
                </div>
            </div>
        }
    }
}
