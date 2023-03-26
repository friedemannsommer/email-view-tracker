use super::{
    base::{Button, ButtonLink, ButtonType, Layout, ThemeColor},
    header::Header,
};

const TITLE: &str = "Profile";

pub fn template(user: &entity::user::ActiveModel) -> String {
    Layout {
        body: markup::new! {
            @Header { title: TITLE, user }
            section.container."width-restricted" {
                form[method="POST"] {
                    div."input-group" {
                        label["for"="username"] { "Username" }
                        input[id="username", "type"="text", name="username", value={user.name.as_ref()}];
                    }
                    div."input-group" {
                        label["for"="userPassword"] { "New password" }
                        input[id="userPassword", "type"="password", name="password", autocomplete="new-password"];
                    }
                    div."button-group" {
                        @Button{ label: "Update", button_type: ButtonType::Submit, theme: ThemeColor::Primary }
                        @ButtonLink{ url: "/home", label: "Cancel", ..Default::default() }
                    }
                }
            }
        },
        header: super::shared::get_default_header(super::shared::StylesheetVariant::User),
        title: TITLE,
    }
        .to_string()
}
