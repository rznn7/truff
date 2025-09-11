use crate::core::runtime::start_app;
use crate::examples::base::BaseExample;
use crate::examples::conditional_rendering::ConditionalRenderingExample;

mod core;
mod examples;

fn main() {
    start_app(BaseExample {});
    start_app(ConditionalRenderingExample {});
}
