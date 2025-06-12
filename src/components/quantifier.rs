use dioxus::prelude::*;

//　个数单位
#[component]
pub fn one_part() -> Element {
  rsx!{
    table {
      class: "border border-black border-collapse w-full text-left",
      thead { 
        tr {
          th { "序号" }
          th { "つ" }
          th { "罗马音" }
        }
      }
      tbody { 
        tr {
          td { "1" }
          td { "一つ" }
          td { "hitotu" }
        }
        tr {
          td { "2" }
          td { "二つ" }
          td { "futatu" }
        }
        
        tr {
          td { "3" }
          td { "三つ" }
          td { "mittu" }
        }

        tr {
          td { "4" }
          td { "四つ" }
          td { "yottu" }
        }

        tr {
          td { "5" }
          td { "五つ" }
          td { "itutu" }
        }

        tr {
          td { "6" }
          td { "六つ" }
          td { "muttu" }
        }

        tr {
          td { "7" }
          td { "七つ" }
          td { "nanatu" }
        }

        tr {
          td { "8" }
          td { "八つ" }
          td { "yattu" }
        }

        tr {
          td { "9" }
          td { "九つ" }
          td { "kokonotu" }
        }

        tr {
          td { "10" }
          td { "十" }
          td { "too" }
        }
      }
    }
  }
}