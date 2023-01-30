use crate::server::model::tracker_paginator::{OrderType, TrackerOrderColumn, TrackerPagination};

use super::{
    base::{ButtonLink, Layout, ThemeColor},
    header::Header,
};

struct UserQuery {
    pub order: OrderType,
    pub order_by: TrackerOrderColumn,
}

const TITLE: &str = "Home";

pub fn template(
    user: &entity::user::ActiveModel,
    tracker_pagination: &TrackerPagination<'_>,
) -> String {
    Layout {
        body: markup::new! {
            @Header { title: TITLE, user }
            section.trackers {
                header {
                    h2 { "Trackers" }
                    @ButtonLink{ url: "/tracker/create", label: "Create tracker", theme: ThemeColor::Success, ..Default::default() }
                }
                table {
                    thead {
                        tr {
                            th {
                                a[href={get_order_by_url(tracker_pagination, TrackerOrderColumn::Name)}] {
                                    "Name"
                                }
                                @get_order_sign(tracker_pagination, TrackerOrderColumn::Name)
                            }
                            th {
                                a[href={get_order_by_url(tracker_pagination, TrackerOrderColumn::Views)}] {
                                    "Views"
                                }
                                @get_order_sign(tracker_pagination, TrackerOrderColumn::Views)
                            }
                            th {
                                a[href={get_order_by_url(tracker_pagination, TrackerOrderColumn::CreatedAt)}] {
                                    "Created"
                                }
                                @get_order_sign(tracker_pagination, TrackerOrderColumn::CreatedAt)
                            }
                            th {
                                a[href={get_order_by_url(tracker_pagination, TrackerOrderColumn::UpdatedAt)}] {
                                    "Updated"
                                }
                                @get_order_sign(tracker_pagination, TrackerOrderColumn::UpdatedAt)
                            }
                            th { "Actions" }
                        }
                    }
                    tbody {
                        @for tracker in &tracker_pagination.entries {
                            tr {
                                td { @tracker.name }
                                td { @tracker.views }
                                td { @format_date_time(&tracker.created_at) }
                                td { @format_date_time(&tracker.updated_at) }
                                td {
                                    @ButtonLink{ url: &format!("/tracker/update/{}", tracker.id), label: "Edit", ..Default::default() }
                                    @ButtonLink{ url: &format!("/tracker/delete/{}", tracker.id), label: "Delete", theme: ThemeColor::Danger, ..Default::default() }
                                }
                            }
                        }
                    }
                }
                nav.pagination["aria-label"="Pagination", role="navigation"] {
                    @if tracker_pagination.page > 0 {
                        a[href={get_pagination_url(tracker_pagination, tracker_pagination.page - 1)}] {
                            "Previous page"
                        }
                    }
                    p {
                        span["aria-current"="page"] { {tracker_pagination.page + 1} }
                        " / "
                        span { {tracker_pagination.number_of_pages} }
                    }
                    @if tracker_pagination.page < tracker_pagination.number_of_pages.saturating_sub(1) {
                        a[href={get_pagination_url(tracker_pagination, tracker_pagination.page + 1)}] {
                            "Next page"
                        }
                    }
                }
            }
        },
        header: super::shared::get_default_header(super::shared::StylesheetVariant::User),
        title: TITLE,
    }
    .to_string()
}

fn format_date_time(date_time: &time::OffsetDateTime) -> String {
    format!(
        "{}-{:02}-{:02} {:02}:{:02}:{:02}",
        date_time.year(),
        date_time.month() as u8,
        date_time.day(),
        date_time.hour(),
        date_time.minute(),
        date_time.second()
    )
}

fn get_pagination_url(pagination: &TrackerPagination<'_>, page: u64) -> String {
    let mut queries: Vec<String> = Vec::with_capacity(3);

    queries.push(format!("page={page}"));

    if let Some(column) = &pagination.user_query.order_by {
        queries.push(format!("order_by={column}"));
    }

    if let Some(order_type) = &pagination.user_query.order {
        queries.push(format!("order={order_type}"));
    }

    format!("?{}", queries.join("&"))
}

fn get_order_by_url(pagination: &TrackerPagination<'_>, column: TrackerOrderColumn) -> String {
    let user_query = get_user_query(pagination);

    format!(
        "?page={}&order_by={}&order={}",
        pagination.page,
        column,
        if user_query.order_by == column && user_query.order == OrderType::Desc {
            OrderType::Asc
        } else {
            OrderType::Desc
        }
    )
}

fn get_order_sign(pagination: &TrackerPagination<'_>, column: TrackerOrderColumn) -> &'static str {
    let user_query = get_user_query(pagination);

    if user_query.order_by == column {
        if user_query.order == OrderType::Asc {
            "↑"
        } else {
            "↓"
        }
    } else {
        ""
    }
}

fn get_user_query(pagination: &TrackerPagination<'_>) -> UserQuery {
    UserQuery {
        order: pagination
            .user_query
            .order
            .clone()
            .unwrap_or(OrderType::Desc),
        order_by: pagination
            .user_query
            .order_by
            .clone()
            .unwrap_or(TrackerOrderColumn::CreatedAt),
    }
}
