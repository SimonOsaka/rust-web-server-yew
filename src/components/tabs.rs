use web_sys::MouseEvent;
use yew::{classes, function_component, html, Callback, Children, ChildrenWithProps, Properties};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct TabsProps {
    #[prop_or("".to_string())]
    pub current: String,
    #[prop_or_default]
    pub children: ChildrenWithProps<Tab>,
    pub callback: Callback<String>,
    pub callback_delete: Callback<String>,
}

#[function_component(Tabs)]
pub fn tabs(props: &TabsProps) -> Html {
    let TabsProps {
        current,
        callback,
        children,
        callback_delete,
    } = props;

    // pass children value
    let children = children.clone();
    let tab_children = {
        children.iter().map(|mut tab| {
            if !current.is_empty() && *current == tab.props.name {
                std::rc::Rc::make_mut(&mut tab.props).active = true;
            }
            tab
        })
    };

    // tab html
    let children = children.clone();
    let li_html = {
        children.iter().map(| tab| {
            let onclick_delete = {
                let callback_delete = callback_delete.clone();
                let tab = tab.props.name.clone();
                Callback::once(move |e: MouseEvent| {
                    e.prevent_default();
                    callback_delete.emit(tab)
                })
            };

            if !current.is_empty() && *current == tab.props.name {
                html! {
                    <li class="is-active">
                        <a>{ tab.props.label.clone() }<button class="delete is-small ml-1" onclick={onclick_delete}></button>
                        </a>
                    </li>
                }
            } else {
                let callback = callback.clone();
                let onclick_tab = {
                    let tab = tab.props.name.clone();
                    Callback::once(move |e: MouseEvent| {
                        e.prevent_default();
                        callback.emit(tab)
                    })
                };

                html! { 
                    <li>
                        <a onclick={onclick_tab}>{ tab.props.label.clone() }<button class="delete is-small ml-1"
                                onclick={onclick_delete}></button>
                        </a>
                    </li>
                }
            }
        })
    };

    html! {
        <>
            <div class="tabs is-boxed">
                <ul>
                    { for li_html }
                </ul>
            </div>
            <div class="container">
                { for tab_children }
            </div>
        </>
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct TabProps {
    pub label: String,
    pub name: String,
    #[prop_or(false)]
    pub active: bool,
    #[prop_or_default]
    pub children: Children,
}
#[function_component(Tab)]
pub fn tab(props: &TabProps) -> Html {
    gloo_console::log!(format!("{} active: {}", props.label, props.active));
    let classes = if !props.active {
        classes!("is-hidden")
    } else {
        classes!("")
    };

    html! { <div class={classes}>{ props.children.clone() }</div> }
}
