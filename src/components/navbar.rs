use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::Route;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <nav class="sidebar">
            <img src="/static/b.jpg" alt="Logo" class="logo" />
            <h1>{"Benjamin Young"}</h1>
            <div class="social-links">
                <a href="mailto:b9young@uwaterloo.ca" target="_blank">
                    <img src="/static/mail.png" alt="Email" />
                </a>
                <a href="https://linkedin.com/in/ben-g-young/" target="_blank">
                    <img src="/static/linkedin.png" alt="LinkedIn" />
                </a>
                <a href="https://github.com/B3ngineering" target="_blank">
                    <img src="/static/github.png" alt="GitHub" />
                </a>
            </div>
            <ul>
                <li><Link<Route> to={Route::Home}>{"Home"}</Link<Route>></li>
                <li><Link<Route> to={Route::About}>{"About"}</Link<Route>></li>
                <li><Link<Route> to={Route::Portfolio}>{"Portfolio"}</Link<Route>></li>
            </ul>
        </nav>
    }
}