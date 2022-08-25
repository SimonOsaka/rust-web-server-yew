use rust_web_server_yew::components::chart::LineChart;
use web_sys::MouseEvent;
use yew::{function_component, html, use_state, Callback};

const LABELS: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

#[function_component(ChartPage)]
pub fn chart_page() -> Html {
    let label: Vec<String> = LABELS.into_iter().map(|m| m.to_string()).collect();
    let data = use_state(|| vec![70, 65, 60, 55, 40, 35, 30, 25, 20, 15, 10, 5]);
    let onclick_linechart = {
        let data = data.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            data.set(generate_data());
        })
    };
    html! {
        <>
            <button class="button is-primary" onclick={onclick_linechart}>{ "update" }</button>
            <LineChart  label={label.clone()} data={(*data).clone()}/>
            <LineChart  label={label} data={(*data).clone()} width={800} height={400}/>
        </>
    }
}

fn generate_data() -> Vec<i32> {
    let mut a = [0i32; 12];
    random_number::random_fill!(a, 0..=100);
    println!("{:?}", a);
    a.to_vec()
}
