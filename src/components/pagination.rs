use yew::{classes, function_component, html, Callback, Html, MouseEvent, Properties};

const DEFAULT_CURRENT: usize = 1;
const DEFAULT_SIZE: usize = 10;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct PaginationProps {
    #[prop_or(DEFAULT_CURRENT)]
    pub current: usize,
    #[prop_or(DEFAULT_SIZE)]
    pub size: usize,
    #[prop_or(0)]
    pub total: usize,
    pub callback: Callback<usize>,
}
#[function_component(Pagination)]
pub fn pagination(props: &PaginationProps) -> Html {
    let PaginationProps {
        current,
        size,
        total,
        callback,
    } = props.clone();

    let total_page = if total < size {
        1
    } else if total % size == 0 {
        total / size
    } else {
        (total / size) + 1
    };

    let current_page = if current > total_page {
        total_page
    } else {
        current
    };

    // previous
    let previous_html = if current_page > 1 {
        let callback = callback.clone();
        let onclick_page_previous = Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            gloo_console::log!("click previous");
            callback.emit(current_page - 1)
        });
        html! { <a class="pagination-previous" onclick={onclick_page_previous}>{"Previous"}</a> }
    } else {
        html! { <a class="pagination-previous is-disabled" disabled=true>{"Previous"}</a> }
    };

    // next
    let next_html = if current_page == total_page {
        html! { <a class="pagination-next is-disabled" disabled=true>{"Next page"}</a> }
    } else {
        let callback = callback.clone();
        let onclick_page_next = Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            gloo_console::log!("click next");
            callback.emit(current_page + 1)
        });
        html! { <a class="pagination-next" onclick={onclick_page_next}>{"Next page"}</a> }
    };

    // 1
    let page_1 = if current_page == 1 {
        html! { <li><a class={classes!("pagination-link", "is-current")} aria-label="Page 1" aria-current="page">{"1"}</a></li> }
    } else {
        let callback = callback.clone();
        let onclick_page_first = Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            gloo_console::log!("click 1");
            callback.emit(1)
        });
        html! { <li><a class="pagination-link" onclick={onclick_page_first} aria-label="Goto page 1">{"1"}</a></li> }
    };

    // 1 ...
    let page_1_ellipsis = if current_page > 4 && total_page > 7 {
        html! { <li><span class="pagination-ellipsis">{"…"}</span></li> }
    } else {
        html! {}
    };

    // current_page - 2, current_page - 1, current_page, current_page + 1, current_page + 2
    let page_range = if total_page < 8 {
        2..total_page
    } else if current_page <= 3 {
        2..6
    } else if current_page + 2 > total_page {
        (total_page - 4)..(total_page)
    } else if current_page + 2 == total_page {
        (current_page - 2)..(current_page + 2)
    } else if current_page + 1 == total_page {
        (current_page - 3)..(current_page + 1)
    } else {
        (current_page - 2)..(current_page + 3)
    };
    let pages = page_range.into_iter().map(|idx| {
        if idx == current_page {
            html! { 
                <li>
                    <a class={classes!("pagination-link", "is-current" )} aria-label={format!("Page {}", idx)}
                        aria-current="page">{idx}</a>
                </li>
            }
        } else {
            let callback = callback.clone();
            let onclick_page = Callback::from(move |e: MouseEvent| {
                e.prevent_default();
                gloo_console::log!(format!("click {}", idx));
                callback.emit(idx)
            });
            html! { 
                <li>
                    <a class="pagination-link" onclick={onclick_page} aria-label={format!("Goto page {}",
                        idx)}>{idx}</a>
                </li>
            }
        }
    }).collect::<Html>();

    // ... last
    let page_ellipsis_last = if current_page + 4 <= total_page && total_page > 7 {
        html! { <li><span class="pagination-ellipsis">{"…"}</span></li> }
    } else {
        html! {}
    };

    // last
    let page_last = if total_page > 1 {
        if current_page == total_page {
            html! {
                <li>
                    <a class={classes!("pagination-link", "is-current" )} aria-label={format!("Page {}", total_page)}
                        aria-current="page">{total_page}</a>
                </li>
            }
        } else {
            let onclick_page_last = Callback::from(move |e: MouseEvent| {
                e.prevent_default();
                gloo_console::log!("click last");
                callback.emit(total_page)
            });
            html! {
                <li>
                    <a class="pagination-link" onclick={onclick_page_last} aria-label={format!("Goto page {}",
                        total_page)}>{total_page}</a>
                </li>
            }
        }
    } else {
        html! {}
    };

    html! {
        <nav class="pagination" role="navigation" aria-label="pagination">
            { previous_html }
            { next_html }
            <ul class="pagination-list">
                { page_1 }
                { page_1_ellipsis }
                { pages }
                { page_ellipsis_last }
                { page_last }
            </ul>
        </nav>
    }
}
