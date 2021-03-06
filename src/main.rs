#![windows_subsystem = "windows"]

extern crate web_view;

mod scripts;

use web_view::*;

use std::collections::HashMap;

struct Bloop {
    script_list: HashMap<String, scripts::Script>,
}

impl Bloop {
    fn new() -> Bloop {
        Bloop {
            script_list: HashMap::new(),
        }
    }
    fn exec(&mut self, html_content: &str) {
        // let mut maximized_state = false;

        let view = builder()
            .title("Bloop")
            .content(Content::Html(&html_content))
            .size(900, 400)
            .frameless(true)
            .resizable(true)
            .debug(false)
            .user_data(())
            .invoke_handler(|webview, arg| {
                match arg {
                    "exit" => webview.exit(),
                    // "maximize" => {
                    //     maximized_state = !maximized_state;
                    //     webview.set_maximized(maximized_state);
                    // }
                    // "minimize" => {
                    //     webview.set_minimized();
                    // },
                    // "drag_intent" => {
                    //     webview.drag_intent();
                    // },
                    "doc_ready" => {
                        scripts::build_scripts(webview, &mut self.script_list)?;
                    }
                    _ => {
                        if arg.starts_with("#") {
                            scripts::script_eval(
                                &self.script_list.get(&arg[1..]).unwrap(),
                                webview,
                            )?;
                        }
                    }
                }
                Ok(())
            })
            .build()
            .unwrap();
        view.run().unwrap();
    }
}

fn main() {
    let html_content = include_str!("../dist/index.html");
    let mut app = Bloop::new();
    app.exec(&html_content);
}

fn _inline_style(s: &str) -> String {
    format!(r#"<style type="text/css">{}</style>"#, s)
}

fn _inline_script(s: &str) -> String {
    format!(r#"<script type="text/javascript">{}</script>"#, s)
}
