#![recursion_limit = "1024"]
extern crate strum;
#[macro_use]
extern crate strum_macros;
#[macro_use]
extern crate serde_derive;
extern crate askama;
extern crate inflector;
extern crate serde;
extern crate serde_json;

mod spectre_editor_views;
use crate::spectre_editor_views::main_view;
use crate::spectre_editor_views::view_create_enum_btn_container;
use crate::spectre_editor_views::view_creating_enum_values;
use crate::spectre_editor_views::view_edit_field_container;
use crate::spectre_editor_views::view_edit_model_view;
use crate::spectre_editor_views::view_editing_enum_value;
use crate::spectre_editor_views::view_enum_values_container;
use crate::spectre_editor_views::view_enum_values_list_container;
use crate::spectre_editor_views::view_field_type_select;
use crate::spectre_editor_views::view_new_field_container;
use askama::Template;

use yew::services::ConsoleService;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

mod form_model;
pub use form_model::form_model::FieldDataType::*;
pub use form_model::form_model::*;

mod react_native_formik;
pub use react_native_formik::form_template::FormTemplate;

#[derive(Debug)]
pub struct EditingEnumValue {
    index: usize,
    enum_value: EnumValues,
}

#[derive(Default, Debug)]
pub struct GeneratedFile {
    file_name: String,
    language: &'static str,
    content: String,
}

#[derive(Debug)]
struct EditingField {
    id: usize,
    field: Field,
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
    generated_files: Vec<GeneratedFile>,
    selected_file: usize,
}

impl Default for State {
    fn default() -> Self {
        State {
            editing_mode: EditorMode::Listing,
            editing_enum_value: None,
            creating_enum_value: None,
            generated_files: vec![],
            selected_file: 0,
            model: Model {
                name: "profile".to_string(),
                title: Some("Édition du profil".to_string()),
                subtitle: None,
                submit_label: "Mettre à jour".to_string(),
                fields: vec![
                    Field {
                        data_type: FieldDataType::Text,
                        name: "nickname".to_string(),
                        label: "Your nickname".to_string(),
                        placeholder: "Enter you nickname".to_string(),
                        validation: Some(Validation {
                            min_length: Some(6),
                            max_length: Some(30),
                            enum_values: None,
                        }),
                        required: true,
                    },
                    Field {
                        data_type: FieldDataType::Email,
                        name: "email".to_string(),
                        label: "Your email".to_string(),
                        placeholder: "Enter you email".to_string(),
                        validation: None,
                        required: true,
                    },
                    Field {
                        data_type: FieldDataType::Radio,
                        name: "sex".to_string(),
                        label: "Your gender".to_string(),
                        placeholder: "Select your gender".to_string(),
                        validation: Some(Validation {
                            min_length: None,
                            max_length: None,
                            enum_values: Some(vec![
                                EnumValues {
                                    value: "M".to_string(),
                                    label: "Male".to_string(),
                                },
                                EnumValues {
                                    value: "F".to_string(),
                                    label: "Female".to_string(),
                                },
                            ]),
                        }),
                        required: true,
                    },
                ],
            },
        }
    }
}
// TODO:
// bug: click on edit enum value -> show edit then redirect to home without saving
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
#[derive(Clone)]
pub enum Msg {
    UpdateName(String),
    UpdateTitle(String),
    UpdateSubtitle(String),
    UpdateSubmitLabel(String),

    EditField(usize),
    DeleteField(usize),
    MoveFieldUp(usize),
    MoveFieldDown(usize),
    CancelEditField,
    UpdateField,

    UpdateFieldName(String),
    UpdateFieldLabel(String),
    UpdateFieldPlaceHolder(String),
    UpdateFieldType(FieldDataType),
    ToggleFieldRequired,
    ToogleMinLength,
    ToogleMaxLength,
    UpdateMinLength(usize),
    UpdateMaxLength(usize),

    NewField,
    CancelNewField,
    CreateField,

    EditFieldEnumValues(usize), // click edit on enum value
    UpdateFieldEnumValues,      // click on save (editing enum value)
    CancelEditEnumValues,
    UpdateEnumValueValue(String),
    UpdateEnumValueLabel(String),
    DeleteEnumValue(usize),
    NewEnumValue, // click new enum value
    CancelNewEnumValue,
    CreateNewEnumValue,              // click save new enum value
    UpdateNewEnumValueValue(String), // this is when adding a new field
    UpdateNewEnumValueLabel(String),
    MoveEnumValueUp(usize),
    MoveEnumValueDown(usize),

