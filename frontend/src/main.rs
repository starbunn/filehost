mod pages;
use pages::file::File;
use pages::home::Home;
use pages::notfound::NotFound;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/:path")]
    File { path: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: &Route) -> Html {
    match routes.clone() {
        Route::Home => html! {
            <Home/>
        },
        Route::File { path } => html! {
            <File path={path} />
        },
        Route::NotFound => html! {
            <NotFound />
        },
    }
}
enum Msg {
    ToggleNavbar,
}

struct Model {
    navbar: bool,
}

impl Component for Model {
    type Message = Msg;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { navbar: false }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleNavbar => {
                self.navbar = !self.navbar;
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                <main>
                    <Switch<Route> render={Switch::render(switch)} />
                </main>
            </BrowserRouter>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
