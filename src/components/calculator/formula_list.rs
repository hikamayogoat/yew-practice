use yew::{function_component, html, Html};
use crate::components::calculator::formula::Formula;
use crate::components::calculator::types::CalcHistory;

#[function_component(FormulaList)]
pub fn formula_list() -> Html {
  let calc_histories = vec![
    CalcHistory {
      id: 1,
      left: 1,
      right: 2,
      answer: 3,
    },
    CalcHistory {
      id: 2,
      left: 4,
      right: 5,
      answer: 9,
    },
    CalcHistory {
      id: 3,
      left: 6,
      right: 7,
      answer: 13,
    },

  ];

  html! {
    <ul class="list-group">
      {calc_histories.iter().map(|history| html! {
        <Formula left={history.left} right={history.right} answer={history.answer} />
      }).collect::<Html>()}
    </ul>
  }
}