use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::models::client::Client;
use crate::models::employee::Employee;
use crate::models::role::Role;
use crate::models::service::Service;
use crate::models::tariff::Tariff;

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "priority", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum Priority {
    High,
    Normal,
    Low,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Status {
    New,
    Awaiting,
    InWork,
    Assigned,
    OnReview,
    OnApproval,
    Approved,
    Rejected,
}

#[derive(Debug, FromRow)]
pub struct RequestFlat {
    pub id: i64,
    pub name: String,
    pub priority: Priority,
    pub desc: String,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub desired_at: DateTime<Utc>,
    pub closed_at: Option<DateTime<Utc>>,

    // service
    pub service_name: Option<String>,
    pub service_created_at: Option<DateTime<Utc>>,
    pub service_updated_at: Option<DateTime<Utc>>,

    // owner
    pub owner_id: i64,
    pub owner_name: String,
    pub owner_last_name: String,
    pub owner_middle_name: Option<String>,
    pub owner_email: String,
    pub owner_phone: String,
    pub owner_password: String,
    pub owner_inn: Option<String>,
    pub owner_snils: String,
    pub owner_created_at: DateTime<Utc>,
    pub owner_updated_at: Option<DateTime<Utc>>,
    pub owner_tariff_name: String,
    pub owner_tariff_created_at: DateTime<Utc>,
    pub owner_tariff_updated_at: Option<DateTime<Utc>>,

    // employee
    pub employee_id: i64,
    pub employee_name: String,
    pub employee_last_name: String,
    pub employee_middle_name: Option<String>,
    pub employee_email: String,
    pub employee_password: String,
    pub employee_dismissed: bool,
    pub employee_created_at: DateTime<Utc>,
    pub employee_updated_at: Option<DateTime<Utc>>,
    pub employee_role_name: String,
    pub employee_role_created_at: DateTime<Utc>,
    pub employee_role_updated_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize)]
pub struct Request {
    id: i64,
    name: String,
    service: Option<Service>,
    owner: Client,
    employee: Employee,
    priority: Priority,
    desc: String,
    status: Status,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
    desired_at: DateTime<Utc>,
    closed_at: Option<DateTime<Utc>>,
}

impl From<RequestFlat> for Request {
    fn from(value: RequestFlat) -> Self {
        Request {
            id: value.id,
            name: value.name,
            service: match value.service_name {
                Some(name) => Some(Service {
                    name: name,
                    created_at: value.service_created_at.unwrap(),
                    updated_at: value.service_updated_at,
                }),
                None => None,
            },
            owner: Client {
                id: value.owner_id,
                name: value.owner_name,
                last_name: value.owner_last_name,
                middle_name: value.owner_middle_name,
                email: value.owner_email,
                phone: value.owner_phone,
                tariff: Tariff {
                    name: value.owner_tariff_name,
                    created_at: value.owner_tariff_created_at,
                    updated_at: value.owner_tariff_updated_at,
                },
                inn: value.owner_inn,
                snils: value.owner_snils,
                created_at: value.owner_created_at,
                updated_at: value.owner_updated_at,
            },
            employee: Employee {
                id: value.employee_id,
                name: value.employee_name,
                last_name: value.employee_last_name,
                middle_name: value.employee_middle_name,
                email: value.employee_email,
                role: Role {
                    name: value.employee_role_name,
                    created_at: value.employee_role_created_at,
                    updated_at: value.employee_role_updated_at,
                },
                dismissed: value.employee_dismissed,
                created_at: value.employee_created_at,
                updated_at: value.employee_updated_at,
            },
            priority: value.priority,
            desc: value.desc,
            status: value.status,
            created_at: value.created_at,
            updated_at: value.updated_at,
            desired_at: value.desired_at,
            closed_at: value.closed_at,
        }
    }
}

#[derive(Deserialize)]
pub struct CreateRequest {
    pub name: String,
    pub service: String,
    pub owner_id: i64,
    pub employee_id: i64,
    pub priority: Priority,
    pub desc: String,
    pub desired_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct FilterRequest {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub service: Option<String>,
    pub owner_id: Option<i64>,
    pub employee_id: Option<i64>,
    pub priority: Option<Priority>,
    pub status: Option<Status>,
    pub desired_at: Option<DateTime<Utc>>,
}
