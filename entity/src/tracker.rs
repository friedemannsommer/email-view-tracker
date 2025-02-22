use sea_orm::{
    ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EntityTrait,
    EnumIter, PrimaryKeyTrait, Related, RelationDef, RelationTrait,
    prelude::TimeDateTimeWithTimeZone,
};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, serde::Deserialize, serde::Serialize)]
#[sea_orm(table_name = "trackers")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub name: String,
    pub views: i64,
    pub user_id: Uuid,
    pub created_at: TimeDateTimeWithTimeZone,
    pub updated_at: TimeDateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
