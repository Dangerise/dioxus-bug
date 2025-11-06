use dioxus::prelude::*;

#[derive(Clone)]
struct Elm {
    title: String,
    content: Vec<String>,
}

fn main() {
    dioxus::logger::init(dioxus::logger::tracing::Level::TRACE).unwrap();
    dioxus::launch(app);
}

#[component]
fn app() -> Element {
    let mut elm = use_signal(|| Elm {
        title: "x".into(),
        content: vec![
            "x1138o4u1239487\n12349071234897".into(),
            "x21348u1293847123498\n1239847123894".into(),
        ],
    });
    let mut ano = use_signal(|| Elm {
        title: "y".into(),
        content: vec![
            "y1asdfasdf\n12312312312".into(),
        ],
    });
    let out = elm.cloned();
    let title = &out.title;
    let content = &out.content;
    rsx! {
        button {
            onclick: move |_| {
                let tmp = elm.cloned();
                elm.set(ano.cloned());
                ano.set(tmp);
            },
            "change"
        }
        div {
            p { "{title}" }
            for (idx , one) in content.iter().enumerate() {
                if idx > 0 {
                    hr {}
                }
                for line in one.lines() {
                    p { "{line}" }
                }
            }
        }
    }
}
