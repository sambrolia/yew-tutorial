use yew::prelude::*;

pub struct App {
    counter: i64,
    link: ComponentLink<Self>,
}

pub enum Msg {
    AddOne,
    RemoveOne,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
          link,
          counter: 0
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.counter += 1,
            Msg::RemoveOne => self.counter -= 1,
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
            <p> {"Counter: "} { self.counter }</p>
                <button onclick=self.link.callback(|_| Msg::AddOne)>{ "Add 1" }</button>
                <button onclick=self.link.callback(|_| Msg::RemoveOne)>{ "Remove 1" }</button>
            </div>
        }
    }
}