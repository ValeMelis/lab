use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/games")]
    Games,
    #[not_found]
    #[at("/404")]
    NotFound
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html!{"a"},
        Route::Games => html!{"b"},
        Route::NotFound => html!{"c"}
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
