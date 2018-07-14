use yew::prelude::*;

#[derive(PartialEq, Clone)]
pub enum Color {
    White,
    Primary,
}

pub struct Button {
    title: String,
    color: Color,
    bg: Color,
    onclick: Option<Callback<()>>,
}

pub enum Msg {
    Clicked,
}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub title: String,
    pub color: Color,
    pub bg: Color,
    pub onclick: Option<Callback<()>>,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            title: String::from(""),
            color: Color::White,
            bg: Color::Primary,
            onclick: None,
        }
    }
}

impl Component for Button {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Button {
            title: props.title,
            color: props.color,
            bg: props.bg,
            onclick: props.onclick,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked => {
                if let Some(ref mut callback) = self.onclick {
                    callback.emit(());
                }
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.title = props.title;
        self.color = props.color;
        self.onclick = props.onclick;
        true
    }
}

impl Renderable<Button> for Button {
    fn view(&self) -> Html<Self> {
        let color = match self.color {
            Color::White => "#fff",
            Color::Primary => "mediumseagreen",
        };
        let bg = match self.bg {
            Color::White => "#fff",
            Color::Primary => "mediumseagreen",
        };
        let style = format!(
            "border: none; color: {color}; background-color: {bg};",
            color = color,
            bg = bg
        );
        html! {
            <button style=style, onclick=|_| Msg::Clicked,>{&self.title}</button>
        }
    }
}
