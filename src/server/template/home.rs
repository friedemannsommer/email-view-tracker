use super::{base::Layout, header::Header};

const TITLE: &str = "Home";

pub fn template(user: &entity::user::ActiveModel, trackers: &[entity::tracker::Model]) -> String {
    Layout {
        body: markup::new! {
            @Header { title: TITLE, user }
            section.trackers {
                div.header {
                    h2 { "Trackers" }
                    a[href="/tracker/create", target="_self"] { "Create" }
                }
                table {
                    thead {
                        tr {
                            th { "Name" }
                            th { "Views" }
                            th { "Actions" }
                        }
                    }
                    tbody {
                        @for tracker in trackers {
                            tr {
                                td { @tracker.name }
                                td { @tracker.views }
                                td {
                                    a[href={format!("/tracker/edit/{}", tracker.id)}, target="_self"] {
                                        button { "Edit" }
                                    }
                                }
                            }
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
