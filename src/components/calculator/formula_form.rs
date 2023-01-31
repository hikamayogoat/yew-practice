use yew::{function_component, html, use_state, Callback, InputEvent, UseStateHandle, Properties, MouseEvent};

#[derive(Properties, PartialEq)]
pub struct CalcResult {
  pub left: usize,
  pub right: usize,
  pub answer: usize,
}

#[derive(Properties, PartialEq)]
pub struct FormulaFormProps {
  pub on_add: Callback<CalcResult>
}


// Callback関数を返す関数
// TODO: 参照で受け取った form_number の clone を書き換えて、なぜもとの state が書き換わるか不明
fn oninput_base(form_number: &UseStateHandle<usize>) -> Callback<InputEvent> {
  let number = form_number.clone();

  Callback::from(move |e: InputEvent| { // move をつけることで number の所有権を奪って state を書き換える
    let value = e.data();

    match value {
      Some(value) => {
        // <input type="number"> によって型の変換を安全に行える（はず）
        let input_num: usize = (&value).parse().unwrap();
        number.set(input_num);
      }
      None => {
        number.set(0);
      }
    }
  })
}

#[function_component(FormulaForm)]
pub fn formula_form(props: &FormulaFormProps) -> Html {
  let left = use_state(|| 0 as usize);
  let right = use_state(|| 0 as usize);

  let onclick = {
    let on_add = props.on_add.clone();
    let left = left.clone();
    let right = right.clone();

    Callback::from(move |e: MouseEvent| {
      e.prevent_default();
      let calc_result = CalcResult {
        left: (*left).clone(),
        right: (*right).clone(),
        answer: (*left).clone() + (*right).clone(),
      };
      left.set(0);
      right.set(0);
      on_add.emit(calc_result);
    })
  };

  html! {
    <>
      <form class="mb-5">
        <div class="row">
          <div class="col-auto">
            <input type="number" value={(*left).clone().to_string()} oninput={oninput_base(&left)} class="form-control" placeholder="Left number" />
          </div>
          <div class="col-auto">
            <h3>{"+"}</h3>
          </div>
          <div class="col-auto">
            <input type="number" value={(*right).clone().to_string()} oninput={oninput_base(&right)} class="form-control" placeholder="Right number" />
          </div>
          <div class="col-auto">
            <button type="submit" {onclick} class="btn btn-primary">{"計算"}</button>
          </div>
        </div>
      </form>
      <div class="mb-3">
        {"計算しようとしている："}{&(*left)}{"+"}{&(*right)}
      </div>
    </>
  }
}