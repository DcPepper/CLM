use async_graphql::*;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, SimpleObject)]
#[sea_orm(table_name = "server")]
#[graphql(concrete(name = "Server", params()))]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub fqdn: String,
    pub ipv4: String,
    pub ipv6: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl Entity {
    pub fn find_by_id(id: i32) -> Select<Entity> {
        Self::find().filter(Column::Id.eq(id))
    }
}

impl ActiveModelBehavior for ActiveModel {}
