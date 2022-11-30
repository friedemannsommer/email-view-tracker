use super::{base::Layout, header::Header};

pub fn template(user: &entity::user::ActiveModel) -> String {
    let title = "Profile";

    Layout {
        body: markup::new! {
            @Header { title, user }
            form[method="POST"] {
                div."input-group" {
                    label["for"="username"] { "Username" }
                    input["id"="username", "type"="text", name="username", value={user.name.as_ref()}];
                }
                div."input-group" {
                    label["for"="userPassword"] { "Password" }
                    input[id="userPassword", "type"="password", name="password", autocomplete="new-password"];
                }
                button["type"="submit"] {
                    "Update"
                }
            }
        },
        header: "",
        title,
    }
        .to_string()
}
