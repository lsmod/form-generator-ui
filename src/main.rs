#![recursion_limit="512"]
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew::html::InputData;
mod form_model;
pub use form_model::form_model::*;
use crate::form_model::form_model::FieldDataType::*;

mod react_native;
pub use react_native::to_html_type;


struct App {
    clicked: bool,
    link: ComponentLink<App>,
    state: State
    // state -> model -> fields
    // state -> model -> WIPfield (to validate and be able to store no finished work)
}

pub struct State {
    current_field_id: Option<usize>,
    current_field: Option<Field>,
    model: Model,
}
// TODO:
// - think about how I will retreive a specific field (id ?)
// - think about when creating a field and all require param are not fullfieled


enum Msg {
    Click,
    UpdateName(String),
    UpdateTitle(String),
    UpdateSubtitle(String),
    UpdateSubmitLabel(String)
    /*


    EditField,
    UpdateField

    NewField,
    CreateField,

    RemoveField,
    DeleteField,

    UpdateFieldType,
    UpdateFieldLabel,
    UpdateFieldName,
    UpdateFieldPlaceHolder,

    NewListValue,
    CreateFieldValue,
    EditListValue,
    UpdateListValue,

    NewValidation,
    CreateValidation,
    EditValidation,
    UpdateValidation,

    NewValidationRule,
    CreateValidationRule,
    EditValidationRule,
    UpdateValidationRule,
    */
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut fields = vec!();
        fields.push(Field {
            name: "username".to_string(),
            data_type: Text,
            label: "Your name".to_string(),
            required: false,
            placeholder: "Enter your username".to_string(),
            validation: None
        });
        App {
            link,
            clicked: false,
            state: State {
                current_field_id: Some(0),
                current_field: None,
                model: Model {
                    name: "React-Native".to_string(),
                    title: Some("new title".to_string()),
                    submit_label: "send".to_string(),
                    subtitle: Some("a model to generate react-native forms".to_string()),
                    fields: fields
                }
            }
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {
                self.clicked = !self.clicked;
                true // Indicate that the Component should re-render
            },
            Msg::UpdateName(name) => {
                self.state.model.name = name;
                true // TODO: return true only if new id != previous one ?
            }
            Msg::UpdateTitle(name) => {
                self.state.model.title = Some(name);
                true // TODO: return true only if new id != previous one ?
            }
            Msg::UpdateSubtitle(name) => {
                self.state.model.subtitle = Some(name);
                true // TODO: return true only if new id != previous one ?
            }
            Msg::UpdateSubmitLabel(name) => {
                self.state.model.submit_label = name;
                true // TODO: return true only if new id != previous one ?
            }

        }
    }

    fn view(&self) -> Html {
        let model_title: String = if let Some(model_title) = &self.state.model.title { model_title.clone() } else { "".to_string() };
        let model_subtitle: String = if let Some(model_subtitle) = &self.state.model.subtitle { model_subtitle.clone() } else { "".to_string() };
        let id = match self.state.current_field_id {
            Some(id) => format!("{:?}", id),
            None => "pas d'id".to_string()
        };
        let editing = if let Some(editing) = self.state.current_field_id { self.view_editing_field(editing) } else { html!{""} };

        // TODO: add on change for input
        html! {
            <div>
                <div class="model-header">
                    <h1>{"Model: "}{&self.state.model.name}</h1>
                    <div class="model-property">
                        <div class="model-name-container">
                            {"Name: "}
                            <input type="text" name="model-name" oninput=self.link.callback(move |input: InputData|
                            {
                                Msg::UpdateName(input.value)
                            }) value={&self.state.model.name} />
                        </div>
                    </div>
                    <div class="model-property">
                        <div class="model-title-container">
                            {"Title: "}
                            <input type="text" name="model-title" oninput=self.link.callback(move |input: InputData|
                            {
                                Msg::UpdateTitle(input.value)
                            }) value={model_title} />
                        </div>
                    </div>
                    <div class="model-property">
                        <div class="model-subtitle-container">
                            {"Sub-Title: "}
                            <input type="text" name="model-subtitle" oninput=self.link.callback(move |input: InputData|
                            {
                                Msg::UpdateSubtitle(input.value)
                            }) value={model_subtitle} />
                        </div>
                    </div>
                    <div class="model-property">
                        <div class="model-submit_label-container">
                            {"Submit button label: "}
                            <input type="text" name="model-submit_label" oninput=self.link.callback(move |input: InputData|
                            {
                                Msg::UpdateSubmitLabel(input.value)
                            }) value={&self.state.model.submit_label} />
                        </div>
                    </div>
                    <button>{"New field"}</button>
                    { editing }
                    <div class="model-field-container">
                        <ul>
                        { for self.state.model.fields.iter().enumerate().map(|(index, field)| self.view_field(index, field)) }
                        </ul>
                    </div>
                </div>
            </div>
        }
    }
}

impl App {
    fn view_field(&self, index: usize, field: &Field) -> Html {
        html! {
            <li>
                {index}
                {"Name:"}{&field.name}
                {"Type:"}{&field.data_type}
                {"Label:"}{&field.label}
                {"Placeholder"}{&field.placeholder}
                {"Required"}{&field.required}
                <button>{"edit"}</button>
                <button>{"delete"}</button>
            </li>
        }
    }

    fn view_new_field(&self) -> Html {
        html! {
            <div class="model-field-creating">
                {"a new field"}
            </div>
        }
    }

    fn view_editing_field(&self, index: usize) -> Html {
        self.state.current_field = Some(self.state.model.fields[index].clone());
        let field = self.state.current_field.clone().unwrap();
        html! {
            <div class="model-field-editing">
                <h3>{"Editing field: "}{&field.name}</h3>
                {"Name:"} <input type="text" value={&field.name} />
                {"Type:"} {self.view_select_type(field.data_type.clone())}<br/>
                {"Label:"} <input type="text" value={&field.label} />
                {"Placeholder:"} <input type="text" value={&field.placeholder} /><br/>
                {"Required :"} <input type="checkbox" name="required" required={field.required} />
                <button>{"cancel"}</button>
                <button>{"save"}</button>
            </div>
        }
    }

    fn view_select_type(&self, field_type: FieldDataType) -> Html {
        html! {
            <select>
                <option value="Text" selected={field_type == FieldDataType::Text}>{"Text"}</option>
                <option value="Number">{"Number"}</option>
                <option value="Phone">{"Phone"}</option>
            </select>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
