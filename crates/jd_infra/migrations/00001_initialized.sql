CREATE SCHEMA IF NOT EXISTS "user";
CREATE SCHEMA IF NOT EXISTS "course";

CREATE TABLE IF NOT EXISTS "user"."tbl_users" (
       pk_user_id BIGINT PRIMARY KEY NOT NULL,
       username VARCHAR(150) NOT NULL
);

CREATE TABLE IF NOT EXISTS "course"."tbl_courses" (
       pk_course_id BIGINT PRIMARY KEY NOT NULL,
       title VARCHAR(150) NOT NULL,
       description TEXT
)
