use frontend::components::alert::{AlertComponent, Props as AlertProps};
use frontend::components::feedback::feedback_form::FeedbackForm;
use frontend::components::feedback::feedback_stats::FeedbackStats;
use frontend::store::Store;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component]
fn App() -> Html {
    let (store, _) = use_store::<Store>();
    let message = store.alert_input.alert_message.clone();
    let show_alert = store.alert_input.show_alert;
    let loading = store.loading;
    let alert_props = AlertProps {
        message,
        delay_ms: 5000,
    };

    html! {
        <>
            <AlertComponent
                message={alert_props.message}
                delay_ms={alert_props.delay_ms}
            />

            <main class="md:container mt-24 px-5">
                <FeedbackForm/>
                <FeedbackStats/>
            </main>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
