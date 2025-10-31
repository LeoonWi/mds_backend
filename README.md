# Документация MDS

UPD: Все значения в '{...}' брать из .env или compose.dev.yaml

## Инициализация системы

#### Docker

Загрузка и запуск всех контейнеров.

```sh
docker compose -f compose.dev.yaml up -d
```

#### Миграции

После старта базы данных необходимо выполнить миграции для создания всех актуальных таблиц.

```sh
sqlx migrate run --database-url=postgres://{POSTGRES_USER}:{POSTGRES_PASSWORD}@{POSTGRES_HOST}:{POSTGRES_PORT}/{POSTGRES_DB}?{args}

```

Например:

```sh
sqlx migrate run --database-url=postgres://postgres:admin@localhost:5433/mds?sslmode=disable

```

#### Сборка и запуск сервера

##### Сборка

```sh
cargo build -r
```

##### Запуск

Для Windows:
Выполнять единожды! Создаёт уч. запись суперпользователя.

```powershell
./target/release/init.exe
```

Запуск сервера:

```powershell
./target/release/mds_backend.exe
```

Для Linux:

```sh
./target/release/init
```

```sh
./target/release/mds_backend
```

## Сервер

### Информация перед использованием

#### Аутентификация

Для обработчиков которые работают без авторизации, рядом с URL путем будет надпись **ОТКРЫТ**. Для всех остальных необходимо передавать заголовок **Authorization** и **access_token** с префиксом **Bearer** через пробел. Ниже пример.
Пример заголовка **Authorization**.
`Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIxIiwicm9sZSI6InN1cGVydXNlciIsImV4cCI6MTc2MjUyNTM0NX0.ZJeMFYf1v8e-AdPvb1J2ad426n04QLmapn2Yp7fBFZo`

#### Ошибки

Кроме описаных ниже статус кодов или ошибок, сервер также может вернуть и другие значения, которые описаны здесь.

- **415** `Unsupported Media Type` — это клиентская ошибка HTTP, которая указывает, что сервер отказывается принимать запрос, потому что формат содержимого в теле запроса не поддерживается (например, JSON вместо XML или form-data вместо ожидаемого типа).
  **Возвращает строку.**
- **422** `Unprocessable Entity` — это ошибка валидации. Сервер понял запрос, но не может обработать содержимое из-за семантических ошибок.
  Например, поле с типом `number` передать `string` или в _обязательное_ поле записать `null`.
  **Возвращает строку.**
- **405** `Method Not Allowed` — это ошибка некорректного роутинга. Например, в `POST /clients` попытаться обраться с помощью `PUT`.
  **\*Возвращает строку.**
- **401** `Unauthorized` — запрос требует аутентификации (jwt_token). Клиент не предоставил валидные учётные данные.
- **403** `Forbidden` — клиент аутентифицирован, но не имеет прав на доступ к ресурсу.
  Например, обычный пользователь пытается удалить админский пост.

#### Общие объекты:

#### AppError

```typescript
type AppError = {
  timestamp: string; // время в формате UTC
  message: string;
};
```

### Клиенты

#### POST `/signup` ОТКРЫТ

Принимает в себя объект **(CreateClient)** в JSON.

```typescript
type CreateClient = {
  name: string;
  last_name: string;
  middle_name: string | undefined;
  email: string;
  phone: string;
  password: string; // len() >= 6
  inn: string | undefined; // len() == 10 | 12
  snils: string; // len() == 11
};
```

