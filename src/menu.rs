use yew::prelude::*;
use yew_router::{
    agent::RouteRequest::ChangeRoute,
    prelude::{Route, RouteAgentDispatcher},
};

pub struct Model {
    router: RouteAgentDispatcher,
    onlogin: Callback<()>,
    show_menu: bool,
}

#[derive(PartialEq, Properties)]
pub struct Props {
    #[props(required)]
    pub onlogin: Callback<()>,
}

pub enum Msg {
    ToggleMenu,
    LinkClicked(String),
    LoginClicked,
}
impl Msg {
    fn link<T: Into<String>>(route: T) -> Self {
        Msg::LinkClicked(route.into())
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Model {
            show_menu: false,
            router: RouteAgentDispatcher::new(),
            onlogin: props.onlogin,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        // TODO: The menu should be updated when a user logs in.
        match msg {
            Msg::ToggleMenu => {
                self.show_menu = !self.show_menu;
                true
            }
            Msg::LinkClicked(route) => {
                let route = Route::from(route.as_str());
                self.router.send(ChangeRoute(route));
                self.show_menu = false;
                true
            }
            Msg::LoginClicked => {
                self.onlogin.emit(());
                false
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html<Self> {
        let (menu_class, burger_class) = if self.show_menu {
            ("navbar-menu is-active", "navbar-burger is-active")
        } else {
            ("navbar-menu", "navbar-burger")
        };
        html! {
            <nav class="navbar is-fixed-top is-light">
                <div class="container">
                    <div class="navbar-brand">
                        <a class="navbar-item" onclick=|_| Msg::link("/")>{ "$LOGO" }</a>
                        <a role="button" class=burger_class onclick=|_| Msg::ToggleMenu>
                            <span aria-hidden="true"></span>
                            <span aria-hidden="true"></span>
                            <span aria-hidden="true"></span>
                        </a>
                    </div>

                    <div class=menu_class, >
                        <div class="navbar-end">
                            <a class="navbar-item" onclick=|_| Msg::link("/topic/1")>{ "Topic 1" }</a>
                            <div class="navbar-item">
                                <div class="buttons">
                                    <button class="button is-link" onclick=|_| Msg::LoginClicked>
                                        { "Log in" }
                                    </button>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </nav>
        }
    }
}
