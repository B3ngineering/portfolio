mod components;
mod router;

use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::{switch, Route};
use crate::components::navbar::Navbar;

#[function_component(App)]
fn model() -> Html {
    html! {
        <BrowserRouter>
            <Navbar />
            <div class="main-content">
                <Switch<Route> render={Switch::render(switch)} />
            </div>
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<App>();
}