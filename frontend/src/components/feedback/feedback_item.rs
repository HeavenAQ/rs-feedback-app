use crate::{
    api,
    store::{self, set_loading, set_show_alert, Store},
};
use common::Feedback;
use yew::{platform::spawn_local, prelude::*};
use yewdux::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub feedback: Feedback,
}

fn confirm_delete(message: &str) -> bool {
    web_sys::Window::confirm_with_message(&web_sys::window().unwrap(), message).unwrap()
}

#[function_component]
pub fn FeedbackItem(props: &Props) -> Html {
    let (_, dispatch) = use_store::<Store>();
    let on_delete = {
        let cloned_dispatch = dispatch.clone();
        let feedback_id = props.feedback.id.clone();
        Callback::from(move |_e: MouseEvent| {
            let dispatch = cloned_dispatch.clone();
            let confirm = confirm_delete("Do you really want to delete this feedback?");

            if confirm {
                spawn_local(async move {
                    set_loading(true, dispatch.clone());
                    let response = api::delete_feedback(&feedback_id.to_string()).await;
                    match response {
                        Ok(_res) => {
                            set_loading(true, dispatch.clone());
                            set_show_alert(
                                "Feedback was deleted successfully!".to_owned(),
                                dispatch.clone(),
                            );
                            store::delete_feedback(feedback_id, dispatch);
                        }
                        Err(err) => {
                            set_loading(false, dispatch.clone());
                            set_show_alert(err.to_string(), dispatch)
                        }
                    }
                })
            }
        })
    };

    html! {
        <div class="bg-white text-gray-700 rounded-lg p-8 my-5 relative">
            <div class="bg-pink-500 text-white rounded-full border-2 border-gray-200 w-12 h-12 flex items-center justify-center text-2xl font-bold absolute top-0 left-0 -mt-4 -ml-4">
                {props.feedback.rating}
            </div>
            <button class="absolute font-bold top-2 right-4 cursor-pointer bg-transparent border-none" onclick={on_delete}>{"X"}</button>
            <p>
                {&props.feedback.text}
            </p>
        </div>
    }
}
