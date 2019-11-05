#![recursion_limit = "512"]
use yew::prelude::*;
use yew_router::{prelude::*, Switch};

mod menu;
use menu::Model as Menu;

pub struct Model;

#[derive(Debug, Switch, Clone)]
pub enum AppRoute {
    #[to = "/page-not-found"]
    PageNotFound(Option<String>),
    #[to = "/topic/{id}"]
    Topic { id: u64 },
    #[to = "/"]
    Index,
}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Model
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! { <>
            <Menu />
            <div class="container">
                <section class="section">
                    <Router<AppRoute, ()>
                        render = Router::render(|switch: AppRoute| {
                            match switch {
                                AppRoute::Index => html!{ <h1 class="title is-2">{ "Home page"}</h1> },
                                AppRoute::Topic { id } => html!{
                                    <h1 class="title is-2">
                                        { format!("Hello! Topic {} here.", id) }
                                    </h1>
                                },
                                AppRoute::PageNotFound(None) => html!{"Page not found"},
                                AppRoute::PageNotFound(Some(missed_route)) => html!{format!("Page '{}' not found", missed_route)},
                            }
                        })
                        redirect = Router::redirect(|route: Route| {
                            AppRoute::PageNotFound(Some(route.route))
                        })
                    />
                </section>
            </div>
        </> }
    }
}
