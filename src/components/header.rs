use yew::{function_component, html};

#[function_component(Header)]
pub fn header() -> Html {
  html! {
    // Bootstrap 使ったことないのでサンプルそのまま
    <nav class="navbar navbar-expand-lg navbar-dark bg-dark">
      <div class="container">
        <a class="navbar-brand" href="#">{ "Yew Practice" }</a>
      </div>
    </nav>
  }
}
