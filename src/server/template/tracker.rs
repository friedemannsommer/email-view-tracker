use super::{base::Layout, header::Header};

const TITLE: &str = "Tracker";

pub fn template(
    user: &entity::user::ActiveModel,
    tracker: Option<&entity::tracker::ActiveModel>,
) -> String {
    let has_tracker = tracker.is_some();

    Layout {
        body: markup::new! {
            @Header { title: TITLE, user }
            form[method="POST"] {
                div."input-group" {
                    label["for"="name"] { "Name" }
                    input["id"="name", "type"="text", name="name", value={tracker.map(|tracker|tracker.name.as_ref().as_str()).unwrap_or("")}];
                }
                @if has_tracker {
                    div."input-group" {
                        label { "Views" }
                        span.views {{tracker.map(|tracker|*tracker.views.as_ref()).unwrap_or_default()}}
                    }
                    div."input-group" {
                        label { "Tracking code" }
                        code {
                            pre {
                                r#"<img src="https://server/track/"#
                                {tracker.map(|tracker|tracker.id.as_ref().to_string()).unwrap_or_default()}
                                r#"" />"#
                            }
                        }
                    }
                }
                button["type"="submit"] {
                    @if tracker.is_some() { "Update" } else { "Create" }
                }
            }
        },
        header: "",
        title: TITLE,
    }
        .to_string()
}
