use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    // この中では1つのルート要素しか許されていない
    // only one root html element is allowed (hint: you can wrap multiple html elements in a fragment `<></>`)
    html! {
        <h1>{ "Hello World" }</h1>
    }
}

fn main() {
    // yew v0.20.0 から start_app に名称変更が入りそうなので注意
    // cf.https://github.com/yewstack/yew/issues/2531
    yew::start_app::<App>();
}
