use crate::core::component::{Component, ComponentContext};
use crate::core::el::El;
use leptos_reactive::{SignalGet, SignalUpdate, WriteSignal, create_signal};

// ============= Components =============

pub struct ConditionalRenderingExample;
impl Component for ConditionalRenderingExample {
    fn render(&self, ctx: &ComponentContext) -> El {
        let show_signal = create_signal(ctx.scope(), false);

        El::new("div")
            .attr("style", "padding: 20px; font-family: sans-serif;")
            .child(
                ComponentA {
                    write_signal: show_signal.1,
                }
                .mount(ctx),
            )
            .dyn_child(ctx, move |ctx| {
                if show_signal.0.get() {
                    Some(ComponentB {}.mount(ctx))
                } else {
                    None
                }
            })
    }
}

pub struct ComponentA {
    write_signal: WriteSignal<bool>,
}
impl Component for ComponentA {
    fn render(&self, _: &ComponentContext) -> El {
        let write_signal = self.write_signal;
        El::new("button")
            .text("Toggle ")
            .on("click", move |_| write_signal.update(|show| *show = !*show))
    }
}

pub struct ComponentB;
impl Component for ComponentB {
    fn render(&self, ctx: &ComponentContext) -> El {
        let show_signal_bis = create_signal(ctx.scope(), false);

        El::new("div")
            .text("Displayed!")
            .child(
                El::new("div").child(El::new("button").text("again?").on("click", move |_| {
                    show_signal_bis.1.update(|show| *show = !*show)
                })),
            )
            .dyn_child(ctx, move |_| {
                if show_signal_bis.0.get() {
                    Some(El::new("span").text("YES!"))
                } else {
                    None
                }
            })
    }
}
