use crate::admin::api::wrapped_get;
use crate::admin::structure::breadcrumb::BreadcrumbItem;
use crate::admin::structure::error_text::error_markup;
use crate::admin::structure::page::Page;
use crate::admin::structure::{form, page, split};
use crate::admin::{listing_page, product_page, reactivity};
use crate::unwrap_result_else_markup;
use actix_web::web;
use actix_web::web::ServiceConfig;
use inventory::inventory_location::InventoryLocationSerial;
use inventory::item::{ItemCondition, ItemSerial, ItemStatus};
use inventory::marketplace::MarketplaceSerial;
use inventory::product::ProductSerial;
use maud::{html, Markup};
use reqwest::Method;
use serde_json::{json, Map, Value};

pub const PAGE: Page = Page {
    name: "Product",
    relative_path: "/admin/product/{product_id}/item",
    configurer,
};
pub const ITEM_FIELDS: [&str; 13] = [
    "id",
    "product_id",
    "inventory_location_id",
    "condition",
    "status",
    "price_cents",
    "priority",
    "note",
    "acquisition_datetime",
    "acquisition_price_cents",
    "acquisition_location",
    "created",
    "updated",
];

/// U+00A2 is the "cent" sign.
const HEADINGS: [&str; 6] = ["id", "location", "condition", "status", "price (\u{00A2})", "actions"];
const ITEM_DETAILS_CONTAINER_ID: &str = "item_details_container";
const ITEM_DETAIL_ID_PREFIX: &str = "item_detail_";
const CREATE_LISTING_FORM_CONTAINER_ID: &str = "create_listing_form_container";

fn configurer(config: &mut ServiceConfig) {
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
            BreadcrumbItem::from(product_page::PAGE),
            BreadcrumbItem::new("Item", &PAGE.relative_path.replace("{product_id}", &product_id)),
        ),
        Markup::default(),
        split::split(
            left(&product_id).await,
            right().await,
        ),
    )
}

async fn left(product_id: &str) -> Markup {
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

async fn right() -> Markup {
    html! {
        (item_details())
        (create_listing_form().await)
    }
}

async fn table(elements: &Vec<ItemSerial>) -> Markup {
    let inventory_location_vec: Vec<InventoryLocationSerial> = unwrap_result_else_markup!(
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
                            Ok(variant) => format!("{}", variant),
                            Err(error) => Markup::into_string(error_markup(error)),
                        }) }
                        td { (match ItemStatus::try_from_repr(element.status) {
                            Ok(variant) => format!("{}", variant),
                            Err(error) => Markup::into_string(error_markup(error)),
                        }) }
                        td { (element.price_cents) }
                        td {
                            a
                                href=(listing_page::PAGE.relative_path
                                    .replace("{product_id}", &element.product_id.to_string())
                                    .replace("{item_id}", &element.id.to_string())
                                )
                                { button { "View listings" } }
                            button onclick=(activate_item_details_script(element)) { "Details" }
                            button onclick=(activate_create_listing_script(element)) { "Create listing" }
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
                    @for field in &ITEM_FIELDS {
                        tr {
                            td { (field) }
                            td #(String::from(ITEM_DETAIL_ID_PREFIX) + field) { }
                        }
                    }
                }
            }
            button style="margin-top: .5rem;"
                onclick=(reactivity::hide_element_handler(ITEM_DETAILS_CONTAINER_ID)) { "Close" }
        }
    }
}

async fn create_listing_form() -> Markup {
    let marketplace_vec: Vec<MarketplaceSerial> = unwrap_result_else_markup!(
        wrapped_get::<Vec<MarketplaceSerial>>("/marketplace").await
    );

    html! {
        div #(CREATE_LISTING_FORM_CONTAINER_ID) style=(concat!("display: none;")) {
            hr {}
            (form::form(Some("Create listing"), "/listing", Method::POST, html! {
                label {
                    "Item ID"
                    input type="text" readonly[true] name="item_id";
                }
                label {
                    "Marketplace"
                    select name="marketplace_id" {
                        @for marketplace in marketplace_vec {
                            option value=(marketplace.id) { (marketplace.display_name) }
                        }
                    }
                }
                label {
                    "URI (optional)"
                    input type="text" name="uri";
                }

                // Need to use number/datetime-local instead of hidden type for form_data mutators; see submit_form.js;
                input style="display: none;" type="datetime-local" name="created" value=(form::get_current_datetime_string());
                input style="display: none;" type="datetime-local" name="updated" value=(form::get_current_datetime_string());
                input type="submit";
            }))
            button onclick=(reactivity::hide_element_handler(CREATE_LISTING_FORM_CONTAINER_ID)) { "Close" }
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

fn activate_item_details_script(item: &ItemSerial) -> String {
    let mut script: String = reactivity::activate_element_handler(ITEM_DETAILS_CONTAINER_ID);
    script.push_str(&reactivity::set_content_by_prefix_from_serialize(ITEM_DETAIL_ID_PREFIX, item));
    script
}

fn activate_create_listing_script(item: &ItemSerial) -> String {
    let mut json_map: Map<String, Value> = Map::with_capacity(1);
    json_map.insert(String::from("item_id"), json!(item.id));

    let mut script = reactivity::activate_element_handler(CREATE_LISTING_FORM_CONTAINER_ID);
    script.push_str(&reactivity::update_form_from_serialize("/listing", &json_map));
    script
}
