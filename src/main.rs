use yew::prelude::*;

#[function_component(Model)]
fn model() -> Html {
    html! {
        <div>{"Hello World"}</div>
    }
}

fn main() {
    yew::start_app::<Model>();
}