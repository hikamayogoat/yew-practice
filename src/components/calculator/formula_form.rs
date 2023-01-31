use yew::{function_component, html};

#[function_component(FormulaForm)]
pub fn formula_form() -> Html {
  html! {
    <form class="mb-5">
      <div class="row">
        <div class="col-auto">
          <input type="number" class="form-control" placeholder="Left number" />
        </div>
        <div class="col-auto">
          <h3>{"+"}</h3>
        </div>
        <div class="col-auto">
          <input type="number" class="form-control" placeholder="Right number" />
        </div>
        <div class="col-auto">
          <button type="submit" class="btn btn-primary">{"計算"}</button>
        </div>
      </div>
    </form>
  }
}