use super::{base::Layout, header::Header};

const TITLE: &str = "Tracker";

pub fn template(
    user: &entity::user::ActiveModel,
    tracker: Option<&entity::tracker::ActiveModel>,
    is_ssl: bool,
    hostname: &str,
) -> String {
    let has_tracker = tracker.is_some();
    let tracker_id = tracker.map(|tracker| tracker.id.as_ref().to_string());

    Layout {
        body: markup::new! {
            @Header { title: TITLE, user }
            form[method="POST"] {
                div."input-group" {
                    label["for"="name"] { "Name" }
                    input["id"="name", "type"="text", name="name", value={tracker.map(|tracker|tracker.name.as_ref().as_str()).unwrap_or_default()}];
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
                                r#"<img src=""#
                                {if is_ssl {"https"} else {"http"}}
                                "://"
                                {hostname}
                                "/track/"
                                {tracker_id.as_deref().unwrap_or_default()}
                                r#"" />"#
                            }
                        }
                    }
                }
                div."button-group" {
                    button["type"="submit"] {
                        @if has_tracker { "Update" } else { "Create" }
                    }
                    a[href="/home"] {
                        button["type"="button"] { "Cancel" }
                    }
                    @if has_tracker {
                        a[href={format!("/tracker/delete/{}", tracker_id.as_deref().unwrap_or_default())}] {
                            button["type"="button"] { "Delete" }
                        }
                    }
                }
            }
        },
        header: "",
        title: TITLE,
    }
        .to_string()
}
