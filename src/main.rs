use leptos_reactive::{
    Scope, SignalGet, SignalUpdate, create_effect, create_runtime, create_scope, create_signal,
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

        ctx.provide(ThemeService {
            primary_color: "blue".to_string(),
            font_size: "16px".to_string(),
        });

        ctx.provide(UserService {
            username: "alice".to_string(),
            is_admin: true,
        });

        El::new("div")
            .component(
                ThemedButton {
                    label: "Click me".to_string(),
                },
                &ctx,
            )
            .component(UserProfile {}, &ctx)
    })
}

#[derive(Clone)]
struct ThemeService {
    primary_color: String,
    font_size: String,
}

impl Default for ThemeService {
    fn default() -> Self {
        Self {
            primary_color: "red".to_string(),
            font_size: "12px".to_string(),
        }
    }
}

#[derive(Clone)]
struct UserService {
    username: String,
    is_admin: bool,
}

struct ThemedButton {
    label: String,
}

impl Component for ThemedButton {
    fn render(&self, ctx: &ComponentContext) -> El {
        let theme = ctx.inject::<ThemeService>().unwrap_or_default();

        El::new("button")
            .attr(
                "style",
                &format!(
                    "color: {}; font-size: {}",
                    theme.primary_color, theme.font_size
                ),
            )
            .text(&self.label)
    }
}

struct UserProfile;
impl Component for UserProfile {
    fn render(&self, ctx: &ComponentContext) -> El {
        let user = ctx.inject::<UserService>();

        match user {
            Some(u) => {
                El::new("div").text(&format!("User: {} (admin: {})", u.username, u.is_admin))
            }
            None => El::new("div").text("Not logged in"),
        }
    }
}

struct LoggingCounter {
    initial: i32,
    name: String,
}

impl Component for LoggingCounter {
    fn on_init(&mut self, ctx: &ComponentContext) {
        web_sys::console::log_1(
            &format!("{} initialized with value {}", self.name, self.initial).into(),
        );
        self.initial += 1;
    }

    fn render(&self, ctx: &ComponentContext) -> El {
        El::new("div")
            .child(El::new("div").text(&self.name))
            .child(El::new("div").text(&self.initial.to_string()))
    }
}

struct ServiceContainer {
    services: HashMap<TypeId, Box<dyn Any>>,
}

impl ServiceContainer {
    fn new() -> Self {
        Self {
            services: HashMap::new(),
        }
    }

    fn register<T: Any>(&mut self, service: T) {
        self.services.insert(TypeId::of::<T>(), Box::new(service));
    }

    fn get<T: Any + Clone>(&self) -> Option<T> {
        self.services
            .get(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_ref::<T>())
            .cloned()
    }
}

struct ComponentContext {
    scope: Scope,
    services: Rc<RefCell<ServiceContainer>>, // Shared container
}

impl ComponentContext {
    fn scope(&self) -> Scope {
        self.scope
    }

    fn provide<T: Any>(&self, service: T) {
        self.services.borrow_mut().register(service);
    }

    fn inject<T: Any + Clone>(&self) -> Option<T> {
        self.services.borrow().get::<T>()
    }
}

impl ComponentContext {
    fn new(scope: Scope) -> Self {
        Self {
            scope,
            services: Rc::new(RefCell::new(ServiceContainer::new())),
        }
    }
}

struct CounterComponent {
    initial: i32,
}

impl Component for CounterComponent {
    fn render(&self, ctx: &ComponentContext) -> El {
        let (count, set_count) = create_signal(ctx.scope(), self.initial);
        El::new("div")
            .child(El::new("div").dyn_text(ctx.scope(), move || count.get().to_string()))
            .child(
                El::new("div")
                    .child(
                        El::new("button")
                            .on("click", move |_| set_count.update(move |n| *n -= 1))
                            .text("-"),
                    )
                    .child(
                        El::new("button")
                            .on("click", move |_| set_count.update(move |n| *n += 1))
                            .text("+"),
                    ),
            )
    }
}

struct HelloComponent {
    message: String,
}

impl Component for HelloComponent {
    fn render(&self, _cx: &ComponentContext) -> El {
        El::new("div").text(&self.message)
    }
}

trait Component {
    fn on_init(&mut self, ctx: &ComponentContext) {}
    fn render(&self, cx: &ComponentContext) -> El;
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

impl Deref for El {
    type Target = Element;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

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
