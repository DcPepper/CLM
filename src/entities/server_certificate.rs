//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0-rc.5

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "server_certificate")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub certificate_id: i32,
    pub server_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::certificate::Entity",
        from = "Column::CertificateId",
        to = "super::certificate::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Certificate,
    #[sea_orm(
        belongs_to = "super::server::Entity",
        from = "Column::ServerId",
        to = "super::server::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Server,
}

impl Related<super::certificate::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Certificate.def()
    }
}

impl Related<super::server::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Server.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
