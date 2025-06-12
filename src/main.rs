use dioxus::prelude::*;

mod components;
use components::quantifier::one_part;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
  rsx! { 
    h1 { "测试" }
    span { "测试2" }
    one_part{}
  }
}
