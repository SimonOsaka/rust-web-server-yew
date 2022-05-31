use web_sys::{MouseEvent, HtmlButtonElement};
use yew::{classes, function_component, html, Callback, Children, ChildrenWithProps, Properties, NodeRef, Html};

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
    let tab_container_children = {
        children.iter().map(|mut tab| {
            if !current.is_empty() && *current == tab.props.name {
                std::rc::Rc::make_mut(&mut tab.props).active = true;
            }
            tab
        }).collect::<Html>()
    };

    // tab html
    let children = children.clone();
    let tab_html = {
        children.iter().map(move|tab| {
            let onclick_delete = {
                let callback_delete = callback_delete.clone();
                let tab = tab.props.name.clone();
                Callback::once(move |e: MouseEvent| {
                    e.prevent_default();
                    callback_delete.emit(tab)
                })
            };

            let el_ref = NodeRef::default();

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

                let onmouseover_delete = {
                    let el_ref=el_ref.clone();
                    Callback::from(move |e: MouseEvent| {
                        e.prevent_default();
                        gloo_console::log!("tab onmouseover ...");
                        if let Some(btn) =el_ref.cast::<HtmlButtonElement>(){
                            btn.set_class_name("delete is-small ml-1 fade-in");
                        }
                    })
                };

                let onmouseout_delete = {
                    let el_ref = el_ref.clone();
                    Callback::from(move |e: MouseEvent| {
                        e.prevent_default();
                        gloo_console::log!("tab onmouseout ...");
                        if let Some(btn) = el_ref.cast::<HtmlButtonElement>(){
                            btn.set_class_name("delete is-small ml-1 fade-out");
                        }
                    })
                };

                let onanimationend = {
                    let el_ref = el_ref.clone();
                    Callback::from(move |_| {
                        if let Some(btn) = el_ref.cast::<HtmlButtonElement>(){
                            let class_name = btn.class_name();
                            gloo_console::log!("tabs onanimationend ...", class_name.clone());
                            if class_name.contains("fade-out") {
                                btn.set_class_name("delete is-small ml-1 is-invisible");
                            }
                        }
                    })
                };


                html! { 
                    <li onmouseover={onmouseover_delete} onmouseout={onmouseout_delete}>
                        <a onclick={onclick_tab}>{ tab.props.label.clone() }<button ref={el_ref} class="delete is-small ml-1 is-invisible" onclick={onclick_delete} {onanimationend}></button>
                        </a>
                    </li>
                }
            }
        }).collect::<Html>()
    };

    html! {
        <>
            <div class="tabs is-boxed">
                <ul>
                    { tab_html }
                </ul>
            </div>
            <div class="container">
                { tab_container_children }
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
