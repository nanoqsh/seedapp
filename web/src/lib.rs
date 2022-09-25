#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};

fn init<O>(_: Url, _: &mut O) -> Model
where
    O: Orders<Msg>,
{
    Model { counter: 0 }
}

struct Model {
    counter: i32,
}

#[derive(Copy, Clone)]
enum Msg {
    Increment,
}

fn update<O>(msg: Msg, model: &mut Model, _: &mut O)
where
    O: Orders<Msg>,
{
    match msg {
        Msg::Increment => model.counter += 1,
    }
}

fn view(model: &Model) -> Node<Msg> {
    div![
        "This is a counter: ",
        C!["counter"],
        button![model.counter, ev(Ev::Click, |_| Msg::Increment)],
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    App::start("root", init, update, view);
}
