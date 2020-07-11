extern crate web_view;

mod scripts;

use web_view::*;

fn main() {
    let html_content = format!(include_str!("template/index.html"),
        styles = inline_style(include_str!("template/style.css")),
        scripts = inline_script(include_str!("template/app.js")),
    );
    
    let _scripts = scripts::Asset::iter();

    let mut maximized_state = false;
    
    let webview = builder()
        .title("Bloop")
        .content(Content::Html(html_content))
        .size(750, 400)
        .frameless(true)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|webview, arg|  {
            match arg {
                "exit" => webview.exit(),
                "maximize" => {
                    maximized_state = !maximized_state;
                    webview.set_maximized(maximized_state);
                },
                "minimize" => {
                    webview.set_minimized();
                },
                "drag_intent" => {
                    webview.drag_intent();
                },
                _ => {
                    if arg.starts_with("sc"){
                        script_eval(&arg[2..], webview)?;
                    }   
                },
            }    
            Ok(())
        })
        .build()
        .unwrap();

        webview.run().unwrap();
}

fn inline_style(s: &str) -> String {
    format!(r#"<style type="text/css">{}</style>"#, s)
}

fn inline_script(s: &str) -> String {
    format!(r#"<script type="text/javascript">{}</script>"#, s)
}

fn script_eval(arg: &str, webview: &mut WebView<()>) -> WVResult {
    let script_str: &str;
    match scripts::Asset::get("test.txt") {
        Some(content) => {
            script_str = std::str::from_utf8(content.as_ref()).unwrap();
            println!("{}", &script_str);
            webview.eval(&format!("{:?}; {}.main(editorObj)", &script_str, &arg[2..]))
        },
        None => {
            println!("Error: File Not Found");
            webview.eval("Alert('Programmer made a fucky-wucky');")
        }
    }
}