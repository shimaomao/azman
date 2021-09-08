use crate::{repository::{DBError, POOL, Dao}, util::serde_format::naive_datetime};
use chrono::NaiveDateTime;
use rbatis::{crud::CRUD, wrapper::Wrapper};
use app_macro::Dao;
use serde::Serialize;
use async_trait::async_trait;

#[crud_table(table_name: "user_has_roles")]
#[derive(Debug, Clone, Dao)]
pub struct UserRole {
    pub user_id: String,
    pub role_id: String,
    #[serde(serialize_with = "naive_datetime::serialize")]
    pub expire: NaiveDateTime,
    #[serde(serialize_with = "naive_datetime::serialize")]
    pub created_at: NaiveDateTime,
}

impl UserRole {
  pub async fn find_by_id(user_id: &str, role_id: &str) -> Result<Self, DBError> {
    let w = POOL.new_wrapper().eq("user_id", user_id).and().eq("role_id", role_id);
    Self::find_one(&w).await
  }
  pub async fn find_by_user(user_id: &str) -> Result<Vec<Self>, DBError> {
    let w = POOL.new_wrapper().eq("user_id", user_id);
    Self::find_list(&w).await
  }
  pub async fn find_by_role(role_id: &str) -> Result<Vec<Self>, DBError> {
    let w = POOL.new_wrapper().eq("role_id", role_id);
    Self::find_list(&w).await
  }
}