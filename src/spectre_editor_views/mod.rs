use crate::form_model::form_model::Model;
use crate::form_model::form_model::FieldDataType;
use crate::form_model::form_model::EnumValues;
use crate::form_model::form_model::Field;

use yew::{Html, html, ComponentLink};
use yew::html::InputData;
use yew::components::Select;
use strum::IntoEnumIterator;

use crate::App;
use crate::Msg;
use crate::EditingEnumValue;

// TODO: add cancel save (when creating editing enum values)
pub fn main_view(link: &ComponentLink<App>, model: &Model, top_view: Html ) -> Html {
    html! {
        <div>
            { top_view }
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
                      <th>{"Action"}</th>
                    </tr>
                  </thead>
                  <tbody>
                  { for model.fields.iter().enumerate().map(|(index, field)| view_field_item(link, index, field)) }
                  </tbody>
                </table>
            </div>
        </div>
    }
}

pub fn view_edit_model_view(link: &ComponentLink<App>, model: &Model) -> Html {
    let model_subtitle: String = if let Some(model_subtitle) = &model.subtitle { model_subtitle.clone() } else { "".to_string() };
    let model_title: String = if let Some(model_title) = &model.title { model_title.clone() } else { "".to_string() };

    html! {
        <div>
            <h1>{"Model: "}{&model.name}</h1>
            <div>
                <form class="form-horizontal">
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
                            <button class="btn" onclick=link.callback(|_| Msg::NewField)>{"New field"}</button>
                        </div>
                    </div>
                </form>
            </div>
        </div>
    }
}

fn view_field_item(link: &ComponentLink<App>, index: usize, field: &Field) -> Html {
    html! {
        <tr>
          <td>{&field.name}</td>
          <td>{&field.data_type}</td>
          <td>{&field.label}</td>
          <td>{&field.placeholder}</td>
          <td>{&field.required}</td>
          <td>
            <button class="btn" onclick=link.callback(move |_| Msg::EditField(index))>{"edit"}</button>
            <button class="btn">{"delete"}</button>
            </td>
        </tr>
    }
}

pub fn view_enum_values_list_container(link: &ComponentLink<App>, enum_values: &Vec<EnumValues>) -> Html {
    html! {
        <table class="table table-striped table-hover">
          <thead>
            <tr>
              <th>{"Value"}</th>
              <th>{"Label"}</th>
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
                <button class="btn" onclick=link.callback(move |_| Msg::DeleteEnumValue(index))>{"delete"}</button>
                <button class="btn" onclick=link.callback(move |_| Msg::EditFieldEnumValues(index))>{"edit"}</button>
            </td>
        </tr>
    }
}

pub fn view_editing_enum_value(link: &ComponentLink<App>, editing_enum_value: &EditingEnumValue) -> Html {
    html!{
        <div>
            <h4>{"Editing Enum value"}</h4>
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
                <button class="btn" onclick=link.callback(|_| Msg::CancelEditEnumValues)>{"cancel"}</button>
                <button class="btn" onclick=link.callback(|_| Msg::UpdateFieldEnumValues)>{"save"}</button>
            </div>
        </div>
    }
}

// TODO clone select component and add class, id param as a prop
pub fn view_field_type_select(link: &ComponentLink<App>, field_type: FieldDataType) -> Html {
    html! {
        <div class="form-group">
            <Select<FieldDataType>
                selected=Some(field_type)
                options=FieldDataType::iter().collect::<Vec<_>>()
                onchange=link.callback(Msg::UpdateFieldType) />
        </div>
    }
}

pub fn view_enum_values_container(editing_enum_value: Html, enum_values_list: Html) -> Html {
    html!{
        <div>
            <h4>{"Enum values"}</h4>
            {editing_enum_value}
            {enum_values_list}
        </div>
    }
}

pub fn view_new_field_container(link: &ComponentLink<App>, creating_field: &Field, creating_enum_values_view: Html, create_enum_btn_view: Html, field_type_select_view: Html) -> Html {
    html! {
        <div>
            <h3>{"New field: "}{&creating_field.name}</h3>
            <form class="form-horizontal">
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
                    </div>
                </div>

                <div class="form-group">
                    <div class="col-3 col-sm-12">

                    </div>
                    <div class="col-9 col-sm-12">
                        <button class="btn" onclick=link.callback(|_| Msg::CancelNewField)>{"cancel"}</button><button class="btn" onclick=link.callback(|_| Msg::CreateField)>{"save"}</button>
                    </div>
                </div>
            </form>
        </div>
    }
}

pub fn view_edit_field_container(link: &ComponentLink<App>, creating_field: &Field, creating_enum_values_view: Html, create_enum_btn_view: Html, field_type_select_view: Html) -> Html {
    html! {
        <div>
            <h3>{"Edit field: "}{&creating_field.name}</h3>
            <form class="form-horizontal">
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
                    </div>
                </div>

                <div class="form-group">
                    <div class="col-3 col-sm-12">

                    </div>
                    <div class="col-9 col-sm-12">
                        <button class="btn" onclick=link.callback(|_| Msg::CancelEditField)>{"cancel"}</button>
                        <button class="btn" onclick=link.callback(|_| Msg::UpdateField)>{"save"}</button>
                    </div>
                </div>
            </form>



        </div>
    }
}

pub fn view_create_enum_btn_container(link: &ComponentLink<App>, view_enum_values_list: Html) -> Html {
    html! {
        <div>
            <button class="btn" onclick=link.callback(|_| Msg::NewEnumValue)>
                {"New Type value"}
            </button>
            {view_enum_values_list}
        </div>
    }
}

pub fn view_creating_enum_values(link: &ComponentLink<App>, creating_enum_value: &EnumValues) -> Html {
    html!{
        <div>
            <h5>{"creating enum"}</h5>
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
            <button class="btn" onclick=link.callback(|_| Msg::CreateNewEnumValue)>
                {"add"}
            </button>
        </div>
    }
}
