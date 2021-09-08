use crate::repository::{DBError, POOL};
use app_macro::Dao;
use app_macro_trait::Dao;
use async_trait::async_trait;
use chrono::NaiveDateTime;
use serde::Serialize;
use rbatis::{crud::CRUD, wrapper::Wrapper};

#[crud_table(table_name: "roles")]
#[derive(Debug, Clone, Dao)]
pub struct RoleDao {
    pub id: Option<i32>,
    pub name: String,
    pub description: Option<String>,
    pub value: String,
    pub level: i32,
    pub domain_id: String,
    pub is_deleted: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
}

impl RoleDao{
  pub async fn find_by_ids(id: Vec<i32>, domain_id: Option<String>) -> Result<Vec<Self>, DBError> {
    let mut w = POOL.new_wrapper().r#in("id", &id);
    if let Some(domain_id) = domain_id {
      w = w.eq("domain_id", domain_id);
    }
    Self::find_list(&w).await
  }
  pub async fn find_all(domain_id: Option<String>) -> Result<Vec<Self>, DBError> {
    let mut w = POOL.new_wrapper();
    if let Some(domain_id) = domain_id {
      w = w.eq("domain_id", domain_id);
    }
    Self::find_list(&w).await
  }
}