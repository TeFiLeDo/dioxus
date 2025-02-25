use dioxus::prelude::*;
use log::error;

use crate::{helpers::use_router_subscription, service::RouterMessage};

/// The props for a [`GoBackButton`] or a [`GoForwardButton`].
#[derive(Debug, Props)]
pub struct HistoryButtonProps<'a> {
    /// The children to render inside the button.
    pub children: Element<'a>,
}

/// A button that acts like a browsers back button.
///
/// Only works as a descendent of a [`Router`] component, otherwise it is inactive.
///
/// The button will disable itself if it is known, that no prior history is available.
///
/// # Panic
/// - When not nested within a [`Router`], but only in debug builds.
///
/// # Example
/// ```rust
/// # use dioxus::prelude::*;
/// # use dioxus_router::prelude::*;
/// rsx! {
///     GoBackButton { "Go back" }
/// };
/// ```
///
/// [`Router`]: crate::components::Router
#[allow(non_snake_case)]
pub fn GoBackButton<'a>(cx: Scope<'a, HistoryButtonProps<'a>>) -> Element {
    // hook up to router
    let router = match use_router_subscription(&cx) {
        Some(x) => x,
        None => {
            error!(
                "`GoBackButton` can only be used as a descendent of a `Router`, will be inactive"
            );
            #[cfg(debug_assertions)]
            panic!("`GoBackButton` can only be used as a descendent of a `Router`");
            #[cfg(not(debug_assertions))]
            return None;
        }
    };
    let state = router.state.read().expect("router lock poison");
    let tx = router.tx.clone();

    let disabled = !state.can_go_back;

    cx.render(rsx! {
        button {
            disabled: "{disabled}",
            prevent_default: "onclick",
            onclick: move |_| {let _ = tx.unbounded_send(RouterMessage::GoBack);},
            &cx.props.children
        }
    })
}

/// A button that acts like a browsers forward button.
///
/// Only works as a descendent of a [`Router`] component, otherwise it is inactive.
///
/// The button will disable itself if it is known, that no later history is available.
///
/// # Panic
/// - When not nested within a [`Router`], but only in debug builds.
///
/// # Example
/// ```rust
/// # use dioxus::prelude::*;
/// # use dioxus_router::prelude::*;
/// rsx! {
///     GoForwardButton { "Go forward" }
/// };
/// ```
///
/// [`Router`]: crate::components::Router
#[allow(non_snake_case)]
pub fn GoForwardButton<'a>(cx: Scope<'a, HistoryButtonProps<'a>>) -> Element {
    // hook up to router
    let router = match use_router_subscription(&cx) {
        Some(x) => x,
        None => {
            error!(
                "`GoForwardButton` can only be used as a descendent of a `Router`, will be inactive"
            );
            #[cfg(debug_assertions)]
            panic!("`GoForwardButton` can only be used as a descendent of a `Router`");
            #[cfg(not(debug_assertions))]
            return None;
        }
    };
    let state = router.state.read().expect("router lock poison");
    let tx = router.tx.clone();

    let disabled = !state.can_go_forward;

    cx.render(rsx! {
        button {
            disabled: "{disabled}",
            prevent_default: "onclick",
            onclick: move |_| {let _ = tx.unbounded_send(RouterMessage::GoForward);},
            &cx.props.children
        }
    })
}
