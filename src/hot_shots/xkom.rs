use chrono::{NaiveDateTime};
use rusqlite::{params, Connection};
use serde::Deserialize;
use std::{borrow::Borrow, marker::PhantomData};
use ureq::{Agent, Request, Response};

#[derive(Deserialize)]
pub(crate) struct XKomHotShot {
    #[serde(rename = "Id")]
    id: String,
    #[serde(rename = "Price")]
    price: f32,
    #[serde(rename = "OldPrice")]
    old_price: f32,
    #[serde(rename = "PromotionTotalCount")]
    promotion_total_count: u32,
    #[serde(rename = "SaleCount")]
    sale_count: u32,
    #[serde(rename = "MaxBuyCount")]
    max_buy_count: u32,
    #[serde(rename = "PromotionName")]
    promotion_name: String,
    #[serde(rename = "PromotionEnd")]
    promotion_end: NaiveDateTime,
    #[serde(rename = "Product")]
    product: XKomProduct,
}

#[derive(Deserialize)]
pub(crate) struct XKomProductCategory {
    #[serde(rename = "Id")]
    id: String,
    #[serde(rename = "NameSingular")]
    name_singular: String,
}

#[derive(Deserialize)]
pub(crate) struct XKomProduct {
    #[serde(rename = "Id")]
    id: String,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Category")]
    category: XKomProductCategory,
    #[serde(rename = "WebUrl")]
    web_url: String,
}

pub(crate) struct HotShot<'h>(PhantomData<&'h ()>);

impl<'h> crate::hot_shots::HotShotSource<'h> for HotShot<'h> {
    type Output = XKomHotShot;

    fn check_configuration(&self) -> Result<(), String> {
        Ok(())
            .and_then(|_| std::env::var("HS_XKOM_URL").map(|_| ()))
            .and_then(|_| std::env::var("HS_XKOM_API_KEY").map(|_| ()))
            .map_err(|err| err.to_string())
    }

    fn configure_scraper(&self, agent: &Agent) -> Request {
        agent
            .get(std::env::var("HS_XKOM_URL").unwrap().borrow())
            .set(
                "X-API-Key",
                std::env::var("HS_XKOM_API_KEY").unwrap().borrow(),
            )
            .set("Accept", "application/json")
    }

    fn store(&self, db_conn: &Connection, response: Self::Output) -> std::io::Result<()> {
        let exists = db_conn
            .query_row::<i64, _, _>(
                "SELECT 1 FROM hs_xkom WHERE id = ?",
                params![response.id],
                |row| row.get(0),
            )
            .and(Ok(true))
            .unwrap_or(false);

        if exists {
            return Ok(());
        }

        db_conn
            .execute(
                "
            INSERT INTO hs_xkom
            (
                id, price, old_price, promotion_total_count, sale_count, max_buy_count, promotion_name,
                promotion_end, product_id, product_name, product_category_id,
                product_category_name_singular, product_web_url
            )
            VALUES
            (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                ",
                params![
                    response.id,
                    response.price,
                    response.old_price,
                    response.promotion_total_count,
                    response.sale_count,
                    response.max_buy_count,
                    response.promotion_name,
                    response.promotion_end.timestamp(),
                    response.product.id,
                    response.product.name,
                    response.product.category.id,
                    response.product.category.name_singular,
                    response.product.web_url
                ],
            )
            .map_or_else(
                |x| Err(std::io::Error::new(std::io::ErrorKind::Other, x)),
                |_| Ok(()),
            )
    }

    fn transform_response(&self, raw_response: Response) -> std::io::Result<Self::Output> {
        raw_response.into_json()
    }

    fn new() -> Self {
        Self (PhantomData)
    }
}
