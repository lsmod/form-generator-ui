use crate::GeneratedFile;
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Properties, Clone)]
pub struct Props {
    pub id: Option<String>,
    pub files: Vec<GeneratedFile>,
}

pub enum Msg {
    SelectFile(usize),
}

pub struct GeneratedFilesPannel {
    props: Props,
    link: ComponentLink<Self>,
    selected_file: usize,
}

impl Component for GeneratedFilesPannel {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        GeneratedFilesPannel {
            props,
            link,
            selected_file: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SelectFile(index) => self.selected_file = index,
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        if self.props.files.len() > 0 {
            html! {
                <div style="margin-top: 20px;">
                    <h2 style="text-align: center;">
                        <i>{"Source Files:"}</i>
                    </h2>
                    <ul class="tab tab-block" >
                    {for self.props.files.iter().enumerate().map(|(index, file)| {
                        let li_class = if index == self.selected_file { "tab-item active"} else { "tab-item"};
                        html!{
                        <li class={li_class} onclick=self.link.callback(move |_| Msg::SelectFile(index))>
                          <a href="#">{&file.file_name}</a>
                        </li>
                        }
                    })}
                    </ul>
                    <pre class="code" data-lang={self.props.files[self.selected_file].language}>
                        <code>{&self.props.files[self.selected_file].content}</code>
                    </pre>
                </div>
            }
        } else {
            html! {}
        }
    }
}
