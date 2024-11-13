use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::Route;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <nav class="sidebar">
            <img src="/static/images/blue_lock.png" alt="Logo" class="logo" />
            <h1>{"Benjamin Young"}</h1>
            <div class="social-links">
                <a href="https://youtube.com" target="_blank">{"YouTube"}</a>
                <a href="https://linkedin.com" target="_blank">{"LinkedIn"}</a>
                <a href="https://github.com" target="_blank">{"GitHub"}</a>
            </div>
            <ul>
                <li><Link<Route> to={Route::Home}>{"Home"}</Link<Route>></li>
                <li><Link<Route> to={Route::About}>{"About"}</Link<Route>></li>
                <li><Link<Route> to={Route::Portfolio}>{"Portfolio"}</Link<Route>></li>
            </ul>
        </nav>
    }
}