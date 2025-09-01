use leptos_reactive::{Scope, create_effect, create_runtime, create_scope};
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
        ctx.provide(AppConfig {
            api_url: "https://api.example.com".to_string(),
        });

        El::new("div").component(Dashboard {}, &ctx)
    })
}

#[derive(Clone)]
struct AppConfig {
    api_url: String,
}

#[derive(Clone)]
struct DashboardSettings {
    refresh_rate: u32,
}

struct Dashboard;
impl Component for Dashboard {
    fn render(&self, ctx: &ComponentContext) -> El {
        ctx.provide(DashboardSettings { refresh_rate: 5000 });
        let app_config = ctx.inject::<AppConfig>().unwrap();

        El::new("div")
            .child(El::new("h1").text("Dashboard"))
            .child(El::new("span").text(&format!("url used: {}", app_config.api_url)))
            .component(MainPanel {}, ctx)
            .component(Sidebar {}, ctx)
    }
}

struct Sidebar;
impl Component for Sidebar {
    fn render(&self, ctx: &ComponentContext) -> El {
        let settings = ctx.inject::<DashboardSettings>().unwrap();

        El::new("div")
            .attr("class", "sidebar")
            .text(&format!("Sidebar (refresh: {}ms)", settings.refresh_rate))
    }
}

struct MainPanel;
impl Component for MainPanel {
    fn render(&self, ctx: &ComponentContext) -> El {
        let new_context = ctx.create_child();
        new_context.provide(DashboardSettings { refresh_rate: 1000 });

        El::new("div")
            .attr("class", "main-panel")
            .child(El::new("h2").text("Main Panel"))
            .component(Widget {}, &new_context)
    }
}

struct Widget;
impl Component for Widget {
    fn render(&self, ctx: &ComponentContext) -> El {
        let settings = ctx.inject::<DashboardSettings>().unwrap();

        El::new("div").text(&format!("Widget refresh rate: {}ms", settings.refresh_rate))
    }
}

struct ServiceContainer {
    services: HashMap<TypeId, Rc<dyn Any>>,
}
impl ServiceContainer {
    fn new() -> Self {
        Self {
            services: HashMap::new(),
        }
    }

    fn register<T: Any>(&mut self, service: T) {
        self.services.insert(TypeId::of::<T>(), Rc::new(service));
    }

    fn get<T: Any + Clone>(&self) -> Option<T> {
        self.services
            .get(&TypeId::of::<T>())
            .and_then(|rc| rc.downcast_ref::<T>())
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

    fn provide<T: Any>(&self, service: T) {
        self.services.borrow_mut().register(service);
    }

    fn inject<T: Any + Clone>(&self) -> Option<T> {
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
