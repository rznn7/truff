use leptos_reactive::{
    Scope, SignalGet, SignalUpdate, create_effect, create_runtime, create_scope, create_signal,
};
use std::ops::Deref;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{self, Element, Event, console, window};

fn main() {
    mount(|cx| {
        let (count, set_count) = create_signal(cx, 0);
        El::new("div")
            .child(
                El::new("button")
                    .on("click", move |_| set_count.update(|n| *n -= 1))
                    .attr("id", "my-button")
                    .text("-1"),
            )
            .text(" Value: ")
            .dyn_text(cx, move || count.get().to_string())
            .child(
                El::new("button")
                    .on("click", move |_| set_count.update(|n| *n += 1))
                    .attr("id", "my-button")
                    .text("+1"),
            )
    })
}

fn mount(f: impl FnOnce(Scope) -> El + 'static) {
    let runtime = create_runtime();
    _ = create_scope(runtime, |cx| {
        let window = window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let root = f(cx);

        body.append_child(&root).unwrap();
    });
}

#[derive(Debug, Clone)]
pub struct El(Element);

impl Deref for El {
    type Target = Element;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl El {
    pub fn new(tag_name: &str) -> Self {
        let window = window().unwrap();
        let document = window.document().unwrap();
        let el = document.create_element(tag_name).unwrap();

        Self(el)
    }

    fn on(self, event_name: &str, callback: impl FnMut(Event) + 'static) -> Self {
        use wasm_bindgen::prelude::Closure;
        let callback = Closure::wrap(Box::new(callback) as Box<dyn FnMut(Event)>);
        self.0
            .add_event_listener_with_callback(event_name, callback.as_ref().unchecked_ref())
            .unwrap();
        callback.forget();
        self
    }

    fn attr(self, attr: &str, val: &str) -> Self {
        self.0.set_attribute(attr, val).unwrap();
        self
    }

    fn text(self, data: &str) -> Self {
        let window = window().unwrap();
        let document = window.document().unwrap();
        let node = document.create_text_node(data);
        self.0.append_child(&node).unwrap();
        self
    }

    fn child(self, node: El) -> Self {
        self.append_child(&node).unwrap();
        self
    }

    fn dyn_text(self, cx: Scope, f: impl Fn() -> String + 'static) -> Self {
        let window = window().unwrap();
        let document = window.document().unwrap();
        let node = document.create_text_node("");

        self.0.append_child(&node).unwrap();

        create_effect(cx, move |_| {
            let value = f();
            node.set_data(&value);
        });
        self
    }
}
