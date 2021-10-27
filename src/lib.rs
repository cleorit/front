// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        source: Sentence::new("pt_BR", "O gato esta no cesto".to_string()),
        target: "creole",
        translations: vec![
            Sentence::new("creole", "Translation 1".to_string()),
            Sentence::new("creole", "Translation 2".to_string()),
            Sentence::new("creole", "Translation 3".to_string()),
            Sentence::new("ja", "Translation 4".to_string()),
            Sentence::new("ja", "Translation 5".to_string()),
        ],
        placeholder: Some("Traducao do google".to_string()),
        placeholder_element: ElRef::default(),
    }
}

// ------ ------
//     Model
// ------ ------

struct Sentence {
    lang: &'static str,
    text: String,
    votes: i32,
}

impl Sentence {
    fn new(lang: &'static str, text: String) -> Self {
        Self {
            lang,
            text,
            votes: 0,
        }
    }
}

// `Model` describes our app state.
struct Model {
    source: Sentence,
    target: &'static str,
    translations: Vec<Sentence>,
    placeholder: Option<String>,
    placeholder_element: ElRef<web_sys::HtmlInputElement>,
}

// ------ ------
//    Update
// ------ ------

// (Remove the line below once any of your `Msg` variants doesn't implement `Copy`.)
#[derive(Copy, Clone)]
// `Msg` describes the different events you can modify state with.
enum Msg {
    // Target(&'static str),
    Again,
    Great,
    Vote(usize, i32),
    Promote,
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        // Msg::Target(lang) => {
        //     model.target = lang;
        // }
        Msg::Again => {
            log!("Again :/");
        }
        Msg::Great => {
            log!("Great :)");
        }
        Msg::Vote(i, v) => {
            log!("Vote: ", i, v);
            model.translations[i].votes += v;
        }
        Msg::Promote => {
            model.placeholder.take().unwrap();
            let text = model.placeholder_element.get().unwrap().value();

            let mut sentence = Sentence::new(model.target, text);
            sentence.votes += 1;

            model.translations.push(sentence);
        }
    }
}

// ------ ------
//     View
// ------ ------

// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    div![
        C![
            "min-h-screen",
            "flex",
            "flex-col",
            "gap-2",
            "place-content-center",
            "bg-indigo-50"
        ],
        div![
            C!["w-9/12", "mx-auto", "bg-white", "shadow", "p-8", "rounded"],
            div![
                // div![
                //     span!["set target:"],
                //     button!["ja", ev(Ev::Click, |_| Msg::Target("ja"))],
                //     button!["creole", ev(Ev::Click, |_| Msg::Target("creole"))],
                // ],
                div![
                    div![
                        C!["py-2"],
                        span![C!["w-24"], model.source.lang, ":"],
                        span![C!["px-4"], &model.source.text]
                    ],
                    div![
                        C!["flex", "flex-row", "py-2"],
                        div![
                            C!["flex", "flex-col", "place-content-center"],
                            div![model.target, ":"],
                        ],
                        div![
                            C!["flex", "flex-col", "gap-2", "p-4", "flex-grow"],
                            model
                                .translations
                                .iter()
                                .enumerate()
                                .filter(|(_, t)| t.lang == model.target)
                                .map(|(i, t)| {
                                    div![
                                        C!["flex", "gap-2", "flex-grow"],
                                        span![C!["bg-blue-10", "w-4"], &t.votes],
                                        span![C!["flex-grow"], &t.text],
                                        button![
                                            C![
                                                "bg-green-500",
                                                "hover:bg-green-300",
                                                "rounded",
                                                "px-2"
                                            ],
                                            "u",
                                            ev(Ev::Click, move |_| Msg::Vote(i, 1))
                                        ],
                                        button![
                                            C!["bg-red-500", "hover:bg-red-300", "rounded", "px-2"],
                                            "d",
                                            ev(Ev::Click, move |_| Msg::Vote(i, -1))
                                        ]
                                    ]
                                }),
                            model.placeholder.as_ref().map(|sentence| div![
                                C!["flex", "gap-2", "flex-grow"],
                                span![C!["w-4"], 0],
                                input![
                                    C!["flex-grow", "border-2", "rounded", "hover:shadow-2xl"],
                                    el_ref(&model.placeholder_element),
                                    attrs! {At::Value => sentence}
                                ],
                                button![
                                    C!["bg-green-500", "hover:bg-green-300", "rounded", "px-2"],
                                    "u",
                                    ev(Ev::Click, |_| Msg::Promote)
                                ],
                                div![C!["bg-gray-500", "rounded", "px-2"], "d",]
                            ],)
                        ]
                    ],
                    div![
                        C!["flex", "flex-row", "gap-2", "place-content-center"],
                        button![
                            C!["bg-red-500", "hover:bg-red-300", "rounded", "px-2"],
                            "Again :/",
                            ev(Ev::Click, |_| Msg::Again)
                        ],
                        button![
                            C!["bg-green-500", "hover:bg-green-300", "rounded", "px-2"],
                            "Great :)",
                            ev(Ev::Click, |_| Msg::Great)
                        ]
                    ],
                ],
            ],
        ],
    ]
}

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
