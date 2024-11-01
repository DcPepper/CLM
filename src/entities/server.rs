//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0-rc.5

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "server")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub fqdn: String,
    pub ipv4: String,
    pub ipv6: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::server_certificate::Entity")]
    ServerCertificate,
    #[sea_orm(has_many = "super::server_crontab::Entity")]
    ServerCrontab,
}

impl Related<super::server_certificate::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ServerCertificate.def()
    }
}

impl Related<super::server_crontab::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ServerCrontab.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
