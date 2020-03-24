#![recursion_limit="512"]
extern crate strum;
#[macro_use]
extern crate strum_macros;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use crate::spectre_editor_views::view_creating_enum_values;
use crate::spectre_editor_views::view_editing_enum_value;
use strum::IntoEnumIterator;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew::components::Select;
use yew::services::ConsoleService;
use yew::html::InputData;

mod form_model;
pub use form_model::form_model::*;
pub use form_model::form_model::FieldDataType::*;

mod react_native;
pub use react_native::to_html_type;

mod spectre_editor_views;
use spectre_editor_views::view_field_item;
use spectre_editor_views::view_enum_values_item;

#[derive(Debug)]
pub struct EditingEnumValue {
    index: usize,
    enum_value: EnumValues,
}

#[derive(Debug)]
struct EditingField {
    id: usize,
    field: Field
}

#[derive(Debug)]
enum EditorMode {
    EditingField(EditingField),
    CreatingField(Field),
    Listing,
}

// TODO move into separated module
pub struct App {
    link: ComponentLink<App>,
    state: State,
    console: ConsoleService,
}

#[derive(Debug)]
pub struct State {
    editing_enum_value: Option<EditingEnumValue>,
    creating_enum_value: Option<EnumValues>,
    editing_mode: EditorMode,
    model: Model,
}
// TODO:
// - separator

// -> divide this monster file
// - handle all FieldType using enum
// move up/move down enum value
// move up

// - think about validation...
// -> create error Struct with message
// -> call validation before exporting (look for all empty string)

// - camelCase, snake_case
// - move into a separated module
// - divide into sub Component (edit field, new field, header)
pub enum Msg {
    UpdateName(String),
    UpdateTitle(String),
    UpdateSubtitle(String),
    UpdateSubmitLabel(String),

    EditField(usize), // TODO display enum_value if type = radio
    CancelEditField,
    UpdateField,

    UpdateFieldName(String),
    UpdateFieldLabel(String),
    UpdateFieldPlaceHolder(String),
    UpdateFieldType(FieldDataType),
    ToggleFieldRequired,

    NewField,
    CancelNewField,
    CreateField,

