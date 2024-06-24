//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.3

use std::fmt::Debug;

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub login: String,
    pub password: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::subdomain::Entity")]
    Subdomain,
}

impl Debug for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Model")
            .field("id", &self.id)
            .field("login", &self.login)
            .field("password", &"***")
            .finish()
    }
}

impl Related<super::subdomain::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Subdomain.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
