use web_sys::{HtmlInputElement, InputEvent};
use yew::{function_component, html, Callback, ChildrenWithProps, Html, Properties, TargetCast};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct RadioProps {
    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub value: String,
    #[prop_or(false)]
    pub check: bool,
    #[prop_or_default]
    pub callback: Callback<String>,
}

#[function_component(Radio)]
pub fn radio(props: &RadioProps) -> Html {
    let RadioProps {
        label,
        value,
        check,
        callback,
    } = props.clone();

    let oninput = {
        Callback::from(move |e: InputEvent| {
            let radio: HtmlInputElement = e.target_unchecked_into();
            callback.emit(radio.value())
        })
    };

    html! {
        <label class="radio">
            <input type="radio" {value} {oninput} checked={check} />
            { label }
        </label>
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct RadioGroupProps {
    #[prop_or_default]
    pub value: String,
    #[prop_or_default]
    pub callback: Callback<String>,
    #[prop_or_default]
    pub children: ChildrenWithProps<Radio>,
}

#[function_component(RadioGroup)]
pub fn radio_group(props: &RadioGroupProps) -> Html {
    let RadioGroupProps {
        callback,
        children,
        value,
    } = props.clone();

    let radio_group_html = {
        children
            .iter()
            .map(move |mut radio| {
                if radio.props.value == value {
                    std::rc::Rc::make_mut(&mut radio.props).check = true;
                }
                std::rc::Rc::make_mut(&mut radio.props).callback = callback.clone();
                radio
            })
            .collect::<Html>()
    };

    html! {
        <div class="control">
            { radio_group_html }
        </div>
    }
}
