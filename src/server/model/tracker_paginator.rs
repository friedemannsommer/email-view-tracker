#[derive(serde::Deserialize, Debug)]
pub struct TrackersQuery {
    pub order: Option<OrderType>,
    pub order_by: Option<TrackerOrderColumn>,
    pub page: Option<u64>,
}

#[derive(Debug)]
pub struct TrackerPagination<'user_query> {
    pub entries: Vec<entity::tracker::Model>,
    pub number_of_items: u64,
    pub number_of_pages: u64,
    pub page: u64,
    pub user_query: &'user_query TrackersQuery,
}

#[derive(serde::Deserialize, Debug, Clone, Eq, PartialEq)]
pub enum TrackerOrderColumn {
    #[serde(rename = "created_at")]
    CreatedAt,
    #[serde(rename = "updated_at")]
    UpdatedAt,
    #[serde(rename = "name")]
    Name,
    #[serde(rename = "views")]
    Views,
}

#[derive(serde::Deserialize, Debug, Clone, Eq, PartialEq)]
pub enum OrderType {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}

impl std::fmt::Display for TrackerOrderColumn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CreatedAt => f.write_str("created_at"),
            Self::Name => f.write_str("name"),
            Self::UpdatedAt => f.write_str("updated_at"),
            Self::Views => f.write_str("views"),
        }
    }
}

impl std::fmt::Display for OrderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Asc => f.write_str("asc"),
            Self::Desc => f.write_str("desc"),
        }
    }
}
