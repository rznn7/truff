use leptos_reactive::{
    ReadSignal, Scope, SignalGet, SignalSet, SignalUpdate, WriteSignal, create_signal,
};

use crate::core::component::{Component, ComponentContext};
use crate::core::el::El;

// ============= Components =============

pub struct BaseExample;
impl Component for BaseExample {
    fn render(&self, ctx: &ComponentContext) -> El {
        ctx.provide(CounterService::new(ctx.scope()));

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
            .child(El::new("h2").text("Display Panel"))
            .child(
                El::new("div")
                    .attr("style", "font-size: 24px; margin: 10px 0;")
                    .child(El::new("span").text("Current Count: "))
                    .child(
                        El::new("span")
                            .attr("style", "font-weight: bold; color: #2196F3;")
                            .dyn_text(ctx.scope(), move || count_signal.get().to_string()),
                    ),
            )
            .child(
                El::new("div")
                    .attr("style", "font-size: 14px; color: #666;")
                    .dyn_text(ctx.scope(), move || {
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
        ctx.provide(CounterService::new(ctx.scope()));

        El::new("div")
            .attr(
                "style",
                "margin: 20px 0; padding: 15px; background: #f4ddff; border-radius: 8px;",
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
                    .dyn_text(ctx.scope(), move || count_signal.get().to_string()),
            )
    }
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
