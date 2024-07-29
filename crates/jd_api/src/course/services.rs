use jd_core::AppResult;
use jd_domain::user::{
  Course, CourseFilter, RequestCreateCourse, RequestUpdateCourse, ResponseCreateCourse,
};
use jd_infra::gen_common_fn;

use super::CourseDmc;

gen_common_fn! {
  DMC: CourseDmc,
  Entity: Course,
  ReqCreate: RequestCreateCourse,
  ResCreate: ResponseCreateCourse,
  ReqUpdate: RequestUpdateCourse,
  Filter: CourseFilter,
}
