use mds_backend::adapters;
use mds_backend::config;
use mds_backend::di;
use mds_backend::models::employee::CreateEmployee;

#[tokio::main]
async fn main() {
    let config = config::Config::build().expect("Failed to build config");
    let postgres = adapters::pg_connect(&config.db_url, 2).expect("Failed to connect postgres");

    let default_val = di::default_value_container::DefaultValueContainer::new(postgres.clone());
    let employee =
        di::employee_container::EmployeeContainer::new(postgres.clone(), default_val.repo.clone());

    let email = "furiblack904@gmail.com".to_owned();
    let password = "LetsStartToUseMDS".to_owned();

    let payload = CreateEmployee {
        name: "Данил".to_owned(),
        last_name: "Ромашкан".to_owned(),
        middle_name: Some("Алексеевич".to_owned()),
        email: email.clone(),
        password: password.clone(),
        role: Some("Superuser".to_owned()),
    };
    employee
        .logic
        .create_employee(payload)
        .await
        .expect("Failed to create superuser");

    println!(
        "Superuser was created successfully. Data to sign-in:\nEmail:\t\t{email}\nPassword:\t{password}"
    );
}
