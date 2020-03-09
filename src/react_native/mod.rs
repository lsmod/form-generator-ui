use crate::form_model::form_model::FieldDataType;

pub fn to_html_type(type_definition: &FieldDataType) -> &str {
    match type_definition {
        FieldDataType::Text => "input",
        FieldDataType::Email => "input",
        FieldDataType::Url => "input",
        FieldDataType::Password => "input",
        FieldDataType::Phone => "input",
        FieldDataType::LongText => "input",
        FieldDataType::Date => "input",
        FieldDataType::Number => "input",
        FieldDataType::Radio => "input",
        FieldDataType::Checkbox => "input",
        FieldDataType::SelectList => "input",
        FieldDataType::EditableSelectList => "input",
        FieldDataType::MultiSelectList => "input",
        FieldDataType::EditableMultiSelectList => "input",
    }
}
