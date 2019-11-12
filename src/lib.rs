#![recursion_limit = "512"]
use yew::prelude::*;
use yew_router::{prelude::*, Switch};

mod menu;
use menu::Model as Menu;
mod login_form;
use login_form::Model as LoginForm;

pub struct Model {
    show_login: bool,
}

#[derive(Debug, Switch, Clone)]
pub enum AppRoute {
    #[to = "/page-not-found"]
    PageNotFound(Option<String>),
    #[to = "/topic/{id}"]
    Topic { id: u64 },
    #[to = "/"]
    Index,
}

pub enum Msg {
    ShowLogin,
    HideLogin,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Model { show_login: false }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ShowLogin => {
                self.show_login = true;
                true
            }
            Msg::HideLogin => {
                self.show_login = false;
                true
            }
        }
    }

    fn view(&self) -> Html<Self> {
        html! { <>
            <Menu onlogin=|_| Msg::ShowLogin />
            <div class="container">
                <section class="section">
                    <Router<AppRoute, ()>
                        render = Router::render(|switch: AppRoute| {
                            match switch {
                                AppRoute::Index => html!{ <h1 class="title is-2">{ "Home page" }</h1> },
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
            <LoginForm active=self.show_login onclose=|_| Msg::HideLogin />
        </> }
    }
}
