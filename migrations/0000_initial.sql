CREATE SCHEMA IF NOT EXISTS "user";

CREATE TABLE IF NOT EXISTS "user"."tbl_users" (
       pk_user_id SERIAL PRIMARY KEY,
       username VARCHAR(150)
)
