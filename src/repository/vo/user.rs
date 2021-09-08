use crate::{repository::dao::UserDao, util::datetime_format::naive_datetime};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub email: Option<String>,
    pub avatar: Option<String>,
    pub memo: Option<String>,
    pub sys_role: Option<String>,
    pub is_actived: bool,
    #[serde(serialize_with = "naive_datetime::serialize")]
    pub last_logined_at: NaiveDateTime,
    #[serde(serialize_with = "naive_datetime::serialize")]
    pub created_at: NaiveDateTime,
}

impl From<UserDao> for User {
    fn from(dao: UserDao) -> Self {
        Self {
            id: dao.id,
            username: dao.username,
            password: dao.password,
            email: dao.email,
            avatar: dao.avatar,
            memo: dao.memo,
            sys_role: dao.sys_role,
            is_actived: dao.is_actived.map(|v| v == 1).unwrap(),
            last_logined_at: dao.last_logined_at,
            created_at: dao.created_at,
        }
    }
}
