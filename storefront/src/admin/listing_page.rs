use crate::admin::api::wrapped_get;
use crate::admin::structure::{page, split};
use crate::admin::{item_page, product_page};
use crate::unwrap_result_else_markup;
use actix_web::web;
use actix_web::web::ServiceConfig;
use inventory::listing::ListingSerial;
use inventory::marketplace::MarketplaceSerial;
use inventory::product::ProductSerial;
use maud::{html, Markup};

pub const RELATIVE_PATH: &str = "/admin/product/{product_id}/item/{item_id}/listing";

const HEADINGS: [&str; 5] = ["id", "marketplace", "uri", "status", "actions"];

pub fn configurer(config: &mut ServiceConfig) {
    config
        .route("/product/{product_id}/item/{item_id}/listing", web::get().to(render))
    ;
}

async fn render(
    path: web::Path<(String, String)>,
) -> Markup {
    let (product_id, item_id): (String, String) = path.into_inner();
    let this_path: String = RELATIVE_PATH.to_string()
        .replace("{product_id}", &product_id)
        .replace("{item_id}", &item_id);

    page::page(
        &vec!(
            (product_page::RELATIVE_PATH, "Product"),
            (&item_page::RELATIVE_PATH.replace("{product_id}", &product_id), "Item"),
            (&this_path, "Listing"),
        ),
        split::split(
            left(&product_id, &item_id).await,
            right(),
        ),
    )
}

async fn left(product_id: &str, item_id: &str) -> Markup {
    let product: ProductSerial = unwrap_result_else_markup!(
        wrapped_get::<ProductSerial>(&format!("/product/{}", product_id)).await
    );
    // let item: ItemSerial = unwrap_result_else_markup!(
    //     wrapped_get::<ItemSerial>(&format!("/item/{}", item_id)).await
    // );
    let listing_vec: Vec<ListingSerial> = unwrap_result_else_markup!(
        wrapped_get::<Vec<ListingSerial>>(&format!("/item/{}/listing", item_id)).await
    );

    html! {
        h2 { (format!("Listings for product \"{}\", item \"{}\"", product.display_name, item_id)) }
        @if listing_vec.is_empty() {
            p { "None" }
        } @else {
            (table(&listing_vec).await)
        }
    }
}

fn right() -> Markup {
    // todo: item details?
    html! { }
}

async fn table(elements: &Vec<ListingSerial>) -> Markup {
    let marketplace_vec: Vec<MarketplaceSerial> = unwrap_result_else_markup!(
        wrapped_get::<Vec<MarketplaceSerial>>("/marketplace").await
    );

    html! {
        table {
            thead {
                tr {
                    @for heading in HEADINGS {
                        th { (heading) }
                    }
                }
            }
            tbody {
                @for element in elements {
                    tr {
                        td { (element.id) }
                        td { (marketplace_markup(&marketplace_vec, element)) }
                        td { (format!("{:?}", element.uri)) }
                        td { (format!("{:?}", element.status)) }
                        td {
                            button disabled[true] { "Details" } // todo
                            button disabled[true] { "Update" } // todo
                        }
                    }
                }
            }
        }
    }
}

fn marketplace_markup(marketplace_vec: &Vec<MarketplaceSerial>, listing: &ListingSerial) -> Markup {
    let marketplace: &MarketplaceSerial = match marketplace_vec
        .iter()
        .find(|element| element.id == listing.marketplace_id)
    {
        Some(value) => value,
        None => return html! {},
    };

    html! { (marketplace.display_name) }
}
