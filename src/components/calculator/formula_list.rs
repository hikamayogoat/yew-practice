use yew::{function_component, html};
use crate::components::calculator::formula::Formula;
use crate::components::calculator::types::CalcHistory;

#[function_component(FormulaList)]
pub fn formula_list() -> Html {
  let calc_history = CalcHistory {
    id: 1,
    left: 1,
    right: 2,
    answer: 3,
  };

  html! {
    <ul class="list-group">
      <Formula left={calc_history.left} right={calc_history.right} answer={calc_history.answer} />
    </ul>
  }
}