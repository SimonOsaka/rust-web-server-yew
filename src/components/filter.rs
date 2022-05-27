use web_sys::MouseEvent;
use yew::{function_component, html, use_state, Callback, Html, Properties};

type Label = String;
type Value = String;
type DefaultValue = String;
type GroupLabel = String;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct FiltersProps {
    #[prop_or(vec![])]
    pub data: Vec<(Vec<(Label, Value)>, DefaultValue, GroupLabel)>,
    #[prop_or_default]
    pub callback: Callback<Vec<String>>,
}

#[function_component(Filters)]
pub fn filters(props: &FiltersProps) -> Html {
    let FiltersProps { data, callback } = props.clone();

    let init_value = data
        .clone()
        .into_iter()
        .map(|(_, def, _)| def)
        .collect::<Vec<String>>();

    let current = use_state(|| init_value);

    let tags_html = data
        .into_iter()
        .enumerate()
        .map(|(i, (item, _, group_label))| {
            html! {
                <>
                <div class="columns">
                    <div class="column has-text-right is-1">{ group_label }{":"}</div>
                    <div class="column">
                        <div class="tags">
                    {
                        item.into_iter()
                            .map(|(label, value)| {
                                // each item
                                let onclick = {
                                    let callback = callback.clone();
                                    let value = value.clone();
                                    let current = current.clone();
                                    Callback::once(move |e: MouseEvent| {
                                        e.prevent_default();
                                        let mut current_val = (*current).clone();
                                        current_val[i] = value.clone();
                                        current.set(current_val.clone());
                                        callback.emit(current_val);
                                    })
                                };
                                if *current[i] == value.clone() {
                                    html! { <span class="tag is-primary">{ label }</span> }
                                } else {
                                    html! { <span class="tag" {onclick}>{ label }</span> }
                                }
                            })
                            .collect::<Html>()
                    }
                    </div>
                    </div>
                </div>
                </>
            }
        })
        .collect::<Html>();

    html! {
        { tags_html }
    }
}
