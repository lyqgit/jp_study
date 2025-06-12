use dioxus::prelude::*;

#[component]
pub fn adjective_one_type() -> Element {
  rsx! {
    div { 
      h1 { "一类形容词"}
    }
    table {
      class: "border border-black border-collapse w-full text-left",
      thead {
        tr {
          th { "形容词" }
          th { "时态" }
          th { "肯定" }
          th { "否定" }
        }
      }
      tbody {  
        tr {
          td { rowspan:2,"辛い" }
          td { "现在/将来" }
          td { "辛い" }
          td { "辛くない" }
        }
        tr {
          td { "过去" }
          td { "辛かった" }
          td { "辛くなかった" }
        }

        tr {
          td { rowspan:2,"広い" }
          td { "现在/将来" }
          td { "広い" }
          td { "広くない" }
        }
        tr {
          td { "过去" }
          td { "広かった" }
          td { "広くなかった" }
        }
      }
    }
  }
}