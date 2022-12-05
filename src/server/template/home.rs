use super::{
    base::{Layout, Stylesheet},
    header::Header,
};

const TITLE: &str = "Home";

pub fn template(user: &entity::user::ActiveModel, trackers: &[entity::tracker::Model]) -> String {
    Layout {
        body: markup::new! {
            @Header { title: TITLE, user }
            section.trackers {
                header {
                    h2 { "Trackers" }
                    a[href="/tracker/create"] {
                        button { "Create tracker" }
                    }
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
                                    a[href={format!("/tracker/update/{}", tracker.id)}] {
                                        button { "Edit" }
                                    }
                                    a[href={format!("/tracker/delete/{}", tracker.id)}] {
                                        button { "Delete" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        },
        header: Stylesheet {
            path: "/css/user.css",
        },
        title: TITLE,
    }
    .to_string()
}
