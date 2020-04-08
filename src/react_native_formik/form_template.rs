use askama::Template;
use crate::form_model::form_model::*;
use crate::react_native_formik::helpers::to_input_type;
use crate::react_native_formik::helpers::to_html_type;
use crate::inflector::Inflector;


/// Field with added informations needed for template generation
struct TemplateField<'a> {
    pub html_element: &'a str,
    pub input_type: &'a str,
    pub name: &'a String,
    pub label: &'a String,
    pub placeholder: &'a String,
    pub required: bool,
    pub validation: &'a Option<Validation>,
}

impl<'a> From<&'a Field> for TemplateField<'a> {
    fn from(field: &'a Field) -> TemplateField<'a> {
        TemplateField {
            html_element: to_html_type(&field.data_type),
            input_type: to_input_type(&field.data_type),
            name: &field.name,
            label: &field.label,
            placeholder: &field.placeholder,
            required: field.required,
            validation: &field.validation,
        }
    }
}

#[derive(Template)]
#[template (path = "react-native-formik/form.html", escape = "none")]
pub struct FormTemplate<'a> {
    name: String,
    submit_label: &'a String,
    title: &'a Option<String>,
    subtitle: &'a Option<String>,
    fields: Vec<TemplateField<'a>>
}

impl<'a> From<& 'a Model> for FormTemplate<'a> {
    fn from(model: &'a Model) -> FormTemplate<'a> {
        let new_fields = model.fields
                                .iter()
                                .map(|field| TemplateField::from(field))
                                .collect();
        FormTemplate {
            name: model.name.to_class_case(),
            submit_label: &model.submit_label,
            title: &model.title,
            subtitle: &model.subtitle,
            fields: new_fields
        }
    }
}
