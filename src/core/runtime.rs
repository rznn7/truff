use leptos_reactive::{create_runtime, create_scope};
use web_sys::window;

use crate::core::{
    component::{Component, ComponentContext},
    el::El,
};

pub fn start_app<T: Component + 'static>(root_component: T) {
    let runtime = create_runtime();
    let _ = create_scope(runtime, |cx| {
        let window = window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let ctx = ComponentContext::new(cx);
        let root_el = El::new("div").component(root_component, &ctx);

        body.append_child(&root_el).unwrap();
    });
}
