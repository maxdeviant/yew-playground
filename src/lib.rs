#[macro_use]
extern crate yew;

use yew::prelude::*;

pub struct Model {
    state: State,
}

pub struct State {
    app_name: String
}

pub enum Msg {
    Empty
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let state = State {
            app_name: String::from("My App")
        };
        Model { state }
    }

    fn update(&mut self, message: Msg) -> ShouldRender {
        match message {
            Msg::Empty => {}
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <h1>{&self.state.app_name}</h1>
            </div>
        }
    }
}
