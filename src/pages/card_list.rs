use crate::components::card::Card;
use yew::{function_component, html};

#[function_component(CardList)]
pub fn card_list() -> Html {
    html! {
        <div class="columns">
            <div class="column">
                <Card/>
            </div>
            <div class="column">
                <Card/>
            </div>
            <div class="column">
                <Card/>
            </div>
        </div>
    }
}
