use crate::search::Search;
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
            <>
                <h1 class="text-red-300 text-center text-5xl">{"Reddit Search"}</h1>
                <Search />
            </>
        }
    }
}
