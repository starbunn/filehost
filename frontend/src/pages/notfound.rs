use yew::prelude::*;

#[function_component(NotFound)]
pub fn notfound() -> Html {
    html! {
        <div>
            <h1>{404}</h1>
        </div>
    }
}
