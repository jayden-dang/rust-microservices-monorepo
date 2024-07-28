use jd_core::AppResult;
use jd_domain::user::{
  request::{RequestCreateUser, RequestUpdateUser},
  User, UserFilter,
};
use jd_infra::gen_common_fn;

use super::UserDmc;

gen_common_fn!(
  DMC: UserDmc,
  Entity: User,
  ReqCreate: RequestCreateUser,
  ResCreate: User,
  ReqUpdate: RequestUpdateUser,
  Filter: UserFilter,
  Route: "user",
);

// impl UserDmc {
//   pub async fn create_user(State(db): State<PgPool>, Json(req): Json<RequestCreateUser>) -> AppResult<Json<User>> {
//     Ok(Json(jd_infra::base::create::<Self, _, _>(db, req).await?))
//   }

//   pub async fn create_many_user(
//     State(db): State<PgPool>,
//     Json(req): Json<Vec<RequestCreateUser>>,
//   ) -> AppResult<Json<Vec<User>>> {
//     Ok(Json(jd_infra::base::create_many::<Self, _, _>(db, req).await?))
//   }

//   pub async fn get_user_by_id(Path(id): Path<i64>, State(db): State<PgPool>) -> AppResult<Json<User>> {
//     Ok(Json(jd_infra::base::get_by_id::<Self, _>(&db, id).await?))
//   }

//   pub async fn get_user_by_sth(
//     Query(query): Query<UserFilter>,
//     Query(list_options): Query<ListOptions>,
//     State(db): State<PgPool>,
//   ) -> AppResult<Json<Option<User>>> {
//     if query.pk_user_id.is_none() && query.username.is_none() {
//       return Err(jd_core::error::AppError::BadRequest("At least one filter criteria must be provided".to_string()));
//     }
//     let user = jd_infra::base::first::<Self, _, _>(&db, Some(query), Some(list_options)).await?;
//     Ok(Json(user))
//   }

//   pub async fn get_user_sth(Query(query): Query<UserFilter>, State(db): State<PgPool>) -> AppResult<Json<User>> {
//     if query.pk_user_id.is_none() && query.username.is_none() {
//       return Err(jd_core::error::AppError::BadRequest("At least one filter criteria must be provided".to_string()));
//     }
//     let user = jd_infra::base::get_by_sth::<Self, _, _>(&db, Some(query)).await?;
//     Ok(Json(user))
//   }

//   pub async fn get_users(
//     Query(query): Query<UserFilter>,
//     Query(list_options): Query<ListOptions>,
//     State(db): State<PgPool>,
//   ) -> AppResult<Json<Value>> {
//     let (users, pagination) = jd_infra::base::list::<Self, _, User>(&db, Some(query), Some(list_options)).await?;

//     let response = json!({
//         "data": users,
//         "metadata": pagination
//     });
//     Ok(Json(response))
//   }

//   pub async fn count_users(Query(query): Query<UserFilter>, State(db): State<PgPool>) -> AppResult<Json<i64>> {
//     let count = jd_infra::base::count::<Self, _>(&db, Some(query)).await?;

//     Ok(Json(count))
//   }

//   pub async fn update_user(
//     Path(id): Path<i64>,
//     State(db): State<PgPool>,
//     Json(req): Json<RequestUpdateUser>,
//   ) -> AppResult<()> {
//     jd_infra::base::update::<Self, _>(&db, id, req).await
//   }

//   pub async fn delete_user(State(db): State<PgPool>, Path(req): Path<i64>) -> AppResult<()> {
//     jd_infra::base::delete::<Self>(&db, req).await
//   }

//   pub async fn delete_many_user(State(db): State<PgPool>, Json(req): Json<Vec<i64>>) -> AppResult<()> {
//     jd_infra::base::delete_many::<Self>(&db, req).await
//   }
// }
