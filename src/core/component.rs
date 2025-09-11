use crate::core::el::El;
use crate::core::service::ServiceContainer;
use leptos_reactive::Scope;
use std::cell::RefCell;
use std::rc::Rc;

thread_local! {
    static CONTEXT_STACK: RefCell<Vec<ComponentContext>> = const { RefCell::new(Vec::new()) };
}

#[derive(Clone)]
pub struct ComponentContext {
    pub scope: Scope,
    services: Rc<RefCell<ServiceContainer>>,
}

impl ComponentContext {
    pub fn new(scope: Scope) -> Self {
        Self {
            scope,
            services: Rc::new(RefCell::new(ServiceContainer::new())),
        }
    }

    pub fn with<R>(&self, f: impl FnOnce() -> R) -> R {
        CONTEXT_STACK.with(|stack| {
            stack.borrow_mut().push(self.clone());
        });

        let result = f();

        CONTEXT_STACK.with(|stack| {
            stack.borrow_mut().pop();
        });

        result
    }

    pub fn current() -> Option<Self> {
        CONTEXT_STACK.with(|stack| stack.borrow().last().cloned())
    }

    pub fn provide<T: std::any::Any + 'static>(service: T) {
        if let Some(ctx) = Self::current() {
            ctx.services.borrow_mut().register(service);
        }
    }

    pub fn inject<T: std::any::Any + 'static>() -> Option<Rc<RefCell<T>>> {
        Self::current()?.services.borrow().get::<T>()
    }

    pub fn scope() -> Option<Scope> {
        Self::current().map(|ctx| ctx.scope)
    }

    pub fn create_child(&self) -> Self {
        let mut new_container = ServiceContainer::new();
        for (type_id, service_rc) in self.services.borrow().services() {
            new_container
                .services_mut()
                .insert(*type_id, service_rc.clone());
        }
        Self {
            scope: self.scope,
            services: Rc::new(RefCell::new(new_container)),
        }
    }
}

pub trait Component {
    fn on_init(&mut self) {}
    fn render(&self) -> El;

    fn mount(mut self) -> El
    where
        Self: Sized,
    {
        self.on_init();
        self.render()
    }
}
