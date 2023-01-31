mod components;

use yew::prelude::*;
use components::header::Header;
use components::calculator::formula_list::FormulaList;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <Header />
            <main class="container-fluid mt-2">
                <FormulaList />
            </main>
        </>
    }
}

fn main() {
    // yew v0.20.0 から start_app に名称変更が入りそうなので注意
    // cf.https://github.com/yewstack/yew/issues/2531
    yew::start_app::<App>();
}