    NewGenerate,
    SelectFile(usize),
    /*

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
        App {
            link,
            console: ConsoleService::new(),
            state: State::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateName(name) => {
                self.state.model.name = name;
                true
            }
            Msg::UpdateTitle(title) => {
                self.state.model.title = Some(title);
                true
            }
            Msg::UpdateSubtitle(subtitle) => {
                self.state.model.subtitle = Some(subtitle);
                true
            }
            Msg::UpdateSubmitLabel(label) => {
                self.state.model.submit_label = label;
                true
            }
            Msg::ToogleMinLength  => {
                match &mut self.state.editing_mode {
                    EditorMode::CreatingField(field) => {
                        match &mut field.validation {
                            Some(validation) => {
                                match validation.min_length {
                                    Some(_) => validation.min_length = None,
                                    None => validation.min_length = Some(8)
                                }
                            },
                            None => field.validation = Some(Validation {
                                min_length: Some(8),
                                max_length: None,
                                enum_values: None,
                            })
                        }
                    },
                    EditorMode::EditingField(field)=> {
                        match &mut field.field.validation {
                            Some(validation) => {
                                match validation.min_length {
                                    Some(_) => validation.min_length = None,
                                    None => validation.min_length = Some(8)
                                }
                            },
                            None => field.field.validation = Some(Validation {
                                min_length: Some(8),
                                max_length: None,
                                enum_values: None,
                            })
                        }
                    },
                    _ => ()
                }
                true
            }
            Msg::ToogleMaxLength => {
                match &mut self.state.editing_mode {
                    EditorMode::CreatingField(field) => {
                        match &mut field.validation {
                            Some(validation) => {
                                match validation.max_length {
                                    Some(_) => validation.max_length = None,
                                    None => validation.max_length = Some(30)
                                }
                            },
                            None => field.validation = Some(Validation {
                                min_length: None,
                                max_length: Some(30),
                                enum_values: None,
                            })
                        }
                    },
                    EditorMode::EditingField(field)=> {
                        match &mut field.field.validation {
                            Some(validation) => {
                                match validation.max_length {
                                    Some(_) => validation.max_length = None,
                                    None => validation.max_length = Some(30)
                                }
                            },
                            None => field.field.validation = Some(Validation {
                                min_length: None,
                                max_length: Some(30),
                                enum_values: None,
                            })
                        }
                    },
                    _ => ()
                }
                true
            }
            Msg::UpdateMinLength(length) => {
                match &mut self.state.editing_mode {
                    EditorMode::CreatingField(field) => {
                        match &mut field.validation {
                            Some(validation) => {
                                match validation.min_length {
                                    Some(_) => validation.min_length = Some(length),
                                    None => ()
                                }
                            },
                            None => ()
                        }
                    },
                    EditorMode::EditingField(field)=> {
                        match &mut field.field.validation {
                            Some(validation) => {
                                match validation.min_length {
                                    Some(_) => validation.min_length = Some(length),
                                    None => ()
                                }
                            },
                            None => ()
                        }
                    },
                    _ => ()
                }
                true
            }
            Msg::UpdateMaxLength(length) => {
                match &mut self.state.editing_mode {
                    EditorMode::CreatingField(field) => {
                        match &mut field.validation {
                            Some(validation) => {
                                match validation.max_length {
                                    Some(_) => validation.max_length = Some(length),
                                    None => ()
                                }
                            },
                            None => ()
                        }
                    },
                    EditorMode::EditingField(field)=> {
                        match &mut field.field.validation {
                            Some(validation) => {
                                match validation.max_length {
                                    Some(_) => validation.max_length = Some(length),
                                    None => ()
                                }
                            },
                            None => ()
                        }
                    },
                    _ => ()
                }
                true
            }
            Msg::SelectFile(index) => {
                self.state.selected_file = index;
                true
            },
            Msg::EditField(index) => {
                self.state.editing_mode = EditorMode::EditingField(EditingField {
                    field: self.state.model.fields[index].clone(),
                    id: index,
                });
                true
            }
            Msg::DeleteField(index) => {
                if index < self.state.model.fields.len() {
                    self.state.model.fields.remove(index);
                }
                true
            }
            Msg::MoveFieldUp(index) => {
                if index > 0 {
                    self.state.model.fields.swap(index, index - 1);
                }
                true
            }
            Msg::MoveFieldDown(index) => {
                if index < &self.state.model.fields.len() - 1 {
                    self.state.model.fields.swap(index, index + 1);
                }
                true
            }
            Msg::CancelEditField => {
                self.state.editing_mode = EditorMode::Listing;
                true
            }
            Msg::UpdateField => {
                match &self.state.editing_mode {
                    EditorMode::EditingField(editing_field) => {
                        self.state.model.fields[editing_field.id] = editing_field.field.clone();
                        self.state.editing_mode = EditorMode::Listing;
                    }
                    _ => (),
                }
                true
            }
            Msg::UpdateFieldName(name) => {
                match &mut self.state.editing_mode {
                    EditorMode::EditingField(editing_field) => editing_field.field.name = name,
                    EditorMode::CreatingField(creating_field) => creating_field.name = name,
                    _ => (),
                };
                true
            }
            Msg::UpdateFieldLabel(label) => {
                match &mut self.state.editing_mode {
                    EditorMode::EditingField(editing_field) => editing_field.field.label = label,
                    EditorMode::CreatingField(creating_field) => creating_field.label = label,
                    _ => (),
                };
                true
            }
            Msg::UpdateFieldPlaceHolder(placeholder) => {
                match &mut self.state.editing_mode {
                    EditorMode::EditingField(editing_field) => {
                        editing_field.field.placeholder = placeholder
                    }
                    EditorMode::CreatingField(creating_field) => {
                        creating_field.placeholder = placeholder
                    }
                    _ => (),
                };
                true
            }
            // TODO: remove validation when updating
            Msg::UpdateFieldType(data_type) => {
                match &mut self.state.editing_mode {
                    EditorMode::EditingField(editing_field) => {
                        editing_field.field.data_type = data_type
                    }
                    EditorMode::CreatingField(creating_field) => {
                        creating_field.data_type = data_type
                    }
                    _ => (),
                };
                true
            }
            Msg::ToggleFieldRequired => {
                match &mut self.state.editing_mode {
                    EditorMode::EditingField(editing_field) => {
                        editing_field.field.required = !editing_field.field.required
                    }
                    EditorMode::CreatingField(creating_field) => {
                        creating_field.required = !creating_field.required
                    }
                    _ => (),
                };
                true
            }
            Msg::NewField => {
                self.state.editing_mode = EditorMode::CreatingField(Field {
                    name: "".to_string(),
                    data_type: FieldDataType::Text,
                    label: "".to_string(),
                    placeholder: "".to_string(),
                    required: false,
                    validation: None,
                });
                true
            }
            Msg::CancelNewField => {
                self.state.editing_mode = EditorMode::Listing;
                true
            }
            Msg::CreateField => {
                match &mut self.state.editing_mode {
                    EditorMode::CreatingField(creating_field) => {
                        self.state.model.fields.push(creating_field.clone())
                    }
                    _ => (),
                };
                self.state.editing_mode = EditorMode::Listing;
                true
            }
            Msg::EditFieldEnumValues(index) => {
                let edit_enum_value =
                    |field: &Field, editing_enum_value: &mut Option<EditingEnumValue>| {
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
                                    }
                                    None => (),
                                }
                            }
                            None => (),
                        }
                    };
                match &mut self.state.editing_mode {
                    EditorMode::CreatingField(new_field) => {
                        edit_enum_value(new_field, &mut self.state.editing_enum_value)
                    }
                    EditorMode::EditingField(editing_field) => {
                        edit_enum_value(&editing_field.field, &mut self.state.editing_enum_value)
                    }
                    EditorMode::Listing => (),
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
            }
            Msg::UpdateNewEnumValueValue(value) => {
                match &mut self.state.creating_enum_value {
                    Some(enum_value) => enum_value.value = value.clone(),
                    None => (),
                };
                true
            }
            Msg::UpdateNewEnumValueLabel(label) => {
                match &mut self.state.creating_enum_value {
                    Some(enum_value) => enum_value.label = label.clone(),
                    None => (),
                };
                true
            }
            Msg::UpdateEnumValueValue(value) => {
                match &mut self.state.editing_enum_value {
                    Some(editing_enum_value) => editing_enum_value.enum_value.value = value.clone(),
                    None => (),
                };
                true
            }
            Msg::UpdateEnumValueLabel(label) => {
                match &mut self.state.editing_enum_value {
                    Some(editing_enum_value) => editing_enum_value.enum_value.label = label.clone(),
                    None => (),
                };
                true
            }
            Msg::CreateNewEnumValue => {
                let add_enum = |field: &mut Field, creating_enum_value: &EnumValues| {
                    match &mut field.validation {
                        Some(validation) => {
                            // there is validation
                            match &mut validation.enum_values {
                                Some(enum_values) => {
                                    enum_values.push(creating_enum_value.clone());
                                }
                                None => {
                                    // there is no enum_values in validation
                                    validation.enum_values =
                                        Some(vec![creating_enum_value.clone()]);
                                }
                            }
                        }
                        None => {
                            field.validation = Some(Validation {
                                min_length: None,
                                max_length: None,
                                enum_values: Some(vec![creating_enum_value.clone()]),
                            });
                            ()
                        }
                    }
                };
                match &mut self.state.creating_enum_value {
                    Some(creating_enum_value) => match &mut self.state.editing_mode {
                        EditorMode::EditingField(editing_field) => {
                            add_enum(&mut editing_field.field, creating_enum_value);
                        }
                        EditorMode::CreatingField(creating_field) => {
                            add_enum(creating_field, creating_enum_value)
                        }
                        _ => (),
                    },
                    None => (),
                }
                self.state.creating_enum_value = None;
                self.console.log(format!("{:?}", self.state).as_str());
                true
            }
            Msg::UpdateFieldEnumValues => {
                let update_enum_values =
                    |field: &mut Field, editing_enum_value: &Option<EditingEnumValue>| {
                        match &mut field.validation {
                            Some(validation) => {
                                // there is validation
                                match &mut validation.enum_values {
                                    Some(enum_values) => {
                                        match editing_enum_value {
                                            Some(editing_enum_value) => {
                                                // we are also editing an enum value
                                                enum_values[editing_enum_value.index] =
                                                    editing_enum_value.enum_value.clone()
                                            }
                                            None => (),
                                        }
                                    }
                                    None => (),
                                }
                            }
                            None => (),
                        }
                    };

                match &mut self.state.editing_mode {
                    EditorMode::EditingField(editing_field) => {
                        update_enum_values(&mut editing_field.field, &self.state.editing_enum_value)
                    }
                    EditorMode::CreatingField(creating_field) => {
                        update_enum_values(creating_field, &self.state.editing_enum_value)
                    }
                    _ => (),
                }
                self.state.editing_enum_value = None;
                self.console.log(format!("{:?}", self.state).as_str());
                true
            }
            // TODO remove validation if 1 item in list
            Msg::DeleteEnumValue(index) => {
                let remove_enum = |field: &mut Field, index: usize| match &mut field.validation {
                    Some(validation) => match &mut validation.enum_values {
                        Some(enum_values) => {
                            enum_values.remove(index);
                        }
                        None => (),
                    },
                    None => (),
                };

                match &mut self.state.editing_mode {
                    EditorMode::CreatingField(creating_field) => remove_enum(creating_field, index),
                    EditorMode::EditingField(editing_field) => {
                        remove_enum(&mut editing_field.field, index)
                    }
                    _ => (),
                }
                true
            }
            Msg::MoveEnumValueUp(index) => {
                let move_up = |field: &mut Field, index: usize| {
                    if index > 0 {
                        match &mut field.validation {
                            Some(validation) => match &mut validation.enum_values {
                                Some(enum_values) => {
                                    if enum_values.len() > 1 {
                                        enum_values.swap(index, index - 1);
                                    }
                                }
                                None => (),
                            },
                            None => (),
                        }
                    }
                };

                match &mut self.state.editing_mode {
                    EditorMode::CreatingField(creating_field) => move_up(creating_field, index),
                    EditorMode::EditingField(editing_field) => {
                        move_up(&mut editing_field.field, index)
                    }
                    _ => (),
                }
                true
            }
            Msg::MoveEnumValueDown(index) => {
                let move_down = |field: &mut Field, index: usize| match &mut field.validation {
                    Some(validation) => match &mut validation.enum_values {
                        Some(enum_values) => {
                            if index < enum_values.len() - 1 && enum_values.len() > 1 {
                                enum_values.swap(index, index + 1);
                            }
                        }
                        None => (),
                    },
                    None => (),
                };

                match &mut self.state.editing_mode {
                    EditorMode::CreatingField(creating_field) => move_down(creating_field, index),
                    EditorMode::EditingField(editing_field) => {
                        move_down(&mut editing_field.field, index)
                    }
                    _ => (),
                }
                true
            }
            Msg::NewGenerate => {
                let form_template = FormTemplate::from(&self.state.model);
                self.state.generated_files = vec![];
                self.state.generated_files.push(GeneratedFile {
                    file_name: "form.html".to_string(),
                    language: "JSX",
                    content: form_template.render().unwrap(),
                });
                self.console.log(
                    format!(
                        "file: {}\ncontent: \n{}",
                        self.state.generated_files[0].file_name,
                        self.state.generated_files[0].content
                    )
                    .as_str(),
                );
                true
            }
        }
    }

    fn view(&self) -> Html {
        let top_view = match &self.state.editing_mode {
            EditorMode::CreatingField(_) => self.view_new_field(),
            EditorMode::EditingField(_) => self.view_editing_field(),
            EditorMode::Listing => view_edit_model_view(&self.link, &self.state.model),
        };
        main_view(&self.link, &self.state.model, &self.state.generated_files, self.state.selected_file, top_view)
    }
}

impl App {
    fn view_editing_enum_value(&self) -> Html {
        match &self.state.editing_enum_value {
            Some(editing_enum_value) => view_editing_enum_value(&self.link, editing_enum_value),
            None => html! {},
        }
    }

    fn view_enum_values_list(&self) -> Html {
        let view = |field: &Field| match &field.validation {
            Some(validation) => match &validation.enum_values {
                Some(enum_values) => view_enum_values_container(view_enum_values_list_container(
                    &self.link,
                    enum_values,
                )),
                None => html! {},
            },
            None => html! {},
        };

        match &self.state.editing_mode {
            EditorMode::CreatingField(creating_field) => view(creating_field),
            EditorMode::EditingField(editing_field) => view(&editing_field.field),
            _ => html! {},
        }
    }

    fn view_creating_enum_values(&self) -> Html {
        match &self.state.creating_enum_value {
            Some(creating_enum_value) => view_creating_enum_values(&self.link, creating_enum_value),
            None => html! {},
        }
    }

    fn view_new_field(&self) -> Html {
        match &self.state.editing_mode {
            EditorMode::CreatingField(creating_field) => {
                let creating_enum_values_view = self.view_creating_enum_values();
                let editing_enum_values_view = self.view_editing_enum_value();
                let enum_values_list_view = self.view_enum_values_list();
                let create_enum_btn_view = match &creating_field.data_type {
                    Checkbox
                    | SelectList
                    | EditableSelectList
                    | MultiSelectList
                    | EditableMultiSelectList
                    | Radio => match self.state.creating_enum_value {
                        Some(_) => html! {},
                        None => view_create_enum_btn_container(&self.link),
                    },
                    _ => html! {},
                };
                view_new_field_container(
                    &self.link,
                    &creating_field,
                    enum_values_list_view,
                    creating_enum_values_view,
                    create_enum_btn_view,
                    editing_enum_values_view,
                    self.view_field_type_select(creating_field.data_type.clone()),
                )
            }
            _ => html! {},
        }
    }

    fn view_editing_field(&self) -> Html {
        match &self.state.editing_mode {
            EditorMode::EditingField(editing_field) => {
                let field = &editing_field.field;
                let creating_enum_values_view = self.view_creating_enum_values();
                let editing_enum_values_view = self.view_editing_enum_value();
                let enum_values_list_view = self.view_enum_values_list();
                let create_enum_btn_view = match editing_field.field.data_type {
                    Checkbox
                    | SelectList
                    | EditableSelectList
                    | MultiSelectList
                    | EditableMultiSelectList
                    | Radio => match self.state.creating_enum_value {
                        Some(_) => html! {},
                        None => view_create_enum_btn_container(&self.link),
                    },
                    _ => html! {},
                };
                view_edit_field_container(
                    &self.link,
                    &field,
                    enum_values_list_view,
                    creating_enum_values_view,
                    create_enum_btn_view,
                    editing_enum_values_view,
                    self.view_field_type_select(field.data_type.clone()),
                )
                // view_edit_field_container(&self.link, &field, creating_enum_values_view, create_enum_btn_view, self.view_field_type_select(field.data_type.clone()))
            }
            _ => html! {},
        }
    }

    fn view_field_type_select(&self, field_type: FieldDataType) -> Html {
        view_field_type_select(&self.link, field_type)
    }
}

fn main() {
    yew::start_app::<App>();
}
