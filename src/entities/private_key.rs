//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0-rc.5

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "private_key")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub certificate_id: i32,
    pub meta: Json,
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
}

impl Related<super::certificate::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Certificate.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}