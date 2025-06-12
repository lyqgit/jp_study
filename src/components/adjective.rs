use dioxus::prelude::*;

#[component]
fn adjective_one_type() -> Element {
  rsx! {
    div { 
      h1 { "一类形容词"}
    }
    table {
      class: "border border-black border-collapse w-full text-left",
      thead {
        tr {
          th { "形容词" }
          th { "体" }
          th { "现在/将来" }
          th { "现在/将来（否定）" }
          th { "过去" }
          th { "过去（否定）" }
        }
      }
      tbody {  
        tr {
          td { "辛い" }
          td { rowspan:2, "敬体" }
          td { "简体" }
          td { "辛い" }
          td { "辛かった" }
          td { "辛くない" }
          td { "辛く買ったない" }
        }
      }
    }
  }
}