use crate::form_model::form_model::EnumValues;
use crate::form_model::form_model::Field;
use crate::form_model::form_model::FieldDataType;
use crate::form_model::form_model::Model;

use strum::IntoEnumIterator;
use yew::components::Select;
use yew::html::InputData;
use yew::{html, ComponentLink, Html};

use crate::App;
use crate::EditingEnumValue;
use crate::GeneratedFile;
use crate::Msg;
mod components;
use self::components::{
    button::Button, field_list_container::FieldListContainer, field_list_item::FieldListItem,
};

// TODO refactor into components:
// ModelForm
// NewFieldForm
// EditFieldForm
// NewEnumForm
// EditEnumForm
// EnumList
// GeneratedFileView

// base component:
// input, h1, h2, icons

// TODO: add cancel save (when creating editing enum values)
pub fn main_view(
    link: &ComponentLink<App>,
    model: &Model,
    generated_files: &Vec<GeneratedFile>,
    selected_file: usize,
    top_view: Html,
) -> Html {
    let generated_files_view = view_generated_files(link, generated_files, selected_file);
    html! {
        <div style="margin-left: 5vw; margin-right: 5vw; margin-top: 20px;">
            { top_view }
            <FieldListContainer>
                { for model.fields
                    .iter()
                    .enumerate()
                    .map(|(index, field)| html!{
                        <FieldListItem
                          field=field
                          down_onclick=link.callback(move |_| Msg::MoveFieldDown(index))
                          up_onclick=link.callback(move |_| Msg::MoveFieldUp(index))
                          delete_onclick=link.callback(move |_| Msg::DeleteField(index))
                          edit_onclick=link.callback(move |_| Msg::EditField(index))
                          />
                    })
                }
            </FieldListContainer>
            {generated_files_view}
        </div>
    }
}

pub fn view_generated_files(
    link: &ComponentLink<App>,
    generated_files: &Vec<GeneratedFile>,
    selected_file: usize,
) -> Html {
    if generated_files.len() > 0 {
        html! {
            <div style="margin-top: 20px;">
                <h2 style="text-align: center;">
                    <i>{"Source Files:"}</i>
                </h2>
                <ul class="tab tab-block" >
                {for generated_files.iter().enumerate().map(|(index, file)| {
                    let li_class = if index == selected_file { "tab-item active"} else { "tab-item"};
                    html!{
                    <li class={li_class} onclick=link.callback(move |_| Msg::SelectFile(index))>
                      <a href="#">{&file.file_name}</a>
                    </li>
                    }
                })}
                </ul>
                <pre class="code" data-lang={generated_files[selected_file].language}>
                    <code>{&generated_files[selected_file].content}</code>
                </pre>
            </div>
        }
    } else {
        html! {}
    }
}

