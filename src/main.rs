use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <h1 class="text-3xl font-bold underline">{ "Hello World" }</h1>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
