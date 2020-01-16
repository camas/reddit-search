use log::*;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        info!("App initialized");
        App {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <section>
                <h1>{"Header"}</h1>
                <p>{"Hello world!"}</p>
            </section>
        }
    }
}
