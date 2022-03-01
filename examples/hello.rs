use dioxus::prelude::*;
use dioxus_toast::{ToastInfo, ToastManager, Icon};

fn main() {
    dioxus::desktop::launch(app)
}

fn app(cx: Scope) -> Element {
    std::panic::set_hook(Box::new(|info| {
        println!("Panic: {}", info);
    }));

    let toast = use_ref(&cx, ToastManager::default);

    cx.render(rsx! {
        dioxus_toast::ToastFrame {
            manager: toast
        }
        div {
            button {
                onclick: move |_| {
                    let _id = toast.write().popup(ToastInfo {
                        heading:Some("Hello Dioxus".into()),
                        context:"hello world: <a href=\"#\">Dioxus</a>".into(),
                        allow_toast_close:true,
                        position:dioxus_toast::Position::BottomLeft, 
                        icon: None, 
                        hide_after: Some(5), 
                    });
                    println!("New Toast ID: {}", _id);
                },
                "Normal Toast"
            }
            button {
                onclick: move |_| {
                    let _id = toast.write().popup(ToastInfo {
                        heading:Some("Success!".into()),
                        context:"Dioxus Toast".into(),
                        allow_toast_close:true,
                        position:dioxus_toast::Position::BottomLeft, 
                        icon: Some(Icon::Success), 
                        hide_after: Some(5), 
                    });
                    println!("New Toast ID: {}", _id);  
                },
                "Success Toast"
            }
        }
    })
}
