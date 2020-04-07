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

/// take a type of data (model type) and return matching type input
pub fn to_input_type(type_definition: &FieldDataType) -> &str {
    match type_definition {
        FieldDataType::Text => "text",
        FieldDataType::Email => "text",
        FieldDataType::Url => "text",
        FieldDataType::Password => "text",
        FieldDataType::Phone => "text",
        FieldDataType::LongText => "text",
        FieldDataType::Date => "text",
        FieldDataType::Number => "text",
        FieldDataType::Radio => "text",
        FieldDataType::Checkbox => "text",
        FieldDataType::SelectList => "text",
        FieldDataType::EditableSelectList => "text",
        FieldDataType::MultiSelectList => "text",
        FieldDataType::EditableMultiSelectList => "text",
    }
}
