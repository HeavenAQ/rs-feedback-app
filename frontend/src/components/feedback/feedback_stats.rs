use yew::prelude::*;
use yewdux::prelude::*;

use crate::store::Store;

#[function_component]
pub fn FeedbackStats() -> Html {
    let (store, _) = use_store::<Store>();
    let count = store.feedbacks.len();
    let sum = store
        .feedbacks
        .iter()
        .map(|feedback| u32::from(feedback.rating))
        .sum::<u32>();

    let average = sum.checked_div(count as u32).unwrap_or(0);

    html! {
    <div class="flex justify-between items-center mb-11">
        <h4 class="text-white">{count} {" "} {"Reviews"}</h4>
        <h4 class="text-white">{"Ratings Average: "} {format!("{:.2}", average as f32)}</h4>
    </div>}
}
