# Yew Toasts

A simple and easy toast notifications for Yew.

## Example

```
use yew::prelude::*;
use yew_toasts::{ToastData, Gravity, Position, show_toast_cb};

#[function_component(ToastButton)]
fn toast_button() -> Html {
    let inner_cb = Callback::from(move |()| {
        web_sys::console::log_1(&"clicked in".into());
    });
    let cb = yew_toasts::show_toast_cb().reform(move |_| yew_toasts::ToastData {
        element: html!(<ToastBanner/>),
        position: yew_toasts::Position::Left,
        gravity: yew_toasts::Gravity::Bottom,
        onclick: inner_cb.clone(),
        ..Default::default()
    });
    use_effect(move || {
        gloo_timers::callback::Interval::new(5000, move || {
            yew_toasts::show_toast(&yew_toasts::ToastData {
                element: html!(<ToastBanner/>),
                position: yew_toasts::Position::Center,
                gravity: yew_toasts::Gravity::Top,
                ..Default::default()
            })
            .expect("")
        })
        .forget();
    });
    html! {
        <>
        <button onclick={cb} class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">{"Test Toast"}</button>
        </>

    }
}

#[function_component(ToastBanner)]
fn toast_banner() -> Html {
    html! {
        <div class="bg-blue-500 text-white font-bold py-2 px-4 rounded">
            {"This is a test toast"}
        </div>
    }
}

#[function_component(Splash)]
fn relay_pool_test() -> Html {
    html! {
        <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center">
       </div>
    }
}
``` 
