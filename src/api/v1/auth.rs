use axum::{extract::Query, handler::post, routing::BoxRoute, Json, Router};
use std::collections::HashMap;
use validator::Validate;

use crate::{
    repository::{
        dto::{LoginUser, NewUser, UserGrantRole},
        vo::{Domain, Org, Role, User, UserOrg, UserRole},
    },
    util::{
        jwt::{self, Auth},
        APIResult,
    },
};

async fn register(
    Query(query): Query<HashMap<String, String>>,
    Json(body): Json<NewUser>,
) -> APIResult {
    body.validate()?;
    if body.exists().await.is_ok() {
        return Err(reject!("用户已存在"));
    }
    let domain = match query.get("from") {
        Some(domain_id) => {
            let domain = match Domain::find_one(domain_id.clone()).await {
                Ok(val) => val,
                Err(_) => return Err(reject!(format!("来源域 {} 不存在", domain_id))),
            };
            domain
        }
        None => return Err(reject!("来源域不能为空")),
    };
    let role = match domain.default_role_id {
        Some(role_id) => {
            let role = Role::find_one(role_id).await?;
            role
        }
        None => {
            return Err(reject!(format!(
                "来源域 {} 没有默认角色",
                domain.id.clone()
            )))
        }
    };
    let user = body.create().await?;
    let user_grant_role = UserGrantRole {
        user_id: user.id.clone(),
        role_id: role.id.unwrap(),
    };
    user_grant_role.save().await?;
    let token = jwt::generate_token(Auth {
        id: user.id.clone(),
        username: user.username.clone(),
        domain_id: Some(domain.id.clone()),
        org_id: vec![],
        role_id: vec![role.id.unwrap()],
        role_level: role.level,
        is_admin: false,
    });
    Ok(reply!({
      "token": token, "user": user,"domain": domain, "roles": vec![role], "orgs": []
    }))
}

async fn login(
    Query(query): Query<HashMap<String, String>>,
    Json(body): Json<LoginUser>,
) -> APIResult {
    body.validate()?;
    let user_dao = match body.find_one().await {
        Ok(val) => val,
        Err(_) => return Err(reject!("用户不存在")),
    };
    let user: User = user_dao.clone().into();
    if !body.is_password_matched(&user.password) {
        return Err(reject!("密码不正确"));
    }
    if !user.is_actived {
        return Err(reject!("用户被禁用"));
    }

    let user = body.login(&user_dao).await?;
    let mut roles: Vec<Role> = vec![];
    let mut role_ids: Vec<i32> = vec![];
    let mut orgs: Vec<Org> = vec![];
    let mut org_ids: Vec<String> = vec![];
    let mut domain: Option<Domain> = None;
    let domain_id = query.get("from").clone().map(|v| v.clone());
    let is_admin = user.sys_role.clone().unwrap() == "admin";
    if !is_admin {
        domain = match domain_id.clone() {
            Some(v) => {
                let domain = match Domain::find_one(v.clone()).await {
                    Ok(val) => val,
                    Err(_) => return Err(reject!(format!("来源域 {} 不存在", v.clone()))),
                };
                Some(domain)
            }
            None => return Err(reject!("来源域不能为空")),
        };
        let user_orgs = UserOrg::find_by_user(user.id.clone()).await?;
        org_ids = user_orgs.iter().map(|v| v.org_id.clone()).collect();
        orgs = Org::find_by_ids(org_ids.clone(), domain_id.clone()).await?;

        let user_roles = UserRole::find_by_user(user.id.clone()).await?;
        role_ids = user_roles.iter().map(|v| v.role_id).collect();
        roles = Role::find_by_ids(role_ids.clone(), domain_id.clone()).await?;
        roles.sort_by(|a, b| a.level.cmp(&b.level));
    }
    let role_level = if roles.len() > 0 { roles[0].level } else { 999 };
    let token = jwt::generate_token(Auth {
        id: user.id.clone(),
        username: user.username.clone(),
        domain_id,
        org_id: org_ids,
        role_id: role_ids,
        role_level,
        is_admin,
    });
    Ok(reply!({
      "token": token, "user": user, "domain": domain, "roles": roles, "orgs": orgs
    }))
}

pub fn apply_routes(v1: Router<BoxRoute>) -> Router<BoxRoute> {
    v1.route("/register", post(register))
        .route("/login", post(login))
        .boxed()
}