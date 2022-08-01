use crate::models::mats::{Material, NewMaterial};
use crate::schema::materials::dsl::*;
use crate::Pool;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use actix_web::{web, Error, HttpResponse};
use diesel::dsl::{delete, insert_into};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InptItem {
    pub item: String,
    pub density: f32,
}


pub async fn get_item(
    db: web::Data<Pool>, 
    item_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let result = materials
        .find(item_id.into_inner())
        .get_result::<Material>(&db.get().unwrap());

    Ok(
        web::block(move || result)
            .await
            .map(|other_item| HttpResponse::Ok().json(other_item))
            .map_err(|_| HttpResponse::NotFound().json("Not Found"))?,
    )
}

pub async fn get_items(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let result = materials.load::<Material>(&db.get().unwrap());

    Ok(
        web::block(move || result)
            .await
            .map(|other_item| HttpResponse::Ok().json(other_item))
            .map_err(|_| HttpResponse::NotFound().json("Not Found"))?,
    )
}

pub async fn create_item(
    db: web::Data<Pool>,
    input_item: web::Json<InptItem>,
) -> Result<HttpResponse, Error> {
    let new_item = NewMaterial {
        item: &input_item.item,
        density: &input_item.density,
    };
    let inserted_cnt = insert_into(materials)
        .values(&new_item)
        .execute(&db.get().unwrap());
        
    Ok(web::block(move || inserted_cnt)
        .await
        .map(|other_item| HttpResponse::Created().json(other_item))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn delete_item(
    db: web::Data<Pool>,
    item_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let cnt = delete(materials.find(item_id.as_ref())).execute(&db.get().unwrap());

    Ok(
        web::block(move || cnt)
            .await
            .map(|other_item| HttpResponse::Ok().json(other_item))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}