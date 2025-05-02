use crate::admin::api::wrapped_get;
use crate::admin::{product_page, reactivity};
use crate::admin::structure::error_text::error_text;
use crate::admin::structure::{form, page, split};
use crate::unwrap_result_else_markup;
use actix_web::web;
use actix_web::web::ServiceConfig;
use inventory::inventory_location::InventoryLocationSerial;
use inventory::item::{ItemCondition, ItemSerial, ItemStatus};
use inventory::product::ProductSerial;
use maud::{html, Markup};
use reqwest::Method;

pub const RELATIVE_PATH: &str = "/admin/product/{product_id}/item";
/// U+00A2 is the "cent" sign.
const HEADINGS: [&str; 6] = ["id", "location", "condition", "status", "price (\u{00A2})", "actions"];
const ITEM_DETAILS_CONTAINER_ID: &str = "item_details_container";

pub fn configurer(config: &mut ServiceConfig) {
    config
        .route("/product/{product_id}/item", web::get().to(render))
    ;
}

async fn render(
    product_id: web::Path<String>,
) -> Markup {
    let product_id = product_id.into_inner();
    page::page(
        &vec!(
            (product_page::RELATIVE_PATH, "Product"),
            (&RELATIVE_PATH.replace("{product_id}", &product_id), "Item"),
        ),
        split::split(left(&product_id).await, right()),
    )
}

async fn left(product_id: &String) -> Markup {
    let product: ProductSerial = unwrap_result_else_markup!(
        wrapped_get::<ProductSerial>(&format!("/product/{}", product_id)).await
    );
    let item_vec: Vec<ItemSerial> = unwrap_result_else_markup!(
        wrapped_get::<Vec<ItemSerial>>(&format!("/product/{}/item", product_id)).await
    );

    html!(
        h2 { (format!("Items for product \"{}\"", product.display_name)) }
        @if item_vec.is_empty() {
            p { "None" }
        } @else {
            (table(&item_vec).await)
        }
    )
}

fn right() -> Markup {
    (item_details())
}

async fn table(elements: &Vec<ItemSerial>) -> Markup {
    let inventory_location_vec = unwrap_result_else_markup!(
        wrapped_get::<Vec<InventoryLocationSerial>>("/inventory_location").await
    );

    html! {
        table {
            thead {
                @for heading in HEADINGS {
                    th { (heading) }
                }
            }
            tbody {
                @for element in elements {
                    tr {
                        td { (element.id) }
                        td { (inventory_location_markup(&inventory_location_vec, &element)) }
                        td { (match ItemCondition::try_from_repr(element.condition) {
                            Ok(variant) => format!("{:?}", variant),
                            Err(error) => Markup::into_string(error_text(error)),
                        }) }
                        td { (match ItemStatus::try_from_repr(element.status) {
                            Ok(variant) => format!("{:?}", variant),
                            Err(error) => Markup::into_string(error_text(error)),
                        }) }
                        td { (element.price_cents) }
                        td {
                            button onclick=(reactivity::activate_element_handler(ITEM_DETAILS_CONTAINER_ID)) { "Details" }
                        }
                    }
                }
            }
        }
    }
}

fn item_details() -> Markup {
    html! {
        div #(ITEM_DETAILS_CONTAINER_ID) style="display: none;" {
            h2 { "Item details" }
            table {
                tbody {
                    tr {
                        td { "id" }
                        td {  }
                    }
                    tr {
                        td { "product_id" }
                        td {  }
                    }
                    tr {
                        td { "inventory_location_id" }
                        td {  }
                    }
                    tr {
                        td { "condition" }
                        td {  }
                    }
                    tr {
                        td { "status" }
                        td {  }
                    }
                    tr {
                        td { "price_cents" }
                        td {  }
                    }
                    tr {
                        td { "priority" }
                        td {  }
                    }
                    tr {
                        td { "note" }
                        td {  }
                    }
                    tr {
                        td { "acquisition_datetime" }
                        td {  }
                    }
                    tr {
                        td { "acquisition_price_cents" }
                        td {  }
                    }
                    tr {
                        td { "acquisition_location" }
                        td {  }
                    }
                    tr {
                        td { "created" }
                        td {  }
                    }
                    tr {
                        td { "updated" }
                        td {  }
                    }
                }
            }
            button style="padding-top: .5rem;"
                onclick=(reactivity::hide_element_handler(ITEM_DETAILS_CONTAINER_ID)) { "Close" }
        }
    }
}

fn inventory_location_markup(inventory_location_vec: &Vec<InventoryLocationSerial>, item: &ItemSerial) -> Markup {
    let inventory_location: &InventoryLocationSerial = match inventory_location_vec
        .iter()
        .find(|location| location.id.eq(&item.inventory_location_id))
    {
        Some(value) => value,
        None => return html! {""},
    };

    html! { (inventory_location.display_name) }
}
