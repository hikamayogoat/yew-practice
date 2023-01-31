use yew::{function_component, html, Html, Properties};
use crate::components::calculator::formula::Formula;
use crate::components::calculator::types::CalcHistory;

#[derive(Properties, PartialEq)]
pub struct FormulaHistoryProps {
  pub calc_histories: Vec<CalcHistory>,
}

#[function_component(FormulaList)]
pub fn formula_list(props: &FormulaHistoryProps) -> Html {
  html! {
    <ul class="list-group">
      <h3>{"計算履歴"}</h3>
      {props.calc_histories.iter().map(|history| html! {
        <Formula left={history.left} right={history.right} answer={history.answer} />
      }).collect::<Html>()}
    </ul>
  }
}
