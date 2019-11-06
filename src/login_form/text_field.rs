use yew::prelude::*;

pub struct Model {
    props: Props,
}

#[derive(PartialEq, Properties)]
pub struct Props {
    #[props(required)]
    pub label: String,
    pub placeholder: String,
    pub input_type: String,
    pub value: String,

    #[props(required)]
    pub onchange: Callback<String>,
}

pub enum Msg {
    UpdateValue(String),
}

impl Component for Model {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Model { props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateValue(new_value) => {
                self.props.value = new_value;
                self.props.onchange.emit(self.props.value.clone());
                true
            }
        }
    }

    // Note: This is necessary because components from fragments will be reused!
    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if props != self.props {
            self.props = props;
            true
        } else {
            false
        }
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div class="field">
                <label class="label">{ &self.props.label }</label>
                <div class="control">
                    <input class="input" type=&self.props.input_type
                           placeholder=&self.props.placeholder
                           oninput=|e| Msg::UpdateValue(e.value)
                           value=&self.props.value
                    />
                </div>
            </div>
        }
    }
}
