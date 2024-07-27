mod routes;
mod services;

use jd_infra::user::{CommonId, DMC};
pub use routes::routes;

// -->>> Region:: START  --->>>  Data Model Control
pub struct CourseDmc;

impl DMC for CourseDmc {
  const SCHEMA: &'static str = "course";
  const TABLE: &'static str = "tbl_courses";
  const ID: jd_infra::user::CommonId = CommonId::PkCourseId;
}
// <<<-- Region:: END    <<<---  Data Model Control