pub fn view_edit_model_view(link: &ComponentLink<App>, model: &Model) -> Html {
    let model_subtitle: String = if let Some(model_subtitle) = &model.subtitle {
        model_subtitle.clone()
    } else {
        "".to_string()
    };
    let model_title: String = if let Some(model_title) = &model.title {
        model_title.clone()
    } else {
        "".to_string()
    };

    html! {
        <div>
            <h1>{"Model: "}{&model.name}</h1>
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
                                value={&model.name}
                                oninput=link.callback(move |input: InputData|
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
                                oninput=link.callback(move |input: InputData|
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
                                oninput=link.callback(move |input: InputData|
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
                                value={&model.submit_label}
                                oninput=link.callback(move |input: InputData|
                                {
                                    Msg::UpdateSubmitLabel(input.value)
                                })
                                />
                        </div>
                    </div>
                    <div class="form-group">
                        <div class="col-3 col-sm-12"></div>
                        <div class="col-9 col-sm-12">
                            <div class="columns">
                                <div class="column col-3 col-mr-auto">
                                    <Button onclick=link.callback(|_| Msg::NewField) primary=true>
                                        <i class="icon icon-plus"></i>
                                        {" New field "}
                                    </Button>
                                </div>
                                <div class="column col-2 col-ml-auto">
                                    <Button onclick=link.callback(|_| Msg::NewGenerate) primary=true>
                                        <i class="icon icon-plus"></i>
                                        {" Generate "}
                                    </Button>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

pub fn view_enum_values_list_container(
    link: &ComponentLink<App>,
    enum_values: &Vec<EnumValues>,
) -> Html {
    html! {
        <table class="table table-striped table-hover">
          <thead>
            <tr>
              <th>{"Value"}</th>
              <th>{"Label"}</th>
              <th>{"Order"}</th>
              <th>{"Edit"}</th>
            </tr>
          </thead>
          <tbody>
          { for enum_values
              .iter()
              .enumerate()
              .map(|(index, enum_value)|
                  view_enum_values_item(link, index, enum_value)
              )
          }
          </tbody>
        </table>
    }
}

fn view_enum_values_item(link: &ComponentLink<App>, index: usize, enum_value: &EnumValues) -> Html {
    html! {
        <tr>
            <td>{&enum_value.value}</td>
            <td>{&enum_value.label}</td>
            <td>
                <Button onclick=link.callback(move |_| Msg::MoveEnumValueUp(index))>
                    <i class="icon icon-arrow-up"></i>
                </Button>
                <Button onclick=link.callback(move |_| Msg::MoveEnumValueDown(index))>
                    <i class="icon icon-arrow-down"></i>
                </Button>
            </td>
            <td>
                <Button onclick=link.callback(move |_| Msg::DeleteEnumValue(index))>
                    <i class="icon icon-cross"></i>
                    {" Delete"}
                </Button>
                <Button onclick=link.callback(move |_| Msg::EditFieldEnumValues(index)) primary=true>
                    <i class="icon icon-edit"></i>
                    {" Edit"}
                </Button>
            </td>
        </tr>
    }
}

pub fn view_editing_enum_value(
    link: &ComponentLink<App>,
    editing_enum_value: &EditingEnumValue,
) -> Html {
    html! {
        <div style="margin-top: 15px;">
            <h3>{"Editing Enum value"}</h3>
            <div class="form-group">
                <label class="form-label" for="input-editing-enum-value">{"Value: "}</label>
                <input
                    type="text"
                    class="form-input"
                    id="input-editing-enum-value"
                    value={&editing_enum_value.enum_value.value}
                    oninput=link.callback(move |input: InputData|
                                {
                                    Msg::UpdateEnumValueValue(input.value)
                                })
                />
                <label class="form-label" for="input-editing-enum-label">{"label: "}</label>
                <input
                    type="text"
                    class="form-input"
                    value={&editing_enum_value.enum_value.label}
                    oninput=link.callback(move |input: InputData|
                                    {
                                        Msg::UpdateEnumValueLabel(input.value)
                                    })
                />
                <Button onclick=link.callback(|_| Msg::CancelEditEnumValues)>
                    <i class="icon icon-stop"></i>
                    {" Cancel"}
                </Button>
                <Button onclick=link.callback(|_| Msg::UpdateFieldEnumValues) primary=true>
                    <i class="icon icon-check"></i>
                    {" Save"}
                </Button>
            </div>
        </div>
    }
}

// TODO clone select component and add class, id param as a prop
pub fn view_field_type_select(link: &ComponentLink<App>, field_type: FieldDataType) -> Html {
    html! {
        <div class="form-group" id="field-editing_type_form-group">
            <Select<FieldDataType>
                selected=Some(field_type)
                options=FieldDataType::iter().collect::<Vec<_>>()
                onchange=link.callback(Msg::UpdateFieldType) />
        </div>
    }
}

pub fn view_enum_values_container(enum_values_list: Html) -> Html {
    html! {
        <div  style="margin-top: 15px;">
            <h3>{"Enum values"}</h3>
            {enum_values_list}
        </div>
    }
}

pub fn view_new_field_container(
    link: &ComponentLink<App>,
    creating_field: &Field,
    enum_values_list_view: Html,
    creating_enum_values_view: Html,
    create_enum_btn_view: Html,
    editing_enum_values_view: Html,
    field_type_select_view: Html,
) -> Html {
    let min_length_view = view_min_length_editor(link, creating_field);
    let max_length_view = view_max_length_editor(link, creating_field);

    html! {
        <div>
            <h1>{"New field: "}{&creating_field.name}</h1>
            <div class="form-horizontal">
                <div class="form-group">
                    <div class="col-3 col-sm-12">
                        <label class="form-label" for="field-editing_name">{"Name:"}</label>
                    </div>
                    <div class="col-9 col-sm-12">
                        <input
                            type="text"
                            class="form-input"
                            id="field-editing_name"
                            required=true
                            value={&creating_field.name}
                            oninput=link.callback(move |input: InputData|
                                {
                                    Msg::UpdateFieldName(input.value)
                                })
                        />
                    </div>
                </div>

                <div class="form-group">
                    <div class="col-3 col-sm-12">
                        <label class="form-label" for="field-editing_label">{"Label:"}</label>
                    </div>
                    <div class="col-9 col-sm-12">
                        <input
                            type="text"
                            class="form-input"
                            id="field-editing_label"
                            value={&creating_field.label}
                            oninput=link.callback(move |input: InputData|
                                {
                                    Msg::UpdateFieldLabel(input.value)
                                })
                        />
                    </div>
                </div>

                <div class="form-group">
                    <div class="col-3 col-sm-12">
                        <label class="form-label" for="field-editing_placeholder">{"Placeholder:"}</label>
                    </div>
                    <div class="col-9 col-sm-12">
                        <input
                            type="text"
                            class="form-input"
                            id="field-editing_placeholder"
                            value={&creating_field.placeholder}
                            oninput=link.callback(move |input: InputData|
                                {
                                    Msg::UpdateFieldPlaceHolder(input.value)
                                })
                        />
                    </div>
                </div>

                <div class="form-group">
                    <div class="col-3 col-sm-12">
                        <label class="form-label" for="field-editing_required">{"Validation:"}</label>
                    </div>
                    <div class="col-9 col-sm-12">
                        <label class="form-checkbox" for="field-editing_required">
                            <input
                                type="checkbox"
                                class="form-checkbox"
                                id="field-editing_required"
                                onclick=link.callback(|_| Msg::ToggleFieldRequired)
                                checked=creating_field.required />
                            <i class="form-icon"></i>{"Required"}
                        </label>
                    </div>
                </div>

                <div class="form-group">
                    <div class="col-3 col-sm-12">
                        {"Validation :"}
                    </div>
                    <div class="col-9 col-sm-12">
                        {min_length_view}
                    </div>
                </div>

                <div class="form-group">
                    <div class="col-3 col-sm-12">
                    </div>
                    <div class="col-9 col-sm-12">
                        {max_length_view}
                    </div>
                </div>

                <div class="form-group">
                    <div class="col-3 col-sm-12">
                        <label class="form-label" for="field-editing_name">{"Type:"}</label>
                    </div>
                    <div class="col-9 col-sm-12">
                        {field_type_select_view}
                    </div>
                </div>

                <div class="form-group">
                    <div class="col-3 col-sm-12">
                    </div>
                    <div class="col-9 col-sm-12">
                        {create_enum_btn_view}
                        {creating_enum_values_view}
                        {editing_enum_values_view}
                        {enum_values_list_view}
                    </div>
                </div>

                <div class="form-group">
                    <div class="col-3 col-sm-12">

                    </div>
                    <div class="col-9 col-sm-12">
                        <Button onclick=link.callback(|_| Msg::CancelNewField)>
                            <i class="icon icon-stop"></i>
                            {" Cancel"}
                        </Button>
                        <Button onclick=link.callback(|_| Msg::CreateField) primary=true>
                            <i class="icon icon-check"></i>
                            {" Save"}
                        </Button>
                    </div>
                </div>
            </div>
        </div>
    }
}

// TODO only for text, email, password, url, phone, EditableSelectList, EditableMultiSelectList
fn view_min_length_editor(link: &ComponentLink<App>, field: &Field) -> Html {
    match &field.validation {
        Some(validation) => match validation.min_length {
            Some(min_length) => html! {
                <table>
                    <tr>
                        <td>
                            <label class="form-checkbox" for="field-editing_min_length">
                                <input
                                    type="checkbox"
                                    class="form-checkbox"
                                    id="field-editing_min_length"
                                    onclick=link.callback(|_| Msg::ToogleMinLength)
                                    checked=true />
                                <i class="form-icon"></i>{"Min Length"}
                            </label>
                        </td>
                        <td>
                            <input
                                type="number"
                                class="form-input"
                                value={min_length}
                                min="0"
                                oninput=link.callback(move |input: InputData|
                                    {
                                        let length : usize = match input.value.parse() {
                                            Ok(len) => len,
                                            Err(_) => 0
                                        };
                                        Msg::UpdateMinLength(length)
                                    })
                            />
                        </td>
                    </tr>
                </table>
            },
            None => html! {
                <label class="form-checkbox" for="field-editing_min_length">
                    <input
                        type="checkbox"
                        class="form-checkbox"
                        id="field-editing_min_length"
                        onclick=link.callback(|_| Msg::ToogleMinLength)
                        checked=false />
                    <i class="form-icon"></i>{"Min Length"}
                </label>
            },
        },
        None => html! {
            <label class="form-checkbox" for="field-editing_min_length">
                <input
                    type="checkbox"
                    class="form-checkbox"
                    id="field-editing_min_length"
                    onclick=link.callback(|_| Msg::ToogleMinLength)
                    checked=false />
                <i class="form-icon"></i>{"Min Length"}
            </label>
        },
    }
}

fn view_max_length_editor(link: &ComponentLink<App>, field: &Field) -> Html {
    match &field.validation {
        Some(validation) => match validation.max_length {
            Some(max_length) => html! {
                <table>
                    <tr>
                        <td>
                            <label class="form-checkbox" for="field-editing_max_length">
                                <input
                                    type="checkbox"
                                    class="form-checkbox"
                                    id="field-editing_max_length"
                                    onclick=link.callback(|_| Msg::ToogleMaxLength)
                                    checked=true />
                                <i class="form-icon"></i>{"Max Length"}
                            </label>
                        </td>
                        <td>
                            <input
                                type="number"
                                class="form-input"
                                value={max_length}
                                min="0"
                                oninput=link.callback(move |input: InputData|
                                    {
                                        let length : usize = match input.value.parse() {
                                            Ok(len) => len,
                                            Err(_) => 0
                                        };
                                        Msg::UpdateMaxLength(length)
                                    })
                            />
                        </td>
                    </tr>
                </table>
            },
            None => html! {
                <label class="form-checkbox" for="field-editing_max_length">
                    <input
                        type="checkbox"
                        class="form-checkbox"
                        id="field-editing_max_length"
                        onclick=link.callback(|_| Msg::ToogleMaxLength)
                        checked=false />
                    <i class="form-icon"></i>{"Max Length"}
                </label>
            },
        },
        None => html! {
            <label class="form-checkbox" for="field-editing_max_length">
                <input
                    type="checkbox"
                    class="form-checkbox"
                    id="field-editing_max_length"
                    onclick=link.callback(|_| Msg::ToogleMaxLength)
                    checked=false />
                <i class="form-icon"></i>{"Max Length"}
            </label>
        },
    }
}

pub fn view_edit_field_container(
    link: &ComponentLink<App>,
    creating_field: &Field,
    enum_values_list_view: Html,
    creating_enum_values_view: Html,
    create_enum_btn_view: Html,
    editing_enum_values_view: Html,
    field_type_select_view: Html,
) -> Html {
    let min_length_view = view_min_length_editor(link, creating_field);
    let max_length_view = view_max_length_editor(link, creating_field);
    html! {
        <div>
            <h1>{"Edit field: "}{&creating_field.name}</h1>
            <div class="form-horizontal">
                <div class="form-group">
                    <div class="col-3 col-sm-12">
                        <label class="form-label" for="field-editing_name">{"Name:"}</label>
                    </div>

                    <div class="col-9 col-sm-12">
                        <input
                            type="text"
                            class="form-input"
                            id="field-editing_name"
                            value={&creating_field.name}
                            oninput=link.callback(move |input: InputData|
                                {
                                    Msg::UpdateFieldName(input.value)
                                })
                        />
                    </div>
                </div>

                <div class="form-group">
                    <div class="col-3 col-sm-12">
                        <label class="form-label" for="field-editing_label">{"Label:"}</label>
                    </div>
                    <div class="col-9 col-sm-12">
                        <input
                            type="text"
                            class="form-input"
                            id="field-editing_label"
                            value={&creating_field.label}
                            oninput=link.callback(move |input: InputData|
                                {
                                    Msg::UpdateFieldLabel(input.value)
                                })
                        />
                    </div>
                </div>

                <div class="form-group">
                    <div class="col-3 col-sm-12">
                        <label class="form-label" for="field-editing_placeholder">{"Placeholder:"}</label>
                    </div>
                    <div class="col-9 col-sm-12">
                        <input
                            type="text"
                            class="form-input"
                            id="field-editing_placeholder"
                            value={&creating_field.placeholder}
                            oninput=link.callback(move |input: InputData|
                                {
                                    Msg::UpdateFieldPlaceHolder(input.value)
                                })
                        />
                    </div>
                </div>

                <div class="form-group">
                    <div class="col-3 col-sm-12">

                    </div>
                    <div class="col-9 col-sm-12">
                        <label class="form-checkbox" for="field-editing_required">
                            <input
                                type="checkbox"
                                class="form-checkbox"
                                id="field-editing_required"
                                onclick=link.callback(|_| Msg::ToggleFieldRequired)
                                checked=creating_field.required />
                            <i class="form-icon"></i>{"Required"}
                        </label>
                    </div>
                </div>

                <div class="form-group">
                    <div class="col-3 col-sm-12">
                        {"Validation :"}
                    </div>
                    <div class="col-9 col-sm-12">
                        {min_length_view}
                    </div>
                </div>

                <div class="form-group">
                    <div class="col-3 col-sm-12">
                    </div>
                    <div class="col-9 col-sm-12">
                        {max_length_view}
                    </div>
                </div>

                <div class="form-group">
                    <div class="col-3 col-sm-12">
                        <label class="form-label" for="field-editing_name">{"Type:"}</label>
                    </div>
                    <div class="col-9 col-sm-12">
                        {field_type_select_view}
                    </div>
                </div>

                <div class="form-group">
                    <div class="col-3 col-sm-12">

                    </div>
                    <div class="col-9 col-sm-12">
                        {create_enum_btn_view}
                        {creating_enum_values_view}
                        {editing_enum_values_view}
                        {enum_values_list_view}
                    </div>
                </div>

                <div class="form-group">
                    <div class="col-3 col-sm-12">

                    </div>
                    <div class="col-9 col-sm-12">
                        <Button onclick=link.callback(|_| Msg::CancelEditField)>
                            <i class="icon icon-stop"></i>
                            {" Cancel"}
                        </Button>
                        <Button onclick=link.callback(|_| Msg::UpdateField) primary=true>
                            <i class="icon icon-check"></i>
                            {" Save"}
                        </Button>
                    </div>
                </div>
            </div>
        </div>
    }
}

pub fn view_create_enum_btn_container(link: &ComponentLink<App>) -> Html {
    html! {
        <div>
            <Button onclick=link.callback(|_| Msg::NewEnumValue) primary=true>
                <i class="icon icon-plus"></i>
                {" New Type value"}
            </Button>
        </div>
    }
}

pub fn view_creating_enum_values(
    link: &ComponentLink<App>,
    creating_enum_value: &EnumValues,
) -> Html {
    html! {
        <div>
            <h3>{"New enum"}</h3>
            <div class="form-group">
                <label for="creating-enum_label" class="form-label">{"Label:"}</label>
                <input
                    type="text"
                    class="form-input"
                    id="creating-enum_label"
                    value={&creating_enum_value.label}
                    oninput=link.callback(move |input: InputData|
                        {
                            Msg::UpdateNewEnumValueLabel(input.value)
                        })
                />
            </div>
            <div class="form-group">
                <label for="creating-enum_value" class="form-label">{"Value:"}</label>
                <input
                    type="text"
                    id="creating-enum_value"
                    class="form-input"
                    value={&creating_enum_value.value}
                    oninput=link.callback(move |input: InputData|
                        {
                            Msg::UpdateNewEnumValueValue(input.value)
                        })
                />
            </div>
            <Button onclick=link.callback(|_| Msg::CreateNewEnumValue) primary=true>
                <i class="icon icon-plus"></i>
                {" Add"}
            </Button>
        </div>
    }
}
