use yew::{Properties, function_component, html};

#[derive(Properties, PartialEq)]
pub struct FormulaProps {
  pub left: usize,
  pub right: usize,
  pub answer: usize,
}

#[function_component(Formula)]
pub fn formula(props: &FormulaProps) -> Html {
  html! {
    <li class="list-group-item">
      {&props.left}{"+"}{&props.right}{"="}{&props.answer}
    </li>
  }
}