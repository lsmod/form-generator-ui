pub mod form_model {
    #[derive(Deserialize, Debug, Clone)]
    pub struct Model {
        pub name: String,
        pub submit_label: String,
        pub title: Option<String>,
        pub subtitle: Option<String>,
        pub fields: Vec<Field>,
    }

    #[derive(Deserialize, Debug, Clone)]
    pub struct Validation {
        pub min_length: Option<u8>,
        pub max_length: Option<u8>,
        pub enum_values: Option<Vec<String>>,
    }

    #[derive(Deserialize, Debug, Clone, EnumIter, EnumString, PartialEq)]
    pub enum FieldDataType {
        Text,
        Email,
        Url,
        Password,
        Phone,
        LongText,
        Date,
        Number,
        Radio,
        Checkbox,
        SelectList,
        EditableSelectList,
        MultiSelectList,
        EditableMultiSelectList,
    }

    impl std::fmt::Display for FieldDataType {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    #[derive(Deserialize, Debug, Clone)]
    pub struct Field {
        #[serde(rename = "type")]
        /// type of data
        pub data_type: FieldDataType,
        pub name: String,
        pub label: String,
        pub placeholder: String,
        pub required: bool,
        pub validation: Option<Validation>,
    }
}
