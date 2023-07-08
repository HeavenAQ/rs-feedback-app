use crate::{
    api,
    store::{set_feedback_list, set_loading, set_show_alert, Store},
};

use super::feedback_item::FeedbackItem;
use gloo::console::log;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component]
pub fn FeedbackList() -> Html {
    let (store, dispatch) = use_store::<Store>();
    let feedback_list = store.feedbacks.clone();

    use_effect_with_deps(
        move |_| {
            let dispatch = dispatch.clone();
            wasm_bindgen_futures::spawn_local(async move {
                set_loading(true, dispatch.clone());
                let response = api::list_feedbacks((1, 10)).await;
                match response {
                    Ok(feedbacks) => {
                        set_loading(false, dispatch.clone());
                        set_feedback_list(feedbacks, dispatch);
                    }
                    Err(err) => {
                        set_loading(false, dispatch.clone());
                        set_show_alert(err.to_string(), dispatch);
                    }
                }
            });
        },
        store.feedbacks.clone(),
    );

    html! {
        <div>
            {
                feedback_list
                    .into_iter()
                    .map(|feedback| {
                        let key = feedback.id.to_string();
                        html!{<FeedbackItem key={key} feedback={feedback.clone()}/>}
                    }).collect::<Html>()
            }
        </div>
    }
}
