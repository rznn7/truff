use crate::core::{el::El, service::ServiceContainer};
use leptos_reactive::*;
use std::{any::Any, cell::RefCell, rc::Rc};

pub trait Component {
    fn on_init(&mut self, _ctx: &ComponentContext) {}
    fn render(&self, _cx: &ComponentContext) -> El;
}

pub struct ComponentContext {
    scope: Scope,
    services: Rc<RefCell<ServiceContainer>>,
}
impl ComponentContext {
    pub fn new(scope: Scope) -> Self {
        Self {
            scope,
            services: Rc::new(RefCell::new(ServiceContainer::new())),
        }
    }

    pub fn provide<T: Any + 'static>(&self, service: T) {
        self.services.borrow_mut().register(service);
    }

    pub fn inject<T: Any + 'static>(&self) -> Option<Rc<RefCell<T>>> {
        self.services.borrow().get::<T>()
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

    pub fn scope(&self) -> Scope {
        self.scope
    }
}
