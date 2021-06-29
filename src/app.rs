use yew::prelude::*;
use super::table::Table;


pub struct App {
    counter: i64,
    link: ComponentLink<Self>,
}

pub enum Msg {
    AddOne,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { counter: 0, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => {
                self.counter += 1;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let increment = self.link.callback(|_| Msg::AddOne);
        let dimensions = (8, 8);

        html!{
            <div>
                <p>{"Counter: "}{ self.counter }</p>
                <button onclick=increment />
                <Table size=dimensions />
            </div>

        }
    }
}
