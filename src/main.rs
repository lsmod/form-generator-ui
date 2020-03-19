#![recursion_limit="512"]
extern crate strum;
#[macro_use]
extern crate strum_macros;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

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

struct EditingEnumValue {
    index: usize,
    enum_value: EnumValues,
}

struct EditingField {
    id: usize,
    field: Field
}

// TODO move into separated module
struct App {
    link: ComponentLink<App>,
    state: State,
    console: ConsoleService,
}

pub struct State {
    editing_field: Option<EditingField>,
    editing_enum_value: Option<EditingEnumValue>,

    new_field: Field, // TODO move this inside it's own struct
    creating_field: bool, // TODO move this inside it's own struct

    creating_enum_value: Option<EnumValues>,
    model: Model,
}
// TODO:
// - separator

// - handle list, radio, (anything with multiple value possible)
// -> use validation -> enum_values
// -> display a list with add/edit/remove/move up/move down
// -> enum_values should be a tuple of string: 1 value 2 the label

// - think about validation...
// -> create error Struct with message
// -> call validation before exporting (look for all empty string)

// - camelCase, snake_case
// - move into a separated module
// - divide into sub Component (edit field, new field, header)
enum Msg {
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

    NewField, // TODO display enum_value if type = radio
    CancelNewField,
    CreateField,
    UpdateNewFieldName(String),
    UpdateNewFieldLabel(String),
    UpdateNewFieldPlaceHolder(String),
    UpdateNewFieldType(FieldDataType),
    ToggleNewFieldRequired,

