extern crate yew;
extern crate yew_app;

use yew::prelude::*;
use yew_app::Model;

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
