use std::{
    any::{Any, TypeId},
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
};

pub struct ServiceContainer {
    services: HashMap<TypeId, Rc<dyn Any>>,
}
impl ServiceContainer {
    pub fn new() -> Self {
        Self {
            services: HashMap::new(),
        }
    }

    pub fn register<T: Any + 'static>(&mut self, service: T) {
        self.services.insert(
            TypeId::of::<T>(),
            Rc::new(Rc::new(RefCell::new(service))) as Rc<dyn Any>,
        );
    }

    pub fn get<T: Any + 'static>(&self) -> Option<Rc<RefCell<T>>> {
        self.services
            .get(&TypeId::of::<T>())
            .and_then(|rc| rc.downcast_ref::<Rc<RefCell<T>>>())
            .cloned()
    }

    pub fn services(&self) -> &HashMap<TypeId, Rc<dyn Any>> {
        &self.services
    }

    pub fn services_mut(&mut self) -> &mut HashMap<TypeId, Rc<dyn Any>> {
        &mut self.services
    }
}
