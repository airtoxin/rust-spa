use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct Model {
    link: ComponentLink<Self>,
    value: i64,
    minus_amount: i64
}

enum Msg {
    AddOne,
    ChangeMinusAmount { amount: String },
    Minus
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
            minus_amount: 1
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.value += 1,
            Msg::ChangeMinusAmount { amount } => self.minus_amount = amount.parse().unwrap(),
            Msg::Minus => self.value -= self.minus_amount
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <button onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
                <input value=self.minus_amount type="number" oninput=self.link.callback(|e: InputData| Msg::ChangeMinusAmount { amount: e.value }) />
                <button onclick=self.link.callback(|_| Msg::Minus)>{ format!("-{}", self.minus_amount) }</button>
                <p>{ self.value }</p>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
