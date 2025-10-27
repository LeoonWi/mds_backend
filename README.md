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

#### Установка базовых значений

Имея готовые к работе таблицы, необходимо загрузить в них некоторые данные, которые необходимы для работы системы.

Для Windows:

```powershell
Get-Content init.sql | docker exec -i {db_container_name} psql -U {POSTGRES_USER} -d {POSTGRES_DB}

```

Для Linux:

```sh
docker exec -i {db_container_name} psql -U {POSTGRES_USER} -d {POSTGRES_DB} < init.sql
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

Кроме описаных ниже статус кодов или ошибок, сервер также может вернуть и другие значения, которые описаны здесь.

- **415** `Unsupported Media Type` — это клиентская ошибка HTTP, которая указывает, что сервер отказывается принимать запрос, потому что формат содержимого в теле запроса не поддерживается (например, JSON вместо XML или form-data вместо ожидаемого типа).
  **Возвращает строку.**
- **422** `Unprocessable Entity` — это ошибка валидации. Сервер понял запрос, но не может обработать содержимое из-за семантических ошибок.
  Например, поле с типом `number` передать `string` или в _обязательное_ поле записать `null`.
  **Возвращает строку.**
- **405** `Method Not Allowed` — это ошибка некорректного роутинга. Например, в `POST /clients` попытаться обраться с помощью `PUT`.
  **\*Возвращает строку.**

Общие объекты:

#### AppError

```typescript
type AppError = {
  timestamp: string; // время в формате UTC
  message: string;
};
```

### Роли

#### GET `/roles`

Возвращает всегда **массив** объектов **(ролей)** в JSON. Массив может оказаться **пустым**, если возникнет ошибка или ролей не будет.

```typescript
type Role = {
  name: string;
  created_at: string;
  updated_at: string | null;
};

type Response = Role[];
```

### Тарифы

#### GET `/tariffs`

Возвращает всегда **массив** объектов **(тарифов)** в JSON. Массив может оказаться **пустым**, если возникнет ошибка или тарифов не будет.

```typescript
type Tariff = {
  name: string;
  created_at: string;
  updated_at: string | null;
};

type Response = Tariff[];
```

### Клиенты

#### POST `/signup`

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

#### GET `/clients`

Принимает в себя JSON объект **(FilterClient)**. Все поля опциональные!!!.

```typescript
type FilterClient = {
  name: string | undefined;
  last_name: string | undefined;
  middle_name: string | undefined;
  email: string | undefined;
  phone: string | undefined;
  tariff: string | undefined;
};
```

Всегда возвращает статус **200** JSON массив объектов **(клиентов)**.

```typescript
type Tariff = {
  name: string;
  created_at: string;
  updated_at: string | null;
};

type Client = {
  id: number;
  name: string;
  last_name: string;
  middle_name: string | null;
  email: string;
  phone: string;
  tariff: Tariff;
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

#### GET `/employee`

Принимает JSON объект **(FilterEmployee)** в качестве фильтра. Все поля опциональные, т.е. могут полностью отсутствовать.

UPD: `dismissed = false` чтобы получить список только работающих сотрудников.

```typescript
type FilterEmployee = {
  name: string | underfined;
  last_name: string | underfined;
  middle_name: string | underfined;
  email: string | underfined;
  role: string | underfined;
  dismissed: string | underfined;
};
```

Возвращает всегда JSON массив объектов **Employee** и статус **200**.

```typescript
type Role = {
  name: string;
  created_at: string;
  updated_at: string | null;
};

type Employee = {
  id: number;
  name: string;
  last_name: string;
  middle_name: string | null;
  email: string;
  role: Role;
  dismissed: boolean;
  created_at: string;
  updated_at: string | null;
};

type Response = Employee[];
```

#### PATCH `/employee/dismiss/{email}`

Принимает строку email в пути запроса и возвращает:

- **200** в случае УСПЕХА;
- **404** и ошибку (AppError) [(см. выше)](#apperror) в случае ПРОВАЛА (Не найден пользователь);

P.S. это запрос для увольнения сотрудника.
