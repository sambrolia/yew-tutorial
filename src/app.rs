use yew::prelude::*;
use rand::prelude::*;
use yew::services::{ConsoleService, DialogService};


pub struct App {
    items: Vec<i64>,
    link: ComponentLink<Self>,
    console: ConsoleService,
    dialog: DialogService,
}

pub enum Msg {
    AddOne,
    RemoveOne,
    About
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App { 
            link, 
            items: Vec::new(), 
            console: ConsoleService::new(), 
            dialog: DialogService::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::About => self.dialog.alert("Purposeless App"),
            Msg::AddOne => {
                let added: i64 = random();
                self.items.push(added);
                self.console.log(format!("Added: {}", added).as_str());
                self.console.info("Added 1 elemet to the vec");
            }
            Msg::RemoveOne => {
                let removed = self.items.pop();
                match removed {
                    Some(x) => self.console.warn(format!("Removed {}", x).as_str()),
                    None => self.console.error("No more elements to remove!"),
                };
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
                        <h2>{"Items: "}</h2>
                        <button onclick=self.link.callback(|_| Msg::About)>{ "About" }</button>
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