    EditFieldEnumValues(usize), // click edit on enum value
    UpdateFieldEnumValues, // click on save (editing enum value)
    CancelEditEnumValues,
    UpdateEnumValueValue(String),
    UpdateEnumValueLabel(String),
    DeleteEnumValue(usize),
    NewEnumValue, // click new enum value
    CancelNewEnumValue,
    CreateNewEnumValue, // click save new enum value
    UpdateNewEnumValueValue(String), // this is when adding a new field
    UpdateNewEnumValueLabel(String),
    /*
    MoveFieldUp
    MoveFieldDown

    RemoveField,
    DeleteField,


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
            console: ConsoleService::new(),
            state: State { // TODO : create a builder for this
                editing_mode: EditorMode::Listing,
                editing_enum_value: None,
                creating_enum_value: None,
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
                self.state.editing_mode = EditorMode::EditingField(
                    EditingField {
                        field: self.state.model.fields[index].clone(),
                        id: index
                    }
                );
                true
            }
            Msg::CancelEditField => {
                self.state.editing_mode = EditorMode::Listing;
                true
            }
            Msg::UpdateField => {
                match &self.state.editing_mode {
                    EditorMode::EditingField(editing_field) => {
                        self.state.model.fields[editing_field.id]  = editing_field.field.clone();
                        self.state.editing_mode = EditorMode::Listing;
                    }
                    _ => ()
                }
                true
            }
            Msg::UpdateFieldName(name) => {
                match &mut self.state.editing_mode {
                    EditorMode::EditingField(editing_field) => editing_field.field.name = name,
                    EditorMode::CreatingField(creating_field) => creating_field.name = name,
                    _ => ()
                };
                true
            }
            Msg::UpdateFieldLabel(label) => {
                match &mut self.state.editing_mode {
                    EditorMode::EditingField(editing_field) => editing_field.field.label = label,
                    EditorMode::CreatingField(creating_field) => creating_field.label = label,
                    _ => ()
                };
                true
            }
            Msg::UpdateFieldPlaceHolder(placeholder) => {
                match &mut self.state.editing_mode {
                    EditorMode::EditingField(editing_field) => editing_field.field.placeholder = placeholder,
                    EditorMode::CreatingField(creating_field) => creating_field.placeholder = placeholder,
                    _ => ()
                };
                true
            }
            Msg::UpdateFieldType(data_type) => {
                match &mut self.state.editing_mode {
                    EditorMode::EditingField(editing_field) => editing_field.field.data_type = data_type,
                    EditorMode::CreatingField(creating_field) => creating_field.data_type = data_type,
                    _ => ()
                };
                true
            }
            Msg::ToggleFieldRequired => {
                match &mut self.state.editing_mode {
                    EditorMode::EditingField(editing_field) => editing_field.field.required = !editing_field.field.required,
                    EditorMode::CreatingField(creating_field) => creating_field.required = !creating_field.required,
                    _ => ()
                };
                true
            }
            Msg::NewField => {
                self.state.editing_mode = EditorMode::CreatingField(
                    Field {
                        name: "".to_string(),
                        data_type: FieldDataType::Text,
                        label: "".to_string(),
                        placeholder: "".to_string(),
                        required: false,
                        validation: None,
                    }
                );
                true
            }
            Msg::CancelNewField => {
                self.state.editing_mode = EditorMode::Listing;
                true
            }
            Msg::CreateField => {
                match &mut self.state.editing_mode {
                    EditorMode::CreatingField(creating_field) => self.state.model.fields.push(creating_field.clone()),
                    _ => ()
                };
                self.state.editing_mode = EditorMode::Listing;
                true
            }
            Msg::EditFieldEnumValues(index) => {
                let edit_enum_value = |field: &Field, editing_enum_value: &mut Option<EditingEnumValue>| {
                    match &field.validation {
                        Some(validation) => {
                            match &validation.enum_values {
                                Some(enum_values) => {
                                    *editing_enum_value = Some(EditingEnumValue {
                                        index: index,
                                        enum_value: EnumValues {
                                            value: enum_values[index].value.to_string(), // TODO get that value form current field
                                            label: enum_values[index].label.to_string(),
                                        },
                                    });
                                },
                                None => (),
                            }
                        },
                        None => ()
                    }
                };
                match &mut self.state.editing_mode {
                    EditorMode::CreatingField(new_field) => {
                        edit_enum_value(new_field, &mut self.state.editing_enum_value)
                    },
                    EditorMode::EditingField(editing_field) => {
                        edit_enum_value(&editing_field.field, &mut self.state.editing_enum_value)
                    },
                    EditorMode::Listing => ()
                };
                self.console.log(format!("{:?}", self.state).as_str());
                true
            }
            Msg::CancelEditEnumValues => {
                self.state.editing_enum_value = None;
                true
            }
            Msg::NewEnumValue => {
                self.state.creating_enum_value = Some(EnumValues {
                    value: "".to_string(),
                    label: "".to_string(),
                });
                true
            }
            Msg::CancelNewEnumValue => {
                self.state.creating_enum_value = None;
                true
            },
            Msg::UpdateNewEnumValueValue(value) => {
                match &mut self.state.creating_enum_value {
                    Some(enum_value) => enum_value.value = value.clone(),
                    None => ()
                };
                true
            },
            Msg::UpdateNewEnumValueLabel(label) => {
                match &mut self.state.creating_enum_value {
                    Some(enum_value) => enum_value.label = label.clone(),
                    None => ()
                };
                true
            },
            Msg::UpdateEnumValueValue(value) => {
                match &mut self.state.editing_enum_value {
                    Some(editing_enum_value) => {
                        editing_enum_value.enum_value.value = value.clone()
                    },
                    None => ()
                };
                true
            }
            Msg::UpdateEnumValueLabel(label) => {
                match &mut self.state.editing_enum_value {
                    Some(editing_enum_value) => {
                        editing_enum_value.enum_value.label = label.clone()
                    },
                    None => ()
                };
                true
            }
            Msg::CreateNewEnumValue  => {
                let add_enum = |field: &mut Field, creating_enum_value: &EnumValues| {
                    match &mut field.validation {
                        Some(validation) => { // there is validation
                            match &mut validation.enum_values {
                                Some(enum_values) => {
                                    enum_values.push(creating_enum_value.clone());
                                },
                                None => { // there is no enum_values in validation
                                    validation.enum_values = Some(vec![creating_enum_value.clone()]);
                                }
                            }
                        },
                        None => {
                            field.validation = Some(Validation{
                                min_length: None,
                                max_length: None,
                                enum_values: Some(vec![creating_enum_value.clone()])
                            });
                            ()
                        }
                    }
                };
                match &mut self.state.creating_enum_value {
                    Some(creating_enum_value) => {
                        match &mut self.state.editing_mode {
                            EditorMode::EditingField(editing_field) => {
                                add_enum(&mut editing_field.field, creating_enum_value);
                            }
                            EditorMode::CreatingField(creating_field) => {
                                add_enum(creating_field, creating_enum_value)
                            },
                            _ => ()
                        }
                    },
                    None => ()
                }
                self.state.creating_enum_value = None;
                self.console.log(format!("{:?}", self.state).as_str());
                true
            }
            Msg::UpdateFieldEnumValues => {
                let update_enum_values = |field: &mut Field, editing_enum_value: & Option<EditingEnumValue>| {
                    match &mut field.validation {
                        Some(validation) => { // there is validation
                            match &mut validation.enum_values {
                                Some(enum_values) => {
                                    match editing_enum_value {
                                        Some(editing_enum_value) => { // we are also editing an enum value
                                            enum_values[editing_enum_value.index] = editing_enum_value.enum_value.clone()
                                        },
                                        None => ()
                                    }
                                },
                                None => ()
                            }
                        },
                        None => ()
                    }
                };

                match &mut self.state.editing_mode {
                    EditorMode::EditingField(editing_field) => {
                        update_enum_values(&mut editing_field.field, &self.state.editing_enum_value)
                    },
                    EditorMode::CreatingField(creating_field) => {
                        update_enum_values(creating_field, &self.state.editing_enum_value)
                    },
                    _ => ()
                }
                self.state.editing_enum_value = None;
                self.console.log(format!("{:?}", self.state).as_str());
                true
            }
            Msg::DeleteEnumValue(index) => {
                let remove_enum = |field: &mut Field, index: usize| {
                    match &mut field.validation {
                        Some(validation) => {
                            match &mut validation.enum_values {
                                Some(enum_values) => {enum_values.remove(index);},
                                None => ()
                            }
                        }
                        None => (),
                    }
                };

                match &mut self.state.editing_mode {
                    EditorMode::CreatingField(creating_field) => {
                        remove_enum(creating_field, index)
                    },
                    EditorMode::EditingField(editing_field) => {
                        remove_enum(&mut editing_field.field, index)
                    },
                    _ => (),
                }
                true
            }
        }
    }

    fn view(&self) -> Html {
        let model_title: String = if let Some(model_title) = &self.state.model.title { model_title.clone() } else { "".to_string() };
        let model_subtitle: String = if let Some(model_subtitle) = &self.state.model.subtitle { model_subtitle.clone() } else { "".to_string() };
        let editing_view = self.view_editing_field();
        let creating_view = self.view_new_field();
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
                    <button onclick=self.link.callback(|_| Msg::NewField)>{"New field"}</button>
                    { editing_view }
                    { creating_view}
                    <div class="model-field-container">
                        <ul>
                        { for self.state.model.fields.iter().enumerate().map(|(index, field)| self.view_field_item(index, field)) }
                        </ul>
                    </div>
                </div>
            </div>
        }
    }
}

impl App {
    fn view_field_item(&self, index: usize, field: &Field) -> Html {
        view_field_item(&self, index, field)
    }

    fn view_editing_enum_value(&self) -> Html {
        match &self.state.editing_enum_value {
            Some(editing_enum_value) => {
                view_editing_enum_value(&self, editing_enum_value)
            }
            None => html!{}
        }
    }

    fn view_enum_values_list(&self) -> Html {
        let view = |field: &Field| {
            let fields = match &field.validation {
                Some(validation) => {
                    match &validation.enum_values {
                        Some(enum_values) => {
                            html! {
                                <ul>
                                { for enum_values
                                    .iter()
                                    .enumerate()
                                    .map(|(index, enum_value)|
                                        view_enum_values_item(&self, index, enum_value)
                                    )
                                }
                                </ul>
                            }
                        },
                        None => html!{}
                    }
                },
                None => html!{}
            };
            html!{
                <div>
                    <h3>{"Enum values"}</h3>
                    {self.view_editing_enum_value()}
                    {fields}
                </div>
            }
        };

        match &self.state.editing_mode {
            EditorMode::CreatingField(creating_field) => {
                view(creating_field)
            },
            EditorMode::EditingField(editing_field) => {
                view(&editing_field.field)
            },
            _ => html!{}
        }
    }

    fn view_creating_enum_values(&self) -> Html {
        match &self.state.creating_enum_value {
            Some(creating_enum_value) => view_creating_enum_values(&self, creating_enum_value),
            None => html!{}
        }
    }

    fn view_new_field(&self) -> Html {
        match &self.state.editing_mode {
            EditorMode::CreatingField(creating_field) => {
                let creating_enum_values_view = self.view_creating_enum_values();
                let create_enum_btn = match &creating_field.data_type {
                    Radio => {
                        match self.state.creating_enum_value {
                            Some(_) => html!{{self.view_enum_values_list()}},
                            None => html!{
                                <div>
                                    <button onclick=self.link.callback(|_| Msg::NewEnumValue)>
                                        {"create enum values"}
                                    </button>
                                    {self.view_enum_values_list()}
                                </div>
                            },
                        }
                    }
                    _ => html!{}
                };
                html! {
                    <div class="model-field-editing">
                        <h3>{"New field: "}{&creating_field.name}</h3>
                        {"Name:"}
                        <input
                            type="text"
                            value={&creating_field.name}
                            oninput=self.link.callback(move |input: InputData|
                                {
                                    Msg::UpdateFieldName(input.value)
                                })
                        />

                        {"Type:"} {self.view_select_type(creating_field.data_type.clone())}<br/>
                        {"Label:"}
                        <input
                            type="text"
                            value={&creating_field.label}
                            oninput=self.link.callback(move |input: InputData|
                                {
                                    Msg::UpdateFieldLabel(input.value)
                                })
                        />
                        {"Placeholder:"} <input
                            type="text"
                            value={&creating_field.placeholder}
                            oninput=self.link.callback(move |input: InputData|
                                {
                                    Msg::UpdateFieldPlaceHolder(input.value)
                                })
                        /><br/>
                        {"Required :"} <input type="checkbox" onclick=self.link.callback(|_| Msg::ToggleFieldRequired) name="required" checked=creating_field.required />
                        <button onclick=self.link.callback(|_| Msg::CancelNewField)>{"cancel"}</button>
                        <button onclick=self.link.callback(|_| Msg::CreateField)>{"save"}</button>
                        <br/>{creating_enum_values_view}
                        {create_enum_btn}
                    </div>
                }
            },
            _ => html!{}
        }
    }

    fn view_editing_field(&self) -> Html {
        match &self.state.editing_mode {
            EditorMode::EditingField(editing_field) => {
                let field = &editing_field.field;
                let creating_enum_values_view = self.view_creating_enum_values();
                let create_enum_btn = match editing_field.field.data_type {
                    Radio => {
                        match self.state.creating_enum_value {
                            Some(_) => html!{{self.view_enum_values_list()}},
                            None => html!{
                                <div>
                                    <button onclick=self.link.callback(|_| Msg::NewEnumValue)>
                                        {"create enum values"}
                                    </button>
                                    {self.view_enum_values_list()}
                                </div>
                            },
                        }
                    }
                    _ => html!{}
                };
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
                    {"Required :"} <input type="checkbox" onclick=self.link.callback(|_| Msg::ToggleFieldRequired) name="required" checked=field.required />
                    <button onclick=self.link.callback(|_| Msg::CancelEditField)>{"cancel"}</button>
                    <button onclick=self.link.callback(|_| Msg::UpdateField)>{"save"}</button>
                    <br/>{creating_enum_values_view}
                    {create_enum_btn}
                </div>
            }},
            _ => html! {}
        }
    }

    // TODO pass on update callback as a function (so it can be used by Edit & create
    fn view_select_type(&self, field_type: FieldDataType) -> Html {
        html! {
            <Select<FieldDataType>
                selected=Some(field_type)
                options=FieldDataType::iter().collect::<Vec<_>>()
                onchange=self.link.callback(Msg::UpdateFieldType) />
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