Возвращает статус код **201** в случае _УСПЕХА_ или ошибку **(AppError)** [(см. выше)](#apperror) в JSON в случае _ПРОВАЛА_.

#### POST `/clients/login` ОТКРЫТ

Принимает JSON объект **(LoginClients)**.

```typescript
type LoginClients = {
  email: string;
  password: string;
};
```

Вернет статус **200** и JSON объект (**Token**) в случае _УСПЕХА_ или ошибку **(AppError)** [(см. выше)](#apperror) в случае _ПРОВАЛА_.

```typescript
type Token = {
  access_token: string;
  refresh_token: string;
};
```

#### POST `/clients/refresh_token`

В заголовке авторизации передать **ВНИМАНИЕ** - `refresh_token`. На выходе вернет статус **200** и JSON объект (**Token**) в случае УСПЕХА или ошибку **(AppError)** [(см. выше)](#apperror) в случае _ПРОВАЛА_.

```typescript
type Token = {
  access_token: string;
  refresh_token: string;
};
```

#### GET `/clients`

Принимает в себя JSON объект **(FilterClient)**. Все поля опциональные!!!.

UPD: Добавил поле id.

```typescript
type Tariff = "free" | "business";

type FilterClient = {
  id: number | underfined;
  name: string | undefined;
  last_name: string | undefined;
  middle_name: string | undefined;
  email: string | undefined;
  phone: string | undefined;
  tariff: Tariff | undefined;
};
```

Всегда возвращает статус **200** JSON массив объектов **(клиентов)**.

```typescript
type Tariff = "free" | "business";

type Client = {
  id: number;
  name: string;
  last_name: string;
  middle_name: string | null;
  email: string;
  phone: string;
  tariff: Tariff; // это string
  inn: string | null;
  snils: string;
  created_at: string;
  updated_at: string;
};

type Response = Clients[];
```

#### DELETE `/clients/{email}`

Возвращает статус код **200** в случае _УСПЕХА_ или ошибку **(AppError)** [(см. выше)](#apperror) в JSON в случае _ПРОВАЛА_.

### Сотрудники

#### POST `/employee`

Принимает JSON объект **(CreateEmployee)** и возвращает **201** в случае _УСПЕХА_ или ошибку **(AppError)** [(см. выше)](#apperror) в случае _ПРОВАЛА_.

```typescript
type CreateEmployee = {
  name: string;
  last_name: string;
  middle_name: string | underfined;
  email: string;
  password: string; // len() >= 6
};
```

#### POST `/employee/login` ОТКРЫТ

Принимает JSON объект **(LoginEmployee)**.

```typescript
type LoginEmployee = {
  email: string;
  password: string;
};
```

Вернет статус **200** и JSON объект (**Token**) в случае _УСПЕХА_ или ошибку **(AppError)** [(см. выше)](#apperror) в случае _ПРОВАЛА_.

```typescript
type Token = {
  access_token: string;
  refresh_token: string;
};
```

#### POST `/employee/refresh_token`

В заголовке авторизации передать **ВНИМАНИЕ** - `refresh_token`. На выходе вернет статус **200** и JSON объект (**Token**) в случае УСПЕХА или ошибку **(AppError)** [(см. выше)](#apperror) в случае _ПРОВАЛА_.

```typescript
type Token = {
  access_token: string;
  refresh_token: string;
};
```

#### GET `/employee`

Принимает JSON объект **(FilterEmployee)** в качестве фильтра. Все поля опциональные, т.е. могут полностью отсутствовать.

UPD: `dismissed = false` чтобы получить список только работающих сотрудников. Обновил фильтр.

```typescript
type Role = "superuser" | "manager" | "employee";

type FilterEmployee = {
  id: number | underfined;
  name: string | underfined;
  last_name: string | underfined;
  middle_name: string | underfined;
  email: string | underfined;
  role: Role | underfined;
  dismissed: string | underfined;
};
```

Возвращает всегда JSON массив объектов **Employee** и статус **200**.

```typescript
type Role = "superuser" | "manager" | "employee";

type Employee = {
  id: number;
  name: string;
  last_name: string;
  middle_name: string | null;
  email: string;
  role: Role; // это тоже string
  dismissed: boolean;
  created_at: string;
  updated_at: string | null;
};

type Response = Employee[];
```

#### GET `/employee_with_services`

Принимает JSON объект **(FilterEmployee)** в качестве фильтра. Все поля опциональные, т.е. могут полностью отсутствовать.

```typescript
type Role = "superuser" | "manager" | "employee";

type FilterEmployee = {
  id: number | underfined;
  name: string | underfined;
  last_name: string | underfined;
  middle_name: string | underfined;
  email: string | underfined;
  role: Role | underfined;
  dismissed: string | underfined;
};
```

Возвращает всегда JSON массив объектов **EmployeeWithServices** и статус **200**.

```typescript
type Role = "superuser" | "manager" | "employee";

type EmployeeWithServices = {
  id: number;
  name: string;
  last_name: string;
  middle_name: string | null;
  email: string;
  role: Role; // это тоже string
  dismissed: boolean;
  created_at: string;
  updated_at: string | null;
  services: string[];
};

type Response = EmployeeWithServices[];
```

#### PATCH `/employee/{email}/set_role={role}`

Принимает 2 строки:

- email
- role = `Role = "superuser" | "manager" | "employee"`

Возвращает:

- **200** в случае _УСПЕХА_;
- **404** и ошибку (**AppError**) [(см. выше)](#apperror) в случае _ПРОВАЛА_ (Не найден пользователь);

#### PATCH `/employee/{id}/add_service={service}`

Принимает id сотрудника (`number`) и название услуги (`string`).
Вернет **200** в случае _УСПЕХА_ или ошибку (**AppError**) [(см. выше)](#apperror) в случае _ПРОВАЛА_.

#### PATCH `/employee/{id}/remove_service={service}`

Принимает id сотрудника (`number`) и название услуги (`string`).
Вернет **200** в случае _УСПЕХА_ или ошибку (**AppError**) [(см. выше)](#apperror) в случае _ПРОВАЛА_.

#### PATCH `/employee/dismiss/{email}`

Принимает строку email в пути запроса и возвращает:

- **200** в случае УСПЕХА;
- **404** и ошибку (AppError) [(см. выше)](#apperror) в случае ПРОВАЛА (Не найден пользователь);

P.S. это запрос для увольнения сотрудника.

### Услуги

#### POST `/services`

Принимает JSON объект **(CreateService)** и возвращает **201** в случае _УСПЕХА_ или ошибку **(AppError)** [(см. выше)](#apperror) в случае _ПРОВАЛА_.

```typescript
type CreateService = {
  name: String;
};
```

#### GET `/services` ОТКРЫТ

Всегда возвращает статус **200** и массив объектов **(Service)** JSON.

```typescript
type Service = {
  name: string;
  created_at: string;
  updated_at: string | null;
};

type Response = Service[];
```

#### GET `/services/{name}` ОТКРЫТ

Принимает в параметре URL имя сервиса _(string)_ и возвращает JSON объект **(Service)** со статусом **200**. Ошибку **(AppError)** [(см. выше)](#apperror) если объект был не найден.

```typescript
type Service = {
  name: string;
  created_at: string;
  updated_at: string | null;
};
```

#### DELETE `/services/{name}`

Принимает в параметре URL имя сервиса _(string)_ и возвращает **200**, если объект был удален успешно. Вернет ошибку **(AppError)** [(см. выше)](#apperror) если объект был не найден.

### Заявки

#### POST `/requests`

Принимает JSON объект **(CreateRequest)** и возвращает **201** в случае _УСПЕХА_. В случае _ПРОВАЛА_ вернет ошибку **(AppError)** [(см. выше)](#apperror).

```typescript
type CreateRequest = {
  name: string;
  service: string;
  owner_id: number;
  employee_id: number;
  priority: "high" | "normal" | "low";
  desc: string;
  desired_at: string; // datetime in format utc
};
```

#### GET `/requests`

Принимает JSON объект **(FilterRequest)**.

```typescript
type Priority =
  | "new"
  | "awaiting"
  | "in_work"
  | "assigned"
  | "on_review"
  | "on_approval"
  | "approved"
  | "rejected";

type Status = "high" | "normal" | "low";

type FilterRequest = {
  id: number | undefined;
  name: string | undefined;
  service: string | undefined;
  owner_id: number | undefined;
  employee_id: number | undefined;
  priority: Priority | undefined;
  status: Status | undefined;
  desired_at: string | undefined;
};
```

Возвращает JSON массив объектов **(Request)** и статус **200**. Массив может быть пустым, если объекты не найдены.

#### PATCH `/requests/{id}/set_status={status}`

Принимает в параметры запроса **id** заявки типа `number` и **status** типа `string` с возможными варинтами `'high'` | `'normal'` | `'low'`.

Вернет статус **200** в случае _УСПЕХА_ или ошибку **(AppError)** [(см. выше)](#apperror) в случае _ПРОВАЛА_.

#### PATCH `/requests/{id}/set_priority={priority}`

Принимает в параметры запроса **id** заявки типа `number` и **priority** типа `Priority (string)`.

```typescript
type Priority =
  | "new"
  | "awaiting"
  | "in_work"
  | "assigned"
  | "on_review"
  | "on_approval"
  | "approved"
  | "rejected";
```

Вернет статус **200** в случае _УСПЕХА_ или ошибку **(AppError)** [(см. выше)](#apperror) в случае _ПРОВАЛА_.

#### PATCH `/requests/{id}/set_employee={employee_id}`

Принимает в параметры запроса **id** заявки типа `number` и **employee_id** типа `number`.

Вернет статус **200** в случае _УСПЕХА_ или ошибку **(AppError)** [(см. выше)](#apperror) в случае _ПРОВАЛА_.

#### DELETE `/requests/{id}`

Принимает в параметры запроса **id** заявки типа `number` и возвращает статус **200** в случае _УСПЕХА_ или ошибку **(AppError)** [(см. выше)](#apperror) в случае _ПРОВАЛА_.
