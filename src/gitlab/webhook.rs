use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::wecom::notify;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Request {
    pub object_kind: String,
    pub event_type: String,
    pub user: User,
    pub project: Project,
    pub repository: Repository,
    pub object_attributes: ObjectAttributes,
    pub labels: Vec<Label>,
    pub changes: Changes,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub username: String,
    pub avatar_url: Option<String>,
    pub email: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Project {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub web_url: String,
    pub avatar_url: Option<String>,
    pub git_ssh_url: String,
    pub git_http_url: String,
    pub namespace: String,
    pub visibility_level: i64,
    pub path_with_namespace: String,
    pub default_branch: String,
    pub homepage: String,
    pub url: String,
    pub ssh_url: String,
    pub http_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Repository {
    pub name: String,
    pub url: String,
    pub description: String,
    pub homepage: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObjectAttributes {
    pub id: i64,
    pub target_branch: String,
    pub source_branch: String,
    pub source_project_id: i64,
    pub author_id: i64,
    pub assignee_id: i64,
    pub title: String,
    pub created_at: String,
    pub updated_at: String,
    pub milestone_id: Option<i64>,
    pub state: String,
    pub blocking_discussions_resolved: bool,
    pub merge_status: String,
    pub target_project_id: i64,
    pub iid: i64,
    pub description: String,
    pub source: Source,
    pub target: Target,
    pub last_commit: LastCommit,
    pub work_in_progress: bool,
    pub url: String,
    pub action: String,
    pub assignee: Assignee,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Source {
    pub name: String,
    pub description: String,
    pub web_url: String,
    pub avatar_url: Option<String>,
    pub git_ssh_url: String,
    pub git_http_url: String,
    pub namespace: String,
    pub visibility_level: i64,
    pub path_with_namespace: String,
    pub default_branch: String,
    pub homepage: String,
    pub url: String,
    pub ssh_url: String,
    pub http_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Target {
    pub name: String,
    pub description: String,
    pub web_url: String,
    pub avatar_url: Option<String>,
    pub git_ssh_url: String,
    pub git_http_url: String,
    pub namespace: String,
    pub visibility_level: i64,
    pub path_with_namespace: String,
    pub default_branch: String,
    pub homepage: String,
    pub url: String,
    pub ssh_url: String,
    pub http_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LastCommit {
    pub id: String,
    pub message: String,
    pub timestamp: String,
    pub url: String,
    pub author: Author,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Author {
    pub name: String,
    pub email: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Assignee {
    pub name: String,
    pub username: String,
    pub avatar_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Label {
    pub id: i64,
    pub title: String,
    pub color: String,
    pub project_id: i64,
    pub created_at: String,
    pub updated_at: String,
    pub template: bool,
    pub description: String,
    pub type_field: Option<String>,
    pub group_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Changes {
    pub updated_by_id: UpdatedById,
    pub updated_at: UpdatedAt,
    pub labels: Labels,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdatedById {
    pub previous: Option<i64>,
    pub current: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdatedAt {
    pub previous: String,
    pub current: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Labels {
    pub previous: Vec<Previou>,
    pub current: Vec<Current>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Previou {
    pub id: i64,
    pub title: String,
    pub color: String,
    pub project_id: i64,
    pub created_at: String,
    pub updated_at: String,
    pub template: bool,
    pub description: String,
    pub type_field: Option<String>,
    pub group_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Current {
    pub id: i64,
    pub title: String,
    pub color: String,
    pub project_id: i64,
    pub created_at: String,
    pub updated_at: String,
    pub template: bool,
    pub description: String,
    pub type_field: Option<String>,
    pub group_id: i64,
}

// #[post("/webhook")]
pub async fn webhook(body: web::Json<Request>, key: web::Data<&str>) -> impl Responder {
    println!("{:?}", body);
    notify(key.get_ref()).await;
    HttpResponse::Ok().body("")
}
