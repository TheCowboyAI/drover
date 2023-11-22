use leptos::*;

const TITLE_CLASS: &'static str = "rounded-lg bg-red-400 text-center text-yellow-100";
const TITLE: &str = "CIM Web";

#[component]
pub fn TitleArea() -> impl IntoView {

  view! {
    <div class={TITLE_CLASS}>
      {TITLE}
    </div>
    }
}

