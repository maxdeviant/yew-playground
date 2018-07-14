use yew::prelude::*;

use design_system::components::button::{Button, Color};

pub struct Counter {
    value: i32,
    onclick: Option<Callback<i32>>,
}

pub enum Msg {
    Increment,
    Decrement,
}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub onclick: Option<Callback<i32>>,
}

impl Default for Props {
    fn default() -> Self {
        Props { onclick: None }
    }
}

impl Component for Counter {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Counter {
            value: 0,
            onclick: props.onclick,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Increment => {
                self.value = self.value + 1;
                if let Some(ref onclick) = self.onclick {
                    onclick.emit(self.value)
                }
            }
            Msg::Decrement => {
                self.value = self.value - 1;
                if let Some(ref onclick) = self.onclick {
                    onclick.emit(self.value)
                }
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.onclick = props.onclick;
        true
    }
}

impl Renderable<Counter> for Counter {
    fn view(&self) -> Html<Self> {
        html! {
            <div class="counter",>
                <Button: color=Color::Primary, title="-", onclick=|_| Msg::Decrement, />
                <p style="display: inline-block; padding: 0 20px;",>{self.value}</p>
                <Button: color=Color::Primary, title="+", onclick=|_| Msg::Increment, />
            </div>
        }
    }
}
