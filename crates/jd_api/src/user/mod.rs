use jd_infra::user::DMC;

mod routes;
mod services;

pub use routes::routes;

pub struct UserDmc;

impl DMC for UserDmc {
  const SCHEMA: &'static str = "user";
  const TABLE: &'static str = "tbl_users";
}
