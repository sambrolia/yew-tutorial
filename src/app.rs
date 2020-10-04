use yew::prelude::*;
use rand::prelude::*;
use yew::services::ConsoleService;


pub struct App {
    items: Vec<i64>,
    link: ComponentLink<Self>,
    console: ConsoleService,
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
            items: Vec::new(), 
            console: ConsoleService::new(), 
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => {
                let added: i64 = random();
                self.items.push(added);
                self.console.log(format!("Added: {}", added).as_str());
            }
            Msg::RemoveOne => {
                let removed = self.items.pop();
                self.console
                    .log(format!("Removed {}", removed.unwrap_or_default()).as_str());
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