use crate::form_model::form_model::Field;
use crate::form_model::form_model::FieldDataType;
use crate::react_native_formik::form_template::FieldType;
use crate::react_native_formik::form_template::FieldType::*;

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
pub fn to_input_type(type_definition: &FieldDataType) -> &FieldType {
    match type_definition {
        FieldDataType::Text => &Input,
        FieldDataType::Email => &Input,
        FieldDataType::Url => &Input,
        FieldDataType::Password => &Input,
        FieldDataType::Phone => &Input,
        FieldDataType::LongText => &Textarea,
        FieldDataType::Date => &Input,
        FieldDataType::Number => &Input,
        FieldDataType::Radio => &Picker,
        FieldDataType::Checkbox => &Picker,
        FieldDataType::SelectList => &Picker,
        FieldDataType::EditableSelectList => &Input,
        FieldDataType::MultiSelectList => &Input,
        FieldDataType::EditableMultiSelectList => &Input,
    }
}

pub fn list_component_imports(fields: &Vec<Field>) -> Vec<&str> {
    let mut imports = Vec::new();

    // TODO use a hashset instead
    for field in fields {
        let import = to_component_import(&field.data_type);
        if !imports.contains(&import) {
            imports.push(import)
        }
    }
    imports
}

fn to_component_import(type_definition: &FieldDataType) -> &str {
    match type_definition {
        FieldDataType::Text => "Input",
        FieldDataType::Email => "Input",
        FieldDataType::Url => "Input",
        FieldDataType::Password => "Input",
        FieldDataType::Phone => "Input",
        FieldDataType::LongText => "Textarea",
        FieldDataType::Date => "Input",
        FieldDataType::Number => "Input",
        FieldDataType::Radio => "Picker",
        FieldDataType::Checkbox => "Picker",
        FieldDataType::SelectList => "Picker",
        FieldDataType::EditableSelectList => "Input",
        FieldDataType::MultiSelectList => "Input",
        FieldDataType::EditableMultiSelectList => "Input",
    }
}
