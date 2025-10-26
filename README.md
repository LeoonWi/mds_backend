# Документация MDS

UPD: Все значения в '{}' брать из .env или compose.dev.yaml

## Инициализация системы

#### Docker

Загрузка и запуск всех контейнеров.

```sh
> docker compose -f compose.dev.yaml up -d
```

#### Миграции

После старта базы данных необходимо выполнить миграции для создания всех актуальных таблиц.

```sh
> sqlx migrate run --database-url=postgres://{POSTGRES_USER}:{POSTGRES_PASSWORD}@{POSTGRES_HOST}:{POSTGRES_PORT}/{POSTGRES_DB}?{args}

```

Например:

```sh
> sqlx migrate run --database-url=postgres://postgres:admin@localhost:5433/mds?sslmode=disable

```

#### Установка базовых значений

Имея готовые к работе таблицы, необходимо загрузить в них некоторые данные, которые необходимы для работы системы.

Для Windows:

```powershell
> Get-Content init.sql | docker exec -i {db_container_name} psql -U {POSTGRES_USER} -d {POSTGRES_DB}

```

Для Linux:

```sh
> docker exec -i {db_container_name} psql -U {POSTGRES_USER} -d {POSTGRES_DB} < init.sql
```

## Роли

#### GET /roles

Возвращает всегда **массив** объектов **(ролей)** в JSON. Массив может оказаться **пустым**, если возникнет ошибка или ролей не будет.

```typescript
type Role = {
  name: string;
  created_at: string;
  updated_at: string | null;
};

type Response = Role[];
```

## Тарифы

#### GET /tariffs

Возвращает всегда **массив** объектов **(тарифов)** в JSON. Массив может оказаться **пустым**, если возникнет ошибка или тарифов не будет.

```typescript
type Tariff = {
  name: string;
  created_at: string;
  updated_at: string | null;
};

type Response = Tariff[];
```
