use std::sync::Arc;

use gloo_events::EventListener;
use log::error;
use wasm_bindgen::JsValue;
use web_sys::{History, HtmlElement, Window};

use super::{HistoryProvider, ScrollPosition};

/// A [`HistoryProvider`] that uses the [History API] and [Location API] to integrate with the
/// browser.
///
/// [History API]: https://developer.mozilla.org/en-US/docs/Web/API/History_API
/// [Location API]: https://developer.mozilla.org/en-US/docs/Web/API/Location
///
/// # Prefix
/// This [`HistoryProvider`] supports a prefix, which allows its use for web apps not located at the
/// root of their domain.
///
/// It is up to the application developer to ensure the prefix ends at a `/`. Otherwise, the first
/// navigation from within the app will add one.
///
/// Application developers are responsible for unmounting the router or app when the prefix isn't
/// present in the current URL. If the router is rendered and a navigation is caused, the prefix
/// will be introduced to the URL.
pub struct BrowserPathHistoryProvider {
    body: HtmlElement,
    history: History,
    listener: Option<EventListener>,
    prefix: Option<String>,
    window: Window,
}

impl BrowserPathHistoryProvider {
    /// Create a new [`BrowserPathHistoryProvider`] with a prefix.
    #[must_use]
    pub fn with_prefix(prefix: String) -> Box<Self> {
        Box::new(Self {
            prefix: Some(prefix),
            ..Default::default()
        })
    }
}

impl Default for BrowserPathHistoryProvider {
    fn default() -> Self {
        let window = web_sys::window().unwrap();
        let body = window.document().unwrap().body().unwrap();
        let history = window.history().unwrap();

        Self {
            body,
            history,
            listener: Default::default(),
            prefix: Default::default(),
            window,
        }
    }
}

impl HistoryProvider for BrowserPathHistoryProvider {
    fn foreign_navigation_handler(&mut self, callback: Arc<dyn Fn() + Send + Sync>) {
        // recreate event listener
        self.listener = Some(EventListener::new(&self.window, "popstate", move |_| {
            callback()
        }));
    }

    fn current_path(&self) -> String {
        let mut p = self
            .window
            .location()
            .pathname()
            .expect("location can provide path");

        if let Some(pre) = &self.prefix {
            if p.starts_with(pre) {
                p = p.split_at(pre.len()).1.to_string();
            }
        }

        if !p.starts_with('/') {
            p = format!("/{p}");
        }

        p
    }

    fn current_prefix(&self) -> String {
        if let Some(pre) = &self.prefix {
            pre.clone()
        } else {
            String::new()
        }
    }

    fn current_query(&self) -> Option<String> {
        let mut q = self
            .window
            .location()
            .search()
            .expect("location can provide query");

        if q.starts_with('?') {
            q.remove(0);
        }

        match q.is_empty() {
            false => Some(q),
            true => None,
        }
    }

    fn go_back(&mut self) {
        self.history.back().ok();

        let ScrollPosition { x, y } = self.history.state().unwrap().into_serde().unwrap();
        self.body.set_scroll_top(y);
        self.body.set_scroll_left(x);
    }

    fn go_forward(&mut self) {
        self.history.forward().ok();

        let ScrollPosition { x, y } = self.history.state().unwrap().into_serde().unwrap();
        self.body.set_scroll_top(y);
        self.body.set_scroll_left(x);
    }

    fn push(&mut self, path: String) {
        if path.starts_with("//") {
            error!(r#"cannot navigate to paths starting with "//", path: {path}"#);
            return;
        }

        let path = if let Some(pre) = &self.prefix {
            if !path.starts_with('/') {
                format!("{pre}{path}")
            } else {
                path
            }
        } else {
            path
        };

        if self
            .history
            .push_state_with_url(
                &JsValue::from_serde(&ScrollPosition {
                    x: self.body.scroll_left(),
                    y: self.body.scroll_top(),
                })
                .unwrap(),
                "",
                Some(&path),
            )
            .is_ok()
        {
            self.body.set_scroll_top(0);
            self.body.set_scroll_left(0);
        }
    }

    fn replace(&mut self, path: String) {
        if path.starts_with("//") {
            error!(r#"cannot navigate to paths starting with "//", path: {path}"#);
            return;
        }

        let path = if let Some(pre) = &self.prefix {
            if !path.starts_with('/') {
                format!("{pre}{path}")
            } else {
                path
            }
        } else {
            path
        };

        if self
            .history
            .replace_state_with_url(
                &JsValue::from_serde(&ScrollPosition {
                    x: self.body.scroll_left(),
                    y: self.body.scroll_top(),
                })
                .unwrap(),
                "",
                Some(&path),
            )
            .is_ok()
        {
            self.body.set_scroll_top(0);
            self.body.set_scroll_left(0);
        };
    }

    fn can_external(&self) -> bool {
        true
    }

    fn external(&self, url: String) {
        self.window.location().set_href(&url).ok();
    }
}
