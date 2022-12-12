use super::base::{Button, ButtonType, Layout, ThemeColor};

pub fn template(csrf_token: &str) -> String {
    Layout {
        body: markup::new! {
            form[method="post"] {
                div."input-group" {
                    label["for"="username"] { "Username" }
                    input[id="username", "type"="text", name="username", required, autofocus];
                }
                div."input-group" {
                    label["for"="userPassword"] { "Password" }
                    input[id="userPassword", "type"="password", name="password", autocomplete="current-password", required];
                }
                input["type"="hidden", name="csrf_token", value={csrf_token}];
                @Button{ label: "Login", button_type: ButtonType::Submit, theme: ThemeColor::Primary }
            }
        },
        header: super::shared::get_default_header(super::shared::StylesheetVariant::Login),
        title: "Login",
    }
    .to_string()
}