    EditFieldEnumValues(usize), // click edit on enum value
    UpdateFieldEnumValues, // click on save (editing enum value)
    CancelEditEnumValues,
    UpdateEnumValueValue(String),
    UpdateEnumValueLabel(String),
    DeleteEnumValue(usize),
    NewEnumValue, // click new enum value
    CreateNewEnumValue, // click save new enum value
    CancelNewEnumValue,
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
                editing_field: None,
                creating_field: false,
                new_field: Field {
                    name: "".to_string(),
                    data_type: FieldDataType::Text,
                    label: "".to_string(),
                    placeholder: "".to_string(),
                    required: false,
                    validation: None,
                },
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
                self.state.editing_field = Some(EditingField {
                    field: self.state.model.fields[index].clone(),
                    id: index
                });
                true
            }
            Msg::CancelEditField => {
                self.state.editing_field = None;
                true
            }
            Msg::UpdateField => {
                match &self.state.editing_field {
                    Some(editing_field) => {
                        self.state.model.fields[editing_field.id]  = editing_field.field.clone();
                        self.state.editing_field = None;
                    }
                    None => ()
                }
                true
            }
            Msg::UpdateFieldName(name) => {
                self.console.log("toto");
                match &mut self.state.editing_field {
                    Some(editing_field) => editing_field.field.name = name,
                    None => ()
                };
                true
            }
            Msg::UpdateFieldLabel(label) => {
                match &mut self.state.editing_field {
                    Some(editing_field) => editing_field.field.label = label,
                    None => ()
                };
                true
            }
            Msg::UpdateFieldPlaceHolder(placeholder) => {
                match &mut self.state.editing_field {
                    Some(editing_field) => editing_field.field.placeholder = placeholder,
                    None => ()
                };
                true
            }
            Msg::UpdateFieldType(data_type) => {
                match &mut self.state.editing_field {
                    Some(editing_field) => editing_field.field.data_type = data_type,
                    None => ()
                };
                true
            }
            Msg::ToggleFieldRequired => {
                match &mut self.state.editing_field {
                    Some(editing_field) => editing_field.field.required = !editing_field.field.required,
                    None => ()
                };
                true
            }
            Msg::NewField => {
                self.state.creating_field = true;
                self.state.new_field = Field {
                    name: "".to_string(),
                    data_type: FieldDataType::Text,
                    label: "".to_string(),
                    placeholder: "".to_string(),
                    required: false,
                    validation: None,
                };
                true
            }
            Msg::CancelNewField => {
                self.state.creating_field = false;
                true
            }
            Msg::CreateField => {
                self.state.creating_field = false;
                self.state.model.fields.push(self.state.new_field.clone());
                true
            }
            Msg::UpdateNewFieldName(name) => {
                self.state.new_field.name = name;
                true
            }
            Msg::UpdateNewFieldLabel(label) => {
                self.state.new_field.label = label;
                true
            }
            Msg::UpdateNewFieldPlaceHolder(placeholder) => {
                self.state.new_field.placeholder = placeholder;
                true
            }
            Msg::UpdateNewFieldType(data_type) => {
                self.state.new_field.data_type = data_type;
                true
            }
            Msg::ToggleNewFieldRequired => {
                self.state.new_field.required = !self.state.new_field.required;
                true
            }
            Msg::EditFieldEnumValues(index) => {
                // TODO: check if editing_field got validation -> create it otherwise
                self.state.editing_enum_value = Some(EditingEnumValue {
                    index: index,
                    enum_value: EnumValues {
                        value: "".to_string(), // TODO get that value form current field
                        label: "".to_string(),
                    },
                });
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
                // TODO match on self.state.creating_enum_value
                match &mut self.state.editing_field {
                    Some(editing_field) => { // we are editing a field
                        match &mut editing_field.field.validation {
                            Some(validation) => { // there is validation
                                match &mut validation.enum_values {
                                    Some(enum_values) => {
                                        match &mut self.state.editing_enum_value {
                                            Some(editing_enum_value) => { // we are also editing an enum value
                                                enum_values.push(editing_enum_value.enum_value.clone())
                                            },
                                            None => ()
                                        }
                                    },
                                    None => ()
                                }
                            },
                            None => {
                                editing_field.field.validation = Some(Validation{
                                    min_length: None,
                                    max_length: None,
                                    enum_values: Some(vec![self.state.creating_enum_value])
                                });
                                ()
                            } // TODO add validation
                        }
                    },
                    None => () // TODO check if we are creating a field
                }
                // TODO check if we are editing or creating
                // TODO check if field has validation ? create empty vec
                // TODO add validation to field
                // TODO add corresponding enum value
                true
            }
            Msg::UpdateFieldEnumValues => {
                match &mut self.state.editing_field {
                    Some(editing_field) => { // we are editing a field
                        match &mut editing_field.field.validation {
                            Some(validation) => { // there is validation
                                match &mut validation.enum_values {
                                    Some(enum_values) => {
                                        match &mut self.state.editing_enum_value {
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
                    },
                    None => ()
                }
                true
            }
            Msg::DeleteEnumValue(index) => {
                // TODO check if we are editing or creating
                // TODO check if field has validation ? none -> do nothing
                // TODO if validation .length = 0 valdiation = None
                true
            }
            // Msg::UpdateNewFieldEnumValueValue(index, value) => {
            //     match &mut self.state.new_field.validation {
            //         Some(validation) => {
            //             match &mut validation.enum_values {
            //                 Some(values) => {
            //                     values[index] = EnumValues {
            //                         value: value,
            //                         label: values[index].value.clone()
            //                     }
            //                 },
            //                 None => ()
            //             }
            //         },
            //         None => ()
            //     };
            //     true
            // }
            // Msg::UpdateNewFieldEnumValueLabel(index, label) => {
            //     match &mut self.state.new_field.validation {
            //         Some(validation) => {
            //             match &mut validation.enum_values {
            //                 Some(values) => {
            //                     values[index] = EnumValues {
            //                         label: label,
            //                         value: values[index].value.clone()
            //                     };
            //                 },
            //                 None => ()
            //             }
            //         },
            //         None => ()
            //     };
            //     true
            // }
            // Msg::DeleteNewFieldEnumValue(index) => {
            //     match &mut self.state.new_field.validation {
            //         Some(validation) => {
            //             match &mut validation.enum_values {
            //                 Some(values) => {
            //                     values.remove(index);
            //                 },
            //                 None => ()
            //             }
            //         },
            //         None => ()
            //     };
            //     true
            // }
            // Msg::NewNewFieldEnumValue() => {
            //     true
            // }
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
        let field = &self.state.new_field;
        if self.state.creating_field {
            html! {
                <div class="model-field-editing">
                    <h3>{"New field: "}{&field.name}</h3>
                    {"Name:"}
                    <input
                        type="text"
                        value={&field.name}
                        oninput=self.link.callback(move |input: InputData|
                            {
                                Msg::UpdateNewFieldName(input.value)
                            })
                    />

                    {"Type:"} {self.view_select_type(field.data_type.clone(), true)}<br/>
                    {"Label:"}
                    <input
                        type="text"
                        value={&field.label}
                        oninput=self.link.callback(move |input: InputData|
                            {
                                Msg::UpdateNewFieldLabel(input.value)
                            })
                    />
                    {"Placeholder:"} <input
                        type="text"
                        value={&field.placeholder}
                        oninput=self.link.callback(move |input: InputData|
                            {
                                Msg::UpdateNewFieldPlaceHolder(input.value)
                            })
                    /><br/>
                    {"Required :"} <input type="checkbox" onclick=self.link.callback(|_| Msg::ToggleNewFieldRequired) name="required" checked=field.required />
                    <button onclick=self.link.callback(|_| Msg::CancelNewField)>{"cancel"}</button>
                    <button onclick=self.link.callback(|_| Msg::CreateField)>{"save"}</button>
                </div>
            }
        }
        else {
            html!{}
        }
    }

    fn view_editing_field(&self) -> Html {
        match &self.state.editing_field {
            Some(editing_field) => {
                let field = &editing_field.field;
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
                    {"Type:"} {self.view_select_type(field.data_type.clone(), false)}<br/>
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
                </div>
            }},
            None => html! {}
        }
    }

    // TODO pass on update callback as a function (so it can be used by Edit & create
    fn view_select_type(&self, field_type: FieldDataType, creation_mode: bool) -> Html {
        let callback = if creation_mode { Msg::UpdateNewFieldType } else { Msg::UpdateFieldType};
        html! {
            <Select<FieldDataType>
                selected=Some(field_type)
                options=FieldDataType::iter().collect::<Vec<_>>()
                onchange=self.link.callback(callback) />
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
