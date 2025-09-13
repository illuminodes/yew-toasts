//! A simple and easy toast notifications for Yew.
#![warn(
    missing_docs,
    clippy::all,
    clippy::missing_errors_doc,
    clippy::style,
    clippy::unseparated_literal_suffix,
    clippy::pedantic,
    clippy::nursery
)]
use web_sys::wasm_bindgen::JsCast;
use yew::prelude::*;

/// The vertical position of the toast
#[derive(Clone, PartialEq, Copy, Default, Eq)]
pub enum Gravity {
    #[default]
    /// The toast will be placed at the top of the screen
    Top,
    /// The toast will be placed at the bottom of the screen
    Bottom,
}
impl AsRef<str> for Gravity {
    fn as_ref(&self) -> &str {
        match self {
            Self::Top => "top",
            Self::Bottom => "bottom",
        }
    }
}

/// The horizontal position of the toast
#[derive(Clone, PartialEq, Copy, Default, Eq)]
pub enum Position {
    /// The toast will be placed on the left side of the screen
    Left,
    /// The toast will be placed on the right side of the screen
    #[default]
    Right,
    /// The toast will be placed in the center of the screen
    Center,
}

impl AsRef<str> for Position {
    fn as_ref(&self) -> &str {
        match self {
            Self::Left => "left",
            Self::Right => "right",
            Self::Center => "center",
        }
    }
}

/// The data needed to render a toast
///
/// # Properties
///
/// - `element`: The HTML element to render inside the toast
/// - `duration`: The duration of the toast in milliseconds
/// - `close`: Whether the toast should close when clicked
/// - `gravity`: The vertical position of the toast
/// - `position`: The horizontal position of the toast
/// - `onclick`: The callback to be called when the toast is clicked
///
/// Setting `close` to `true` will override the `onclick` callback.
#[derive(Clone, PartialEq)]
pub struct ToastData {
    /// The HTML element to render inside the toast
    pub element: Html,
    /// The duration of the toast in milliseconds
    pub duration: u32,
    /// Whether the toast should close when clicked
    pub close: bool,
    /// The vertical position of the toast
    pub gravity: Gravity,
    /// The horizontal position of the toast
    pub position: Position,
    /// The callback to be called when the toast is clicked
    pub onclick: Callback<()>,
}
impl Default for ToastData {
    fn default() -> Self {
        Self {
            duration: 3000,
            close: false,
            gravity: Gravity::Top,
            position: Position::Right,
            element: html!(<div></div>),
            onclick: Callback::noop(),
        }
    }
}


/// Props for toast containers, these are used to position the toasts.
#[derive(Clone, PartialEq, Eq, Properties)]
pub struct ToastCornerContainerProps {
    /// The vertical position of the toast
    pub gravity: Gravity,
    /// The horizontal position of the toast
    pub position: Position,
}

/// A container for toasts, these holde the toasts and position them.
#[function_component(ToastCornerContainer)]
pub fn toast_corner_container(props: &ToastCornerContainerProps) -> Html {
    const fn container_style(gravity: Gravity, position: Position) -> &'static str {
        match (gravity, position) {
            (Gravity::Top, Position::Left) => {
                "position: fixed; top: 1rem; left: 1rem; display: flex; flex-direction: column; gap: 0.5rem; z-index: 9999;"
            }
            (Gravity::Top, Position::Right) => {
                "position: fixed; top: 1rem; right: 1rem; display: flex; flex-direction: column; gap: 0.5rem; z-index: 9999;"
            }
            (Gravity::Top, Position::Center) => {
                "position: fixed; top: 1rem; left: 50%; transform: translateX(-50%); display: flex; flex-direction: column; gap: 0.5rem; z-index: 9999;"
            }
            (Gravity::Bottom, Position::Left) => {
                "position: fixed; bottom: 1rem; left: 1rem; display: flex; flex-direction: column-reverse; gap: 0.5rem; z-index: 9999;"
            }
            (Gravity::Bottom, Position::Right) => {
                "position: fixed; bottom: 1rem; right: 1rem; display: flex; flex-direction: column-reverse; gap: 0.5rem; z-index: 9999;"
            }
            (Gravity::Bottom, Position::Center) => {
                "position: fixed; bottom: 1rem; left: 50%; transform: translateX(-50%); display: flex; flex-direction: column-reverse; gap: 0.5rem; z-index: 9999;"
            }
        }
    }
    let style = container_style(props.gravity, props.position);
    html! {
        <div {style} id={format!("toast-container-{}-{}", props.gravity.as_ref(), props.position.as_ref())} />
    }
}

