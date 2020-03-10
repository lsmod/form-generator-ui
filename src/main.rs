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

// TODO move into separated module
struct App {
    link: ComponentLink<App>,
    state: State
    // state -> model -> fields
    // state -> model -> WIPfield (to validate and be able to store no finished work)
}

pub struct State {
    current_field_id: Option<usize>, // merge these two
    current_field: Option<Field>, // merge these two into a single wrapper to avoid inconsistend state
    model: Model,
}
// TODO:
// - think about how I will retreive a specific field (id ?)
// - think about when creating a field and all require param are not fullfieled


enum Msg {
    UpdateName(String),
    UpdateTitle(String),
    UpdateSubtitle(String),
    UpdateSubmitLabel(String),

    EditField(usize),
    CancelEditField,
    UpdateField,

    UpdateFieldName(String),
    UpdateFieldLabel(String),
    UpdateFieldPlaceHolder(String),
    /*



    NewField,
    CreateField,

    RemoveField,
    DeleteField,

    UpdateFieldType,

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
            state: State { // TODO : create a builder for this
                current_field_id: None,
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
            Msg::EditField(index) => {
                self.state.current_field_id = Some(index);
                self.state.current_field = Some(self.state.model.fields[index].clone());
                true
            }
            Msg::CancelEditField => {
                self.state.current_field_id = None;
                self.state.current_field = None;
                true
            }
            Msg::UpdateField => {
                match self.state.current_field_id {
                    Some(index) => {
                        self.state.current_field_id = None;
                        self.state.model.fields[index] = self.state.current_field.as_ref().unwrap().clone();
                        self.state.current_field = None;
                    }
                    None => ()
                }
                true
            }
            Msg::UpdateFieldName(name) => {
                match &mut self.state.current_field {
                    Some(field) => field.name = name,
                    None => ()
                };
                true
            }
            Msg::UpdateFieldLabel(label) => {
                match &mut self.state.current_field {
                    Some(field) => field.label = label,
                    None => ()
                };
                true
            }
            Msg::UpdateFieldPlaceHolder(placeholder) => {
                match &mut self.state.current_field {
                    Some(field) => field.placeholder = placeholder,
                    None => ()
                };
                true
            }
        }
    }

    fn view(&self) -> Html {
        let model_title: String = if let Some(model_title) = &self.state.model.title { model_title.clone() } else { "".to_string() };
        let model_subtitle: String = if let Some(model_subtitle) = &self.state.model.subtitle { model_subtitle.clone() } else { "".to_string() };

        let editing_view = if let Some(_editing_view) = self.state.current_field_id { self.view_editing_field() } else { html!{""} };

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
                    { editing_view }
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
                <button onclick=self.link.callback(move |_| Msg::EditField(index))>{"edit"}</button>
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

    fn view_editing_field(&self) -> Html {
        // TODO: this is the view who only got a &self ,
        // modification are to be done in update that got a &mut self !!!
        // TODO: add an action to display view field (toggle a state & copy field data)
        let field = self.state.current_field.as_ref().unwrap();
        html! {
            <div class="model-field-editing">
                <h3>{"Editing field: "}{&field.name}</h3>
                {"Name:"}
                <input
                    type="text"
                    value={&field.name}
                    oninput=self.link.callback(move |input: InputData|
                        {
                            Msg::UpdateFieldName(input.value)
                        })
                />
                {"Type:"} {self.view_select_type(field.data_type.clone())}<br/>
                {"Label:"}
                <input
                    type="text"
                    value={&field.label}
                    oninput=self.link.callback(move |input: InputData|
                        {
                            Msg::UpdateFieldLabel(input.value)
                        })
                />
                {"Placeholder:"} <input
                    type="text"
                    value={&field.placeholder}
                    oninput=self.link.callback(move |input: InputData|
                        {
                            Msg::UpdateFieldPlaceHolder(input.value)
                        })
                /><br/>
                {"Required :"} <input type="checkbox" name="required" required={field.required} />
                <button onclick=self.link.callback(|_| Msg::CancelEditField)>{"cancel"}</button>
                <button onclick=self.link.callback(|_| Msg::UpdateField)>{"save"}</button>
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
