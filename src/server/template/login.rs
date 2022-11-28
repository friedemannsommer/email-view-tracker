use super::base::Layout;

pub fn template() -> String {
    Layout {
        body: markup::new! {
            form[method="post"] {
                div."input-group" {
                    label["for"="username"] { "Username" }
                    input["id"="username", "type"="text", name="username", required];
                }
                div."input-group" {
                    label["for"="userPassword"] { "Password" }
                    input[id="userPassword", "type"="password", autocomplete="current-password", required];
                }
                button["type"="submit"] {
                    "Login"
                }
            }
        },
        header: markup::new! {
            style {
                /* TODO: add CSS */
            }
        },
        title: "Login",
    }
    .to_string()
}
