use super::{AnimateDice, Scene};
use crate::components::*;
use crate::rules::*;
use crate::UpdateStatus;
use dioxus::prelude::*;
use tinyrand::{RandRange, StdRand};

#[inline_props]
fn PlayGame<'a>(
    cx: Scope<'a>,
    dice: &'a UseState<(u8, u8)>,
    animate: &'a UseState<bool>,
) -> Element<'a> {
    let rules: Vec<_> = use_shared_state::<Vec<Rule>>(cx)
        .unwrap()
        .read()
        .iter()
        .filter(|r| r.check(dice))
        .cloned()
        .collect();

    render! {
        h1 {
            class: "dice text-center",
            "{dice.0}{dice.1}"
        }
        ul {
            class: "grow scrollbox",
            if rules.is_empty() {
                render! {
                    li { "Ingenting" }
                }
            }
            for r in rules.iter() {
                li { r.name() }
            }
        }

        if **dice == (6,6) {
            render! {
                NavButton {
                    class: "bg-secondary rounded-md box-border w-full h-[15vh] min-h-[15vh]",
                    to: Scene::Create,
                    "Jag har aldrig sett..."
                }
            }
        }

        if **dice == (1,2) || **dice == (2,1) {
            render! {
                NavButton {
                    class: "bg-secondary rounded-md box-border w-full h-[15vh] min-h-[15vh]",
                    to: Scene::Challange,
                    "Utmaning"
                }
            }
        } else {
            render! {
                button {
                    class: "bg-primary rounded-md box-border w-full h-[15vh] min-h-[15vh]",
                    onclick: move |_| {
                        let mut rng = use_shared_state::<StdRand>(cx).unwrap().write_silent();
                        dice.set((
                            rng.next_range(1..7_u16) as u8,
                            rng.next_range(1..7_u16) as u8
                        ));
                        animate.set(true);
                    },
                    "Rulla"
                }
            }
        }
    }
}

pub fn Game(cx: Scope) -> Element {
    let dice = use_state(cx, || (0, 0));
    let animate = use_state(cx, || false);

    let update = use_shared_state::<UpdateStatus>(cx);

    render! {
        div {
            class: "flex flex-col gap-4 p-4 w-[100vmin] h-screen",
            div {
                class: "flex flex-row-reverse h-[4vh] w-100 justify-between items-center",

                Link{
                    to: Scene::Help,
                    class: "w-6 h-6",
                    QuestionMarkIcon { }
                }

                button {
                    id: "update",
                    class: "bg-secondary rounded-md box-border pl-3 pr-2 h-[4vh] min-h-[4vh] text-sm inline-flex items-center gap-2",
                    onclick: move |_| {
                        let js = use_eval(cx);
                        js("window.location.reload()").unwrap();
                    },
                    "Uppdatera"
                    span{
                        class: "w-6 h-6 stroke-white",
                        DownloadIcon { }
                    }
                }

                span { /* empty span makes the install/update button centered */ }
            }

            if **animate {
                render!{
                    AnimateDice {
                        animate: animate,
                        button_color: "bg-primary",
                        button_text: "Rulla",
                    }
                }
            } else {
                render!{
                    PlayGame {
                        dice: dice,
                        animate: animate,
                    }
                }
            }
        }
    }
}
