mod components;

use components::calculator::types::CalcHistory;
use yew::prelude::*;
use components::header::Header;
use components::calculator::formula_history::FormulaList;
use components::calculator::formula_form::{FormulaForm, CalcResult};

#[function_component(App)]
fn app() -> Html {
    let calc_histories = use_state(|| Vec::<CalcHistory>::new());
    let next_id = use_state(|| 1);

    let on_add = {
        let calc_histories = calc_histories.clone();

        Callback::from(move |result: CalcResult| {
            let mut current_history = (*calc_histories).clone();
            current_history.push(CalcHistory {
                id: *next_id,
                left: result.left,
                right: result.right,
                answer: result.answer,
            });
            next_id.set(*next_id + 1);
            calc_histories.set(current_history);
        })
    };

    html! {
        <>
            <Header />
            <main class="container-fluid mt-2">
                <FormulaForm {on_add} />
                <FormulaList calc_histories={(*calc_histories).clone()} />
            </main>
        </>
    }
}

fn main() {
    // yew v0.20.0 から start_app に名称変更が入りそうなので注意
    // cf.https://github.com/yewstack/yew/issues/2531
    yew::start_app::<App>();
}
