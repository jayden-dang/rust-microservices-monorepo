mod routes;
mod services;

use jd_infra::user::DMC;
pub use routes::routes;

// -->>> Region:: START  --->>>  Data Model Control
pub struct CourseDmc;

impl DMC for CourseDmc {
  const SCHEMA: &'static str = "course";
  const TABLE: &'static str = "tbl_courses";
}
// <<<-- Region:: END    <<<---  Data Model Control
