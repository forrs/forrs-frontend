use yew::prelude::*;
use yew_router::prelude::*;

pub struct Model {
    show_menu: bool,
}

pub enum Msg {
    ToggleMenu,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Model { show_menu: false }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        // TODO: The menu should be updated when a user logs in.
        match msg {
            Msg::ToggleMenu => {
                self.show_menu = !self.show_menu;
                true
            }
        }
    }
}

impl Renderable<Model> for Model {
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
                        <a class="navbar-item">{ "$LOGO" }</a>
                        <a role="button" class=burger_class onclick=|_| Msg::ToggleMenu>
                            <span aria-hidden="true"></span>
                            <span aria-hidden="true"></span>
                            <span aria-hidden="true"></span>
                        </a>
                    </div>

                    <div class=menu_class, >
                        <div class="navbar-start">
                            <RouterLink text="Home", link="/", classes="navbar-item" />
                            <RouterLink text="Topic 1", link="/topic/1", classes="navbar-item" />
                        </div>
                    </div>
                </div>
            </nav>
        }
    }
}
