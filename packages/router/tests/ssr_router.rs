#![allow(non_snake_case)]

use dioxus::{
    prelude::*,
    router::history::{
        ControlledHistoryProvider, HistoryController, HistoryProvider, MemoryHistoryProvider,
    },
};

#[test]
fn generates_without_error() {
    let (mut controller, history) =
        HistoryController::new(Box::new(MemoryHistoryProvider::default()));

    controller.replace(String::from("/other"));

    let mut app = VirtualDom::new_with_props(App, AppProps { history });
    app.rebuild();

    let out = dioxus::ssr::render_vdom(&app);

    assert_eq!(out, "<nav>navbar</nav><h1>Other</h1>");
}

#[derive(Props)]
struct AppProps {
    history: ControlledHistoryProvider,
}

impl PartialEq for AppProps {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}

fn App<'a>(cx: Scope<AppProps>) -> Element {
    let routes = use_segment(&cx, || {
        Segment::default()
            .index(RcComponent(Home))
            .fixed("other", Route::new(RcComponent(Other)))
    });
    let history = cx.use_hook(|_| {
        let history = cx.props.history.clone();

        return move || {
            let x: Box<dyn HistoryProvider> = Box::new(history.clone());
            x
        };
    });

    cx.render(rsx! {
        Router {
            init_only: true,
            history: history,
            routes: routes.clone(),
            nav { "navbar" }
            Outlet {}
        }
    })
}

fn Home(cx: Scope) -> Element {
    cx.render(rsx! { h1 { "Home" } })
}

fn Other(cx: Scope) -> Element {
    cx.render(rsx! { h1 { "Other" }})
}
