# Документация Restful MDS

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
