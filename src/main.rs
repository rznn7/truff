use leptos_reactive::{
    ReadSignal, Scope, SignalGet, SignalSet, SignalUpdate, WriteSignal, create_effect,
    create_runtime, create_scope, create_signal,
};
use std::{
    any::{Any, TypeId},
    cell::RefCell,
    collections::HashMap,
    ops::Deref,
    rc::Rc,
};
use wasm_bindgen::JsCast;
use web_sys::{self, Element, Event, window};

fn main() {
    mount(|cx| {
        let ctx = ComponentContext::new(cx);
        El::new("div").component(AppComponent {}, &ctx)
    })
}

// ============= Services =============

struct CounterService {
    count: (ReadSignal<i32>, WriteSignal<i32>),
    total_clicks: (ReadSignal<i32>, WriteSignal<i32>),
}

impl CounterService {
    fn new(cx: Scope) -> Self {
        Self {
            count: create_signal(cx, 0),
            total_clicks: create_signal(cx, 0),
        }
    }

    fn increment(&self) {
        self.count.1.update(|n| *n += 1);
        self.total_clicks.1.update(|n| *n += 1);
    }

    fn decrement(&self) {
        self.count.1.update(|n| *n = n.saturating_sub(1));
        self.total_clicks.1.update(|n| *n += 1);
    }

    fn reset(&self) {
        self.count.1.set(0);
    }

    fn get_count_signal(&self) -> ReadSignal<i32> {
        self.count.0
    }
}

// ============= Components =============

struct AppComponent;
impl Component for AppComponent {
    fn render(&self, ctx: &ComponentContext) -> El {
        ctx.provide(CounterService::new(ctx.scope));

        El::new("div")
            .attr("style", "padding: 20px; font-family: sans-serif;")
            .component(Dashboard {}, ctx)
    }
}
struct Dashboard;
impl Component for Dashboard {
    fn render(&self, ctx: &ComponentContext) -> El {
        El::new("div")
            .child(El::new("h1").text("Angular-like Services Demo"))
            .child(El::new("hr"))
            .component(ControlPanel {}, ctx)
            .component(DisplayPanel {}, ctx)
            .component(OtherCounterProvider {}, ctx)
    }
}

struct ControlPanel;
impl Component for ControlPanel {
    fn render(&self, ctx: &ComponentContext) -> El {
        let counter_service = ctx.inject::<CounterService>().unwrap();
        let counter_service_inc = counter_service.clone();
        let counter_service_dec = counter_service.clone();
        let counter_service_reset = counter_service.clone();

        El::new("div")
            .attr("style", "margin: 20px 0; padding: 15px; background: #f0f0f0; border-radius: 8px;")
            .child(El::new("h2").text("Control Panel"))
            .child(
                El::new("button")
                    .attr("style", "margin: 5px; padding: 10px 20px; font-size: 16px; cursor: pointer;")
                    .text("Increment (+1)")
                    .on("click", move |_| {
                        counter_service_inc.borrow().increment();
                    })
            )
            .child(
                El::new("button")
                    .attr("style", "margin: 5px; padding: 10px 20px; font-size: 16px; cursor: pointer;")
                    .text("Decrement (-1)")
                    .on("click", move |_| {
                        counter_service_dec.borrow().decrement();
                    })
            )
            .child(
                El::new("button")
                    .attr("style", "margin: 5px; padding: 10px 20px; font-size: 16px; cursor: pointer; background: #ff6b6b; color: white; border: none; border-radius: 4px;")
                    .text("Reset")
                    .on("click", move |_| {
                        counter_service_reset.borrow().reset();
                    })
            )
    }
}

struct DisplayPanel;
impl Component for DisplayPanel {
    fn render(&self, ctx: &ComponentContext) -> El {
        let counter_service = ctx.inject::<CounterService>().unwrap();
        let count_signal = counter_service.borrow().get_count_signal();
        let counter_service_clicks = counter_service.clone();

        El::new("div")
            .attr(
                "style",
                "margin: 20px 0; padding: 15px; background: #e8f4f8; border-radius: 8px;",
            )
            .child(El::new("h2").text("Display Panel (Reactive)"))
            .child(
                El::new("div")
                    .attr("style", "font-size: 24px; margin: 10px 0;")
                    .child(El::new("span").text("Current Count: "))
                    .child(
                        El::new("span")
                            .attr("style", "font-weight: bold; color: #2196F3;")
                            .dyn_text(ctx.scope, move || count_signal.get().to_string()),
                    ),
            )
            .child(
                El::new("div")
                    .attr("style", "font-size: 14px; color: #666;")
                    .dyn_text(ctx.scope, move || {
                        format!(
                            "Total button clicks: {}",
                            counter_service_clicks.borrow().total_clicks.0.get()
                        )
                    }),
            )
            .component(NestedCounter, ctx)
    }
}

