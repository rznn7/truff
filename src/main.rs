use crate::{core::runtime::start_app, examples::base::BaseExample};

mod core;
mod examples;

fn main() {
    start_app(BaseExample);
}
