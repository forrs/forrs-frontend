use yew::prelude::*;

mod text_field;
use text_field::Model as TextField;

pub struct Model {
    props: Props,
    show_register: bool,
    form_data: model::FormData,
}

mod model {
    use serde::{Deserialize, Serialize};
    pub enum FormData {
        Login(Login),
        Register(Register),
    }
    impl FormData {
        pub fn update_login<F>(&mut self, f: F)
        where
            F: FnOnce(&mut Login),
        {
            if let Self::Login(l) = self {
                f(l);
            }
        }
        pub fn update_register<F>(&mut self, f: F)
        where
            F: FnOnce(&mut Register),
        {
            if let Self::Register(r) = self {
                f(r);
            }
        }
        pub fn set_password(&mut self, new_pw: String) {
            match self {
                Self::Register(r) => r.password = new_pw,
                Self::Login(l) => l.password = new_pw,
            }
        }
        pub fn change_to_register(&mut self) {
            if let Self::Login(l) = self {
                *self = Self::Register(l.to_register());
            }
        }
        pub fn change_to_login(&mut self) {
            if let Self::Register(r) = self {
                *self = Self::Login(r.to_login());
            }
        }
    }
    #[derive(Serialize, Deserialize, Default)]
    pub struct Login {
        pub login: String,
        pub password: String,
    }
    impl Login {
        pub fn to_register(&self) -> Register {
            Register {
                username: self.login.clone(),
                password: self.password.clone(),
                ..Default::default()
            }
        }
    }
    #[derive(Serialize, Deserialize, Default)]
    pub struct Register {
        pub username: String,
        pub password: String,
        pub email: String,
    }
    impl Register {
        pub fn to_login(&self) -> Login {
            Login {
                login: self.username.clone(),
                password: self.password.clone(),
            }
        }
    }
}

#[derive(PartialEq, Properties)]
pub struct Props {
    #[props(required)]
    pub active: bool,
    #[props(required)]
    pub onclose: Callback<()>,
}

pub enum Msg {
    UpdateLogin(String),
    UpdatePassword(String),
    UpdateUsername(String),
    UpdateEmail(String),
    ChangeToRegister,
    ChangeToLogin,
    Close,
}

impl Component for Model {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Model {
            props,
            show_register: false,
            form_data: model::FormData::Login(model::Login::default()),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use Msg::*;
        match msg {
            UpdateLogin(new_login) => self.form_data.update_login(|l| l.login = new_login),
            UpdatePassword(new_pw) => self.form_data.set_password(new_pw),
            UpdateUsername(new_name) => self.form_data.update_register(|r| r.username = new_name),
            UpdateEmail(new_mail) => self.form_data.update_register(|r| r.email = new_mail),
            ChangeToRegister => {
                self.show_register = true;
                self.form_data.change_to_register();
            }
            ChangeToLogin => {
                self.show_register = false;
                self.form_data.change_to_login();
            }
            Close => self.props.onclose.emit(()),
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html<Self> {
        use model::FormData::*;
        let (title, form) = match &self.form_data {
            Register(r) => (
                "Register",
                html! {
                    <>
                    <div class="tabs is-fullwidth">
                        <ul>
                            <li><a onclick=|_| Msg::ChangeToLogin>{ "Login" }</a></li>
                            <li class="is-active"><a>{ "Register" }</a></li>
                        </ul>
                    </div>
                    <TextField label="Username" onchange=Msg::UpdateUsername value=&r.username />
                    <TextField label="E-Mail address" onchange=Msg::UpdateEmail value=&r.email />
                    <TextField label="Password" input_type="password" value=&r.password
                               onchange=Msg::UpdatePassword />
                    </>
                },
            ),
            Login(l) => (
                "Login",
                html! {
                    <>
                    <div class="tabs is-fullwidth">
                        <ul>
                            <li class="is-active"><a>{ "Login" }</a></li>
                            <li><a onclick=|_| Msg::ChangeToRegister>{ "Register" }</a></li>
                        </ul>
                    </div>
                    <TextField label="Username or e-mail" onchange=Msg::UpdateLogin value=&l.login />
                    <TextField label="Password" input_type="password" value=&l.password
                               onchange=Msg::UpdatePassword />
                    </>
                },
            ),
        };
        let modal_class = if self.props.active {
            "modal is-active"
        } else {
            "modal"
        };
        html! {
            <div class=modal_class>
            <div class="modal-background" onclick=|_| Msg::Close></div>
            <div class="modal-card">
                <header class="modal-card-head">
                    <p class="modal-card-title">{ title }</p>
                </header>
                <section class="modal-card-body">
                    { form }
                </section>
                <footer class="modal-card-foot">
                    <button class="button" onclick=|_| Msg::Close>{ "Cancel" }</button>
                    <button class="button is-link">{ title }</button>
                </footer>
            </div>
            </div>
        }
    }
}
