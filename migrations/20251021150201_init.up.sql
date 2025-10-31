-- Add migration script here
CREATE TYPE "tariff" AS ENUM ('free', 'business');
CREATE TYPE "role" AS ENUM ('superuser', 'manager', 'employee', 'guest');
CREATE TYPE "priority" AS ENUM ('high', 'normal', 'low');
CREATE TYPE "status" AS ENUM ('new', 'awaiting', 'in_work', 'assigned', 'on_review', 'on_approval', 'approved', 'rejected');

CREATE TABLE IF NOT EXISTS "user" (
    "id" BIGSERIAL NOT NULL PRIMARY KEY,
    "name" CHARACTER VARYING(100) NOT NULL,
    "last_name" CHARACTER VARYING(100) NOT NULL,
    "middle_name" CHARACTER VARYING(100),
    "email" CHARACTER VARYING(255) NOT NULL UNIQUE,
    "phone" CHARACTER VARYING(12) NOT NULL UNIQUE,
    "password" TEXT NOT NULL,
    "tariff" tariff NOT NULL,
	"inn" CHARACTER VARYING(12) UNIQUE,
	"snils" CHARACTER VARYING(11) UNIQUE,
	"created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
	"updated_at" TIMESTAMPTZ
);

CREATE UNIQUE INDEX idx_user_email ON "user"("email");
CREATE UNIQUE INDEX idx_user_phone ON "user"("phone");

CREATE TABLE IF NOT EXISTS "service" (
    "name" TEXT NOT NULL PRIMARY KEY,
	"created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
	"updated_at" TIMESTAMPTZ
);

CREATE TABLE IF NOT EXISTS "employee" (
	"id" BIGSERIAL NOT NULL PRIMARY KEY,
	"name" CHARACTER VARYING(100) NOT NULL,
    "last_name" CHARACTER VARYING(100) NOT NULL,
    "middle_name" CHARACTER VARYING(100),
    "email" CHARACTER VARYING(255) NOT NULL UNIQUE,
    "password" TEXT NOT NULL,
	"role" role NOT NULL,
	"dismissed" BOOLEAN NOT NULL,
	"created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
	"updated_at" TIMESTAMPTZ
);

CREATE INDEX idx_employee_email ON "employee"("email");

CREATE TABLE IF NOT EXISTS "employee_specs" (
	"employee_id" BIGSERIAL NOT NULL REFERENCES "employee" ON UPDATE CASCADE ON DELETE CASCADE,
	"service" TEXT NOT NULL REFERENCES "service"("name") ON UPDATE CASCADE ON DELETE CASCADE,
	PRIMARY KEY ("employee_id", "service")
);

CREATE TABLE IF NOT EXISTS "request" (
	"id" BIGSERIAL NOT NULL PRIMARY KEY,
	"name" CHARACTER VARYING(255) NOT NULL,
	"service" TEXT REFERENCES "service" ON UPDATE CASCADE ON DELETE SET NULL,
	"owner_id" BIGINT NOT NULL REFERENCES "user" ON UPDATE CASCADE ON DELETE CASCADE,
	"employee_id" BIGINT REFERENCES "employee" ON UPDATE CASCADE ON DELETE SET NULL,
	"priority" priority NOT NULL,
	"desc" TEXT NOT NULL,
	"status" status NOT NULL,
	"created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
	"updated_at" TIMESTAMPTZ,
	"desired_at" TIMESTAMPTZ NOT NULL,
	"closed_at" TIMESTAMPTZ
);

CREATE INDEX idx_employee_priority ON "request"("priority");
CREATE INDEX idx_employee_status ON "request"("status");