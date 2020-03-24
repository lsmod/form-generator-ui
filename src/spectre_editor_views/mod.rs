use crate::form_model::form_model::EnumValues;
use crate::EditingEnumValue;
use yew::{Html, html};
use yew::html::InputData;

use crate::App;
use crate::Msg;
use crate::form_model::form_model::Field;

pub fn view_field_item(app: &App, index: usize, field: &Field) -> Html {
    html! {
        <li>
            {index}
            {"Name:"}{&field.name}
            {"Type:"}{&field.data_type}
            {"Label:"}{&field.label}
            {"Placeholder"}{&field.placeholder}
            {"Required"}{&field.required}
            <button onclick=app.link.callback(move |_| Msg::EditField(index))>{"edit"}</button>
            <button>{"delete"}</button>
        </li>
    }
}

pub fn view_enum_values_item(app: &App, index: usize, enum_value: &EnumValues) -> Html {
    html! {
        <div>
            <div>{"value: "}{&enum_value.value}</div>
            <div>{"label: "}{&enum_value.label}</div>
            <button onclick=app.link.callback(move |_| Msg::DeleteEnumValue(index))>{"delete"}</button>
            <button onclick=app.link.callback(move |_| Msg::EditFieldEnumValues(index))>{"edit"}</button>
        </div>
    }
}

pub fn view_editing_enum_value(app: &App, editing_enum_value: &EditingEnumValue) -> Html {
    html!{
        <div>
            {"Editing Enum value: "}
            <input type="text"
                name="value_enum_value"
                oninput=app.link.callback(move |input: InputData|
                            {
                                Msg::UpdateEnumValueValue(input.value)
                            })
                value={&editing_enum_value.enum_value.value} />
                {"label: "}
                <input type="text"
                    name="label_enum_value"
                    oninput=app.link.callback(move |input: InputData|
                                {
                                    Msg::UpdateEnumValueLabel(input.value)
                                })
                    value={&editing_enum_value.enum_value.label} />
                    <button onclick=app.link.callback(|_| Msg::CancelEditEnumValues)>{"cancel"}</button>
            <button onclick=app.link.callback(|_| Msg::UpdateFieldEnumValues)>{"save"}</button>
        </div>
    }
}

pub fn view_creating_enum_values(app: &App, creating_enum_value: &EnumValues) -> Html {
    html!{
        <div>
            {"creating enum"}
            {"Label:"}
            <input
                type="text"
                value={&creating_enum_value.label}
                oninput=app.link.callback(move |input: InputData|
                    {
                        Msg::UpdateNewEnumValueLabel(input.value)
                    })
            />
            {"Value:"}
            <input
                type="text"
                value={&creating_enum_value.value}
                oninput=app.link.callback(move |input: InputData|
                    {
                        Msg::UpdateNewEnumValueValue(input.value)
                    })
            />
            <button onclick=app.link.callback(|_| Msg::CreateNewEnumValue)>
                {"add"}
            </button>

        </div>
    }
}
