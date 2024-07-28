#[macro_export]
macro_rules! gen_common_fn {
  (
    DMC: $struct_name:ident,
    Entity: $entity:ty,
    $(ReqCreate: $req_create:ty,)?
      $(ResCreate: $res_create:ty,)?
      $(ReqUpdate: $req_update:ty,)?
      $(Filter: $req_get_filter:ty,)?
      $(Route: $route:expr,)?
  ) => {
    use serde_json::{json, Value};
    use sqlx::PgPool;
    use modql::filter::ListOptions;
    use axum::{
      extract::{Path, Query, State},
      routing::{delete, get, patch, post, put},
      Json, Router
    };

    impl $struct_name {
      $(
        pub async fn create(
          State(db): State<PgPool>,
          Json(req): Json<$req_create>
        ) -> AppResult<Json<$res_create>> {
          Ok(Json(jd_infra::base::create::<Self, _, _>(db, req).await?))
        }

        pub async fn create_many(
          State(db): State<PgPool>,
          Json(req): Json<Vec<$req_create>>,
        ) -> AppResult<Json<Vec<$res_create>>> {
          Ok(Json(jd_infra::base::create_many::<Self, _, _>(db, req).await?))
        }
      )?
        $(
          pub async fn get_by_id(
            Path(id): Path<i64>,
            State(db): State<PgPool>
          ) -> AppResult<Json<$entity>> {
            Ok(Json(jd_infra::base::get_by_id::<Self, _>(&db, id).await?))
          }

          pub async fn get_by_sth(
            Query(query): Query<$req_get_filter>,
            Query(list_options): Query<ListOptions>,
            State(db): State<PgPool>,
          ) -> AppResult<Json<Option<$entity>>> {
            Ok(Json(jd_infra::base::first::<Self, _, _>(&db, Some(query), Some(list_options)).await?))
          }

          pub async fn list(
            Query(query): Query<$req_get_filter>,
            Query(list_options): Query<ListOptions>,
            State(db): State<PgPool>,
          ) -> AppResult<Json<Value>> {
            let (users, pagination) = jd_infra::base::list::<Self, _, $entity>(&db, Some(query), Some(list_options)).await?;

            Ok(Json(json!({
              "data": users,
              "metadata": pagination
            })))
          }

          pub async fn count(
            Query(query): Query<$req_get_filter>,
            State(db): State<PgPool>
          ) -> AppResult<Json<i64>> {
            Ok(Json(jd_infra::base::count::<Self, _>(&db, Some(query)).await?))
          }
        )?
        $(
          pub async fn update(
            Path(id): Path<i64>,
            State(db): State<PgPool>,
            Json(req): Json<$req_update>,
          ) -> AppResult<()> {
            jd_infra::base::update::<Self, _>(&db, id, req).await
          }

          pub async fn delete(
            State(db): State<PgPool>,
            Path(req): Path<i64>
          ) -> AppResult<()> {
            jd_infra::base::delete::<Self>(&db, req).await
          }

          pub async fn delete_many(
            State(db): State<PgPool>,
            Json(req): Json<Vec<i64>>
          ) -> AppResult<()> {
            jd_infra::base::delete_many::<Self>(&db, req).await
          }
        )?
    }

    $(
      pub fn routes() -> Router<PgPool> {
        Router::new()
          .route(&format!("/{}", $route), post($struct_name::create))
          .route(
            &format!("/{}/:id", $route),
            get($struct_name::get_by_id).delete($struct_name::delete).patch($struct_name::update),
          )
          .route(&format!("/{}s", $route), get($struct_name::list))
          .route(&format!("/{}s", $route), delete($struct_name::delete_many))
          .route(&format!("/{}", $route), get($struct_name::get_by_sth))
          .route(&format!("/{}/count", $route), get($struct_name::count))
      }
    )?
  };
}
