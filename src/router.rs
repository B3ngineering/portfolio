use yew_router::prelude::*;
use yew::prelude::*;
use crate::components::pages::about::About;
use crate::components::pages::portfolio::Portfolio;


#[derive(Routable, Debug, Clone, PartialEq)]

pub enum Route {
    #[at("/")]
    About,
    #[at("/portfolio")]
    Portfolio,
}

pub fn switch(route: &Route) -> Html {
    match route {
        Route::About => html! { <About /> },
        Route::Portfolio => html! { <Portfolio /> },
    }
}