struct OtherCounterProvider;
impl Component for OtherCounterProvider {
    fn render(&self, ctx: &ComponentContext) -> El {
        let ctx = ctx.create_child();
        ctx.provide(CounterService::new(ctx.scope));

        El::new("div")
            .attr(
                "style",
                "margin: 20px 0; padding: 15px; background: #e19fff; border-radius: 8px;",
            )
            .child(El::new("h2").text("Another counter"))
            .child(El::new("p").text("This component provides and uses its own counter instance"))
            .component(NestedCounter {}, &ctx)
    }
}

struct NestedCounter;
impl Component for NestedCounter {
    fn render(&self, ctx: &ComponentContext) -> El {
        let counter_service = ctx.inject::<CounterService>().unwrap();
        let count_signal = counter_service.borrow().get_count_signal();
        let counter_service_inc = counter_service.clone();

        El::new("div")
            .attr("style", "display: flex; align-items: center; gap: 10px;")
            .child(
                El::new("button")
                    .attr("style", "padding: 5px 15px; cursor: pointer;")
                    .text("Nested +1")
                    .on("click", move |_| {
                        counter_service_inc.borrow().increment();
                    }),
            )
            .child(
                El::new("span")
                    .attr("style", "font-size: 18px;")
                    .text("Count from nested: "),
            )
            .child(
                El::new("span")
                    .attr(
                        "style",
                        "font-size: 18px; font-weight: bold; color: #ff9800;",
                    )
                    .dyn_text(ctx.scope, move || count_signal.get().to_string()),
            )
    }
}

// ============= Framework Core =============

struct ServiceContainer {
    services: HashMap<TypeId, Rc<dyn Any>>,
}

impl ServiceContainer {
    fn new() -> Self {
        Self {
            services: HashMap::new(),
        }
    }

    fn register<T: Any + 'static>(&mut self, service: T) {
        self.services.insert(
            TypeId::of::<T>(),
            Rc::new(Rc::new(RefCell::new(service))) as Rc<dyn Any>,
        );
    }

    fn get<T: Any + 'static>(&self) -> Option<Rc<RefCell<T>>> {
        self.services
            .get(&TypeId::of::<T>())
            .and_then(|rc| rc.downcast_ref::<Rc<RefCell<T>>>())
            .cloned()
    }
}

struct ComponentContext {
    scope: Scope,
    services: Rc<RefCell<ServiceContainer>>,
}

impl ComponentContext {
    fn new(scope: Scope) -> Self {
        Self {
            scope,
            services: Rc::new(RefCell::new(ServiceContainer::new())),
        }
    }

    fn provide<T: Any + 'static>(&self, service: T) {
        self.services.borrow_mut().register(service);
    }

    fn inject<T: Any + 'static>(&self) -> Option<Rc<RefCell<T>>> {
        self.services.borrow().get::<T>()
    }

    fn create_child(&self) -> Self {
        let mut new_container = ServiceContainer::new();

        for (type_id, service_rc) in &self.services.borrow().services {
            new_container.services.insert(*type_id, service_rc.clone());
        }

        Self {
            scope: self.scope,
            services: Rc::new(RefCell::new(new_container)),
        }
    }
}

trait Component {
    fn on_init(&mut self, _ctx: &ComponentContext) {}
    fn render(&self, _cx: &ComponentContext) -> El;
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
struct El(Element);

impl El {
    fn new(tag_name: &str) -> Self {
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

    fn component(self, mut component: impl Component, ctx: &ComponentContext) -> Self {
        component.on_init(ctx);
        let el = component.render(ctx);
        self.0.append_child(&el).unwrap();

        self
    }
}

impl Deref for El {
    type Target = Element;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
