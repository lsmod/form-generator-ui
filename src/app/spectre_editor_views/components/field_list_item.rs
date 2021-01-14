use crate::app::spectre_editor_views::components::button::Button;
use crate::app::Field;
use yew::{html, Callback, Children, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Properties, Clone)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    pub field: Field,
    pub up_onclick: Callback<()>,
    pub down_onclick: Callback<()>,
    pub delete_onclick: Callback<()>,
    pub edit_onclick: Callback<()>,
}

pub enum Msg {
    MoveFieldUp,
    MoveFieldDown,
    DeleteField,
    EditField,
}

pub struct FieldListItem {
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for FieldListItem {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        FieldListItem { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::MoveFieldUp => self.props.up_onclick.emit(()),
            Msg::MoveFieldDown => self.props.down_onclick.emit(()),
            Msg::DeleteField => self.props.delete_onclick.emit(()),
            Msg::EditField => self.props.edit_onclick.emit(()),
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props; // TODO check if props !=
        true
    }

    fn view(&self) -> Html {
        html! {
            <tr>
                <td>{&self.props.field.name}</td>
                <td>{&self.props.field.data_type}</td>
                <td>{&self.props.field.label}</td>
                <td>{&self.props.field.placeholder}</td>
                <td>{&self.props.field.required}</td>
                <td>
                    <Button onclick=self.link.callback( |_| Msg::MoveFieldUp)>
                        <i class="icon icon-arrow-up"></i>
                    </Button>
                    <Button onclick=self.link.callback( |_| Msg::MoveFieldDown)>
                        <i class="icon icon-arrow-down"></i>
                    </Button>
                </td>
                <td>
                    <Button onclick=self.link.callback(|_| Msg::DeleteField)>
                        <i class="icon icon-cross"></i> {" Delete"}
                    </Button>
                    <Button id={format!("edit-btn-{}", &self.props.field.name)} onclick=self.link.callback(|_| Msg::EditField) primary=true>
                        <i class="icon icon-edit"></i> {" Edit"}
                    </Button>
                </td>
            </tr>
        }
    }
}
