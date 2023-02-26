use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct BfVisualizerProps {}

#[function_component]
pub fn BfVisualizer(props: &BfVisualizerProps) -> Html {
    let BfVisualizerProps {} = props;
    html! {
        <div>{"hi"}</div>
    }
}
