use super::{base::Layout, header::Header};

const TITLE: &str = "Profile";

pub fn template(user: &entity::user::ActiveModel) -> String {
    Layout {
        body: markup::new! {
            @Header { title: TITLE, user }
            form[method="POST"] {
                div."input-group" {
                    label["for"="username"] { "Username" }
                    input["id"="username", "type"="text", name="username", value={user.name.as_ref()}];
                }
                div."input-group" {
                    label["for"="userPassword"] { "Password" }
                    input[id="userPassword", "type"="password", name="password", autocomplete="new-password"];
                }
                div."button-group" {
                    button["type"="submit"] {
                        "Update"
                    }
                    a[href="/home"] {
                        button["type"="button"] { "Cancel" }
                    }
                }
            }
        },
        header: "",
        title: TITLE,
    }
        .to_string()
}
