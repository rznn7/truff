use crate::core::runtime::start_app;
use crate::examples::conditional_rendering::ConditionalRenderingExample;

mod core;
mod examples;

fn main() {
    start_app(ConditionalRenderingExample {});
}
