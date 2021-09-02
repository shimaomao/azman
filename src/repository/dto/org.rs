use crate::repository::{dao::OrgDao, vo::Org, DBError, POOL};
use app_macro::Dao;
use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct NewOrg {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    pub description: Option<String>,
    #[serde(skip_deserializing)]
    pub domain_id: String,
    #[serde(skip_deserializing)]
    pub created_by: Option<String>,
}

fn now() -> NaiveDateTime {
    Local::now().naive_local()
}

impl NewOrg {
    pub async fn create(&self) -> Result<Org, DBError> {
        let id = Uuid::new_v4().to_string();
        let dao = OrgDao {
            id: id.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
            domain_id: self.domain_id.clone(),
            is_deleted: Some(0),
            created_by: self.created_by.clone(),
            updated_by: None,
            created_at: now(),
            updated_at: now(),
        };
        OrgDao::create_one(&dao).await?;
        Ok(dao.into())
    }
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct UpdateOrg {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    pub description: Option<String>,
    #[validate(length(min = 1, max = 200))]
    pub value: String,
    #[serde(skip_deserializing)]
    pub updated_by: Option<String>,
}

impl UpdateOrg {
    pub async fn save(&self, id: String) -> Result<Org, DBError> {
        let w = POOL.new_wrapper().eq("id", id);
        let mut dao = OrgDao::find_one(&w).await?;
        dao.name = self.name.clone();
        dao.description = self.description.clone();
        OrgDao::update_one(&dao, &w).await?;
        Ok(dao.into())
    }
}
