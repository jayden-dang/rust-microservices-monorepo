use jd_infra::base::{CommonId, DMC};

mod services;
pub use services::routes;

pub struct UserDmc;

impl DMC for UserDmc {
  const SCHEMA: &'static str = "user";
  const TABLE: &'static str = "tbl_users";
  const ID: jd_infra::base::CommonId = CommonId::PkUserId;
}
