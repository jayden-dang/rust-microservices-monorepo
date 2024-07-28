mod routes;
mod services;

use jd_infra::base::{CommonId, DMC};
// pub use services::routes;

// -->>> Region:: START  --->>>  Data Model Control
pub struct CourseDmc;

impl DMC for CourseDmc {
  const SCHEMA: &'static str = "course";
  const TABLE: &'static str = "tbl_courses";
  const ID: jd_infra::base::CommonId = CommonId::PkCourseId;
}
// <<<-- Region:: END    <<<---  Data Model Control
