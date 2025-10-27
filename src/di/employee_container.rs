use std::sync::Arc;

use sqlx::PgPool;

use crate::adapters::default_value_adapter::DefaultValueRepository;
use crate::adapters::employee_adapter::EmployeeRepository;
use crate::application::employee::EmployeeLogic;

pub struct EmployeeContainer {
    pub employee_repo: Arc<EmployeeRepository>,
    pub logic: EmployeeLogic<EmployeeRepository, DefaultValueRepository>,
}

impl EmployeeContainer {
    pub fn new(postgres: Arc<PgPool>, default_value_repo: Arc<DefaultValueRepository>) -> Self {
        let employee_repo = Arc::new(EmployeeRepository::new(postgres));
        let logic = EmployeeLogic::new(employee_repo.clone(), default_value_repo);

        EmployeeContainer {
            employee_repo,
            logic,
        }
    }
}
