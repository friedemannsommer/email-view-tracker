use super::{base::Layout, header::Header};

pub fn template(user: &entity::user::ActiveModel) -> String {
    let title = "Home";

    Layout {
        body: markup::new! {
            @Header { title, user }
            h2 { "Create, view, or update trackers" }
        },
        header: "",
        title,
    }
    .to_string()
}