/// A provider for toasts, renders the toast containers and 
/// add a style sheet for the keyframe animations.
#[function_component(ToastProvider)]
pub fn toast_provider() -> Html {
    html! {
        <>
            <style>
            {"
            @keyframes slideRight {
              0%   { opacity: 0; transform: translateX(100%); }
              10%  { opacity: 1; transform: translateX(0); }   /* finished entering */
              90%  { opacity: 1; transform: translateX(0); }   /* hold */
              100% { opacity: 0; transform: translateX(100%); }/* leave */
            }
            
            @keyframes slideLeft {
              0%   { opacity: 0; transform: translateX(-100%); }
              10%  { opacity: 1; transform: translateX(0); }
              90%  { opacity: 1; transform: translateX(0); }
              100% { opacity: 0; transform: translateX(-100%); }
            }
            
            @keyframes slideTop {
              0%   { opacity: 0; transform: translateY(-100%); }
              10%  { opacity: 1; transform: translateY(0); }
              90%  { opacity: 1; transform: translateY(0); }
              100% { opacity: 0; transform: translateY(-100%); }
            }
            
            @keyframes slideBottom {
              0%   { opacity: 0; transform: translateY(100%); }
              10%  { opacity: 1; transform: translateY(0); }
              90%  { opacity: 1; transform: translateY(0); }
              100% { opacity: 0; transform: translateY(100%); }
            }
            "}
            </style>
            <ToastCornerContainer gravity={Gravity::Top} position={Position::Left}/>
            <ToastCornerContainer gravity={Gravity::Top} position={Position::Right}/>
            <ToastCornerContainer gravity={Gravity::Top} position={Position::Center}/>
            <ToastCornerContainer gravity={Gravity::Bottom} position={Position::Left}/>
            <ToastCornerContainer gravity={Gravity::Bottom} position={Position::Right}/>
            <ToastCornerContainer gravity={Gravity::Bottom} position={Position::Center}/>
        </>
    }
}

#[derive(Clone, PartialEq, Properties)]
struct ToastHolderProps {
    duration: u32,
    children: Html,
    gravity: Gravity,
    position: Position,
}

#[function_component(ToastHolder)]
fn toast_holder(props: &ToastHolderProps) -> Html {
    // let animation_duration = format!("{}ms", props.duration);
    let animation = match (props.gravity, props.position) {
        (_, Position::Left) => "slideLeft",
        (_, Position::Right) => "slideRight",
        (Gravity::Top, Position::Center) => "slideTop",
        (Gravity::Bottom, Position::Center) => "slideBottom",
    };
    html! {
       <div style={format!("animation: {} {}ms ease-in-out;", animation, props.duration)}>
           {props.children.clone()}
       </div>
    }
}
/// Builds and renders a new `Toast` according to
/// the parameters passed.
///
/// # Errors
///
/// Will error if it cannot find the appropiate container already rendered.
pub fn show_toast(toast: &ToastData) -> Result<(), web_sys::wasm_bindgen::JsValue> {
    let Some(doc) = web_sys::window().and_then(|win| win.document()) else {
        return Err(web_sys::wasm_bindgen::JsValue::from_str(
            "document not found",
        ));
    };

    let container_id = format!(
        "toast-container-{}-{}",
        toast.gravity.as_ref(),
        toast.position.as_ref()
    );
    let Some(container) = doc.get_element_by_id(&container_id) else {
        return Err(web_sys::wasm_bindgen::JsValue::from_str(
            "container not found",
        ));
    };

    let div = doc
        .create_element("div")?
        .dyn_into::<web_sys::HtmlElement>()?;
    let div_clone = div.clone();
    if toast.close {
        div.set_onclick(Some(
            web_sys::wasm_bindgen::closure::Closure::once_into_js(
                move |_: web_sys::HtmlElement| {
                    div_clone.remove();
                },
            )
            .unchecked_ref(),
        ));
    } else {
        let cb = toast.onclick.clone();
        let closure = web_sys::wasm_bindgen::closure::Closure::wrap(Box::new(move || {
            cb.emit(());
        }) as Box<dyn Fn()>);
        let function = closure.as_ref().clone().unchecked_into();
        closure.forget();
        div.set_onclick(Some(&function));
    }

    let div_clone = div.clone();
    div.set_onanimationend(Some(
        web_sys::wasm_bindgen::closure::Closure::once_into_js(move |_: web_sys::HtmlElement| {
            div_clone.remove();
        })
        .unchecked_ref(),
    ));

    container.append_child(&div)?;
    yew::Renderer::<ToastHolder>::with_root_and_props(
        div.unchecked_into(),
        ToastHolderProps {
            children: toast.element.clone(),
            duration: toast.duration,
            gravity: toast.gravity,
            position: toast.position,
        },
    )
    .render();
    Ok(())
}
/// Reusable Yew `Callback` that can be reformed
/// and reused betwen different elements.
#[must_use]
pub fn show_toast_cb() -> Callback<ToastData> {
    Callback::from(move |toast: ToastData| {
        if let Err(e) = show_toast(&toast) {
            web_sys::console::error_1(&e);
        }
    })
}

