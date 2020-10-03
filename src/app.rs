use yew::prelude::*;
use rand::prelude::*;

pub struct App {
    items: Vec<i64>,
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
        App { link, items: Vec::new() }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.items.push(random()),
            Msg::RemoveOne => {
                self.items.pop();
            }
        }
        true
    }  

    fn view(&self) -> Html {
        let render_item = |item| {
            html! {
                <>
                    <tr><td>{ item }</td></tr>
                </>
            }
        };
        html! {
            <div class="main">
                <div class="card">
                    <header>
                        {"Items: "}
                    </header>
                    <div class="card-body">
                        <table class="primary">
                            { for self.items.iter().map(render_item) }
                        </table>
                    </div>
                    <footer>
                        <button onclick=self.link.callback(|_| Msg::AddOne)>{ "Add item" }</button> {" "}
                        <button onclick=self.link.callback(|_| Msg::RemoveOne)>{ "Remove item" }</button>
                    </footer>
                </div>
            </div>
        }
    }
}