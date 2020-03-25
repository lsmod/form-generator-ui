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


pub fn main_view(link: &ComponentLink<App>, model: &Model, editing_view: Html, creating_view: Html ) -> Html {
    let model_subtitle: String = if let Some(model_subtitle) = &model.subtitle { model_subtitle.clone() } else { "".to_string() };
    let model_title: String = if let Some(model_title) = &model.title { model_title.clone() } else { "".to_string() };

    html! {
        <div>
            <div class="model-header">
                <h1>{"Model: "}{&model.name}</h1>
                <div class="model-property">
                    <div class="model-name-container">
                        {"Name: "}
                        <input type="text" name="model-name" oninput=link.callback(move |input: InputData|
                        {
                            Msg::UpdateName(input.value)
                        }) value={&model.name} />
                    </div>
                </div>
                <div class="model-property">
                    <div class="model-title-container">
                        {"Title: "}
                        <input type="text" name="model-title" oninput=link.callback(move |input: InputData|
                        {
                            Msg::UpdateTitle(input.value)
                        }) value={model_title} />
                    </div>
                </div>
                <div class="model-property">
                    <div class="model-subtitle-container">
                        {"Sub-Title: "}
                        <input type="text" name="model-subtitle" oninput=link.callback(move |input: InputData|
                        {
                            Msg::UpdateSubtitle(input.value)
                        }) value={model_subtitle} />
                    </div>
                </div>
                <div class="model-property">
                    <div class="model-submit_label-container">
                        {"Submit button label: "}
                        <input type="text" name="model-submit_label" oninput=link.callback(move |input: InputData|
                        {
                            Msg::UpdateSubmitLabel(input.value)
                        }) value={&model.submit_label} />
                    </div>
                </div>
                <button onclick=link.callback(|_| Msg::NewField)>{"New field"}</button>
                { editing_view }
                { creating_view}
                <div class="model-field-container">
                    <ul>
                    { for model.fields.iter().enumerate().map(|(index, field)| view_field_item(link, index, field)) }
                    </ul>
                </div>
            </div>
        </div>
    }
}

fn view_field_item(link: &ComponentLink<App>, index: usize, field: &Field) -> Html {
    html! {
        <li>
            {index}
            {"Name:"}{&field.name}
            {"Type:"}{&field.data_type}
            {"Label:"}{&field.label}
            {"Placeholder"}{&field.placeholder}
            {"Required"}{&field.required}
            <button onclick=link.callback(move |_| Msg::EditField(index))>{"edit"}</button>
            <button>{"delete"}</button>
        </li>
    }
}

pub fn view_enum_values_list_container(link: &ComponentLink<App>, enum_values: &Vec<EnumValues>) -> Html {
    html! {
        <ul>
        { for enum_values
            .iter()
            .enumerate()
            .map(|(index, enum_value)|
                view_enum_values_item(link, index, enum_value)
            )
        }
        </ul>
    }
}

fn view_enum_values_item(link: &ComponentLink<App>, index: usize, enum_value: &EnumValues) -> Html {
    html! {
        <div>
            <div>{"value: "}{&enum_value.value}</div>
            <div>{"label: "}{&enum_value.label}</div>
            <button onclick=link.callback(move |_| Msg::DeleteEnumValue(index))>{"delete"}</button>
            <button onclick=link.callback(move |_| Msg::EditFieldEnumValues(index))>{"edit"}</button>
        </div>
    }
}

pub fn view_editing_enum_value(link: &ComponentLink<App>, editing_enum_value: &EditingEnumValue) -> Html {
    html!{
        <div>
            {"Editing Enum value: "}
            <input type="text"
                name="value_enum_value"
                oninput=link.callback(move |input: InputData|
                            {
                                Msg::UpdateEnumValueValue(input.value)
                            })
                value={&editing_enum_value.enum_value.value} />
                {"label: "}
                <input type="text"
                    name="label_enum_value"
                    oninput=link.callback(move |input: InputData|
                                {
                                    Msg::UpdateEnumValueLabel(input.value)
                                })
                    value={&editing_enum_value.enum_value.label} />
                    <button onclick=link.callback(|_| Msg::CancelEditEnumValues)>{"cancel"}</button>
            <button onclick=link.callback(|_| Msg::UpdateFieldEnumValues)>{"save"}</button>
        </div>
    }
}

pub fn view_field_type_select(link: &ComponentLink<App>, field_type: FieldDataType) -> Html {
    html! {
        <Select<FieldDataType>
            selected=Some(field_type)
            options=FieldDataType::iter().collect::<Vec<_>>()
            onchange=link.callback(Msg::UpdateFieldType) />
    }
}

pub fn view_enum_values_container(editing_enum_value: Html, enum_values_list: Html) -> Html {
    html!{
        <div>
            <h3>{"Enum values"}</h3>
            {editing_enum_value}
            {enum_values_list}
        </div>
    }
}

pub fn view_new_field_container(link: &ComponentLink<App>, creating_field: &Field, creating_enum_values_view: Html, create_enum_btn_view: Html, field_type_select_view: Html) -> Html {
    html! {
        <div class="model-field-editing">
            <h3>{"New field: "}{&creating_field.name}</h3>
            {"Name:"}
            <input
                type="text"
                value={&creating_field.name}
                oninput=link.callback(move |input: InputData|
                    {
                        Msg::UpdateFieldName(input.value)
                    })
            />

            {"Type:"}{field_type_select_view}<br/>
            {"Label:"}
            <input
                type="text"
                value={&creating_field.label}
                oninput=link.callback(move |input: InputData|
                    {
                        Msg::UpdateFieldLabel(input.value)
                    })
            />
            {"Placeholder:"} <input
                type="text"
                value={&creating_field.placeholder}
                oninput=link.callback(move |input: InputData|
                    {
                        Msg::UpdateFieldPlaceHolder(input.value)
                    })
            /><br/>
            {"Required :"}
            <input
                type="checkbox"
                onclick=link.callback(|_| Msg::ToggleFieldRequired)
                name="required"
                checked=creating_field.required />
            <button onclick=link.callback(|_| Msg::CancelNewField)>{"cancel"}</button>
            <button onclick=link.callback(|_| Msg::CreateField)>{"save"}</button>
            <br/>{creating_enum_values_view}
            {create_enum_btn_view}
        </div>
    }
}

pub fn view_edit_field_container(link: &ComponentLink<App>, field: &Field, creating_enum_values_view: Html, create_enum_btn_view: Html, field_type_select_view: Html) -> Html {
    html! {
        <div class="model-field-editing">
            <h3>{"Editing field: "}{&field.name}</h3>
            {"Name:"}
            <input
                type="text"
                value={&field.name}
                oninput=link.callback(move |input: InputData|
                    {
                        Msg::UpdateFieldName(input.value)
                    })
            />
            {"Type:"} {field_type_select_view}<br/>
            {"Label:"}
            <input
                type="text"
                value={&field.label}
                oninput=link.callback(move |input: InputData|
                    {
                        Msg::UpdateFieldLabel(input.value)
                    })
            />
            {"Placeholder:"} <input
                type="text"
                value={&field.placeholder}
                oninput=link.callback(move |input: InputData|
                    {
                        Msg::UpdateFieldPlaceHolder(input.value)
                    })
            /><br/>
            {"Required :"}
            <input
                type="checkbox"
                onclick=link.callback(|_| Msg::ToggleFieldRequired)
                name="required"
                checked=field.required />
            <button onclick=link.callback(|_| Msg::CancelEditField)>{"cancel"}</button>
            <button onclick=link.callback(|_| Msg::UpdateField)>{"save"}</button>
            <br/>{creating_enum_values_view}
            {create_enum_btn_view}
        </div>
    }
}

pub fn view_create_enum_btn_container(link: &ComponentLink<App>, view_enum_values_list: Html) -> Html {
    html! {
        <div>
            <button onclick=link.callback(|_| Msg::NewEnumValue)>
                {"create enum values"}
            </button>
            {view_enum_values_list}
        </div>
    }
}

pub fn view_creating_enum_values(link: &ComponentLink<App>, creating_enum_value: &EnumValues) -> Html {
    html!{
        <div>
            {"creating enum"}
            {"Label:"}
            <input
                type="text"
                value={&creating_enum_value.label}
                oninput=link.callback(move |input: InputData|
                    {
                        Msg::UpdateNewEnumValueLabel(input.value)
                    })
            />
            {"Value:"}
            <input
                type="text"
                value={&creating_enum_value.value}
                oninput=link.callback(move |input: InputData|
                    {
                        Msg::UpdateNewEnumValueValue(input.value)
                    })
            />
            <button onclick=link.callback(|_| Msg::CreateNewEnumValue)>
                {"add"}
            </button>

        </div>
    }
}
