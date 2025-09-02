use crate::core::component::{Component, ComponentContext};
use crate::core::el::El;
use leptos_reactive::{SignalGet, SignalUpdate, create_signal};

// ============= Components =============

pub struct ConditionalRenderingExample;
impl Component for ConditionalRenderingExample {
    fn render(&self, ctx: &ComponentContext) -> El {
        let show_signal = create_signal(ctx.scope(), false);

        El::new("div")
            .attr("style", "padding: 20px; font-family: sans-serif;")
            .child(El::new("button").text("Toggle ").on("click", move |_| {
                show_signal.1.update(|show| *show = !*show)
            }))
            .dyn_child(ctx, move |ctx| {
                let show_signal_bis = create_signal(ctx.scope(), false);
                if show_signal.0.get() {
                    Some(
                        El::new("div")
                            .text("Displayed!")
                            .child(El::new("div").child(
                                El::new("button").text("again?").on("click", move |_| {
                                    show_signal_bis.1.update(|show| *show = !*show)
                                }),
                            ))
                            .dyn_child(ctx, move |ctx| {
                                if show_signal_bis.0.get() {
                                    Some(El::new("span").text("YES!"))
                                } else {
                                    None
                                }
                            }),
                    )
                } else {
                    None
                }
            })
    }
}
