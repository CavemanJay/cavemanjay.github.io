use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct BfVisualizerProps {}

#[function_component]
pub fn BfVisualizer(props: &BfVisualizerProps) -> Html {
    let BfVisualizerProps {} = props;
    let tape = (1..10).collect::<Vec<_>>();

    html! {
        <div>
            <Tape {tape}/>
            <hr/>
        </div>
    }
}

#[derive(PartialEq, Properties)]
struct TapeProps {
    tape: Vec<u8>,
}

#[function_component]
fn Tape(props: &TapeProps) -> Html {
    let list = props.tape.iter().enumerate().map(|(i, val)| {
        html! {
            <li key={i}>{val}</li>
        }
    });
    html! {
        <div class="overflow-x-auto whitespace-nowrap p-5">
            <ul class="p-0 -pl-2 mt-3 list-none [&>li]:inline [&>li]:border [&>li]:p-2">
                {list.collect::<Html>()}
            </ul>
        </div>
    }
}
