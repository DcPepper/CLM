use async_graphql::*;
use sea_orm::entity::prelute::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, SimpleObject)]
#[sea_orm(table_name = "user")]
#[graphql(concrete(name = "User", params()))]
pub struct Server {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
    pub password: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    #[sea_orm(has_many = "super::server::Entity")]
    Server,
}

impl Related<super::server::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Server::def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
