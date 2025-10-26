BEGIN;

-- Тарифы
INSERT INTO tariff (name) VALUES ('Free');
INSERT INTO tariff (name) VALUES ('Business');

-- Роли
INSERT INTO role (name) VALUES ('Employee');
INSERT INTO role (name) VALUES ('Manager');
INSERT INTO role (name) VALUES ('Superuser');

-- Дефолтные значения (например, тариф + роль)
INSERT INTO default_value (tariff, role) VALUES ('Free', 'Employee');

COMMIT;