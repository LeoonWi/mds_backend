use std::sync::Arc;

use sqlx::PgPool;

use crate::adapters::employee_adapter::EmployeeRepository;
use crate::application::employee::EmployeeLogic;

pub struct EmployeeContainer {
    pub employee_repo: Arc<EmployeeRepository>,
    pub logic: EmployeeLogic<EmployeeRepository>,
}

impl EmployeeContainer {
    pub fn new(postgres: Arc<PgPool>) -> Self {
        let employee_repo = Arc::new(EmployeeRepository::new(postgres));
        let logic = EmployeeLogic::new(employee_repo.clone());

        EmployeeContainer {
            employee_repo,
            logic,
        }
    }
}
