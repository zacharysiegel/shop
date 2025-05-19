use crate::admin::api::wrapped_get;
use crate::admin::structure::breadcrumb::BreadcrumbItem;
use crate::admin::structure::error_text::error_markup;
use crate::admin::structure::page::Page;
use crate::admin::structure::{form, page, split};
use crate::admin::{item_page, product_page, reactivity};
use crate::registry::REGISTRY;
use crate::unwrap_result_else_markup;
use actix_web::web;
use actix_web::web::ServiceConfig;
use inventory::item::{ItemCondition, ItemSerial, ItemStatus};
use inventory::listing::{ListingSerial, ListingStatus};
use inventory::marketplace::MarketplaceSerial;
use inventory::product::ProductSerial;
use maud::{html, Markup};

pub const PAGE: Page = Page {
    name: "Listing",
    relative_path: "/admin/product/{product_id}/item/{item_id}/listing",
    configurer,
};
pub const LISTING_FIELDS: [&str; 7] = [
    "id",
    "item_id",
    "marketplace_id",
    "uri",
    "status",
    "created",
    "updated",
];

const HEADINGS: [&str; 5] = ["id", "marketplace", "uri", "status", "actions"];
const LISTING_DETAILS_CONTAINER_ID: &str = "listing_details_container";
const LISTING_DETAIL_ID_PREFIX: &str = "listing_detail_";
const LISTING_UPDATE_CONTAINER_ID: &str = "listing_update_container";

fn configurer(config: &mut ServiceConfig) {
    config
        .route("/product/{product_id}/item/{item_id}/listing", web::get().to(render))
    ;
}

async fn render(
    path: web::Path<(String, String)>,
) -> Markup {
    let (product_id, item_id): (String, String) = path.into_inner();
    let this_path: String = PAGE.relative_path.to_string()
        .replace("{product_id}", &product_id)
        .replace("{item_id}", &item_id);

    page::page(
        &vec!(
            BreadcrumbItem::from(product_page::PAGE),
            BreadcrumbItem::new("Item", &item_page::PAGE.relative_path.replace("{product_id}", &product_id)),
            BreadcrumbItem::new(PAGE.name, &this_path),
        ),
        Markup::default(),
        split::split(
            left(&product_id, &item_id).await,
            right(&item_id).await,
        ),
    )
}

async fn left(product_id: &str, item_id: &str) -> Markup {
    let product: ProductSerial = unwrap_result_else_markup!(
        wrapped_get::<ProductSerial>(&format!("/product/{}", product_id)).await
    );
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

async fn right(item_id: &str) -> Markup {
    let item: ItemSerial = unwrap_result_else_markup!(
        wrapped_get::<ItemSerial>(&format!("/item/{}", item_id)).await
    );

    html! {
        (item_details(&item))
        (listing_details())
        (listing_update())
    }
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
                @for listing in elements {
                    tr {
                        td { (listing.id) }
                        td { (marketplace_markup(&marketplace_vec, listing)) }
                        td { (format!("{:?}", listing.uri)) }
                        td { (match ListingStatus::try_from_repr(listing.status) {
                            Ok(variant) => format!("{}", variant),
                            Err(error) => Markup::into_string(error_markup(error)),
                        }) }
                        td {
                            button onclick=(activate_listing_details_script(&listing)) { "Details" }
                            button onclick=(activate_listing_update_form_script(&listing)) { "Update" }
                            @if listing.status == ListingStatus::Draft as u8 {
                                x-publish-listing
                                action=(format!("{}/listing/{}/publish", REGISTRY.inventory_external_path, listing.id))
                                {}
                            }
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

fn item_details(item: &ItemSerial) -> Markup {
    html! {
        div {
            h2 { "Item details" }
            table {
                tbody {
                    tr {
                        td { "id" }
                        td { (item.id) }
                    }
                    tr {
                        td { "product_id" }
                        td { (item.product_id) }
                    }
                    tr {
                        td { "inventory_location_id" }
                        td { (item.inventory_location_id) }
                    }
                    tr {
                        td { "condition" }
                        td { (match ItemCondition::try_from_repr(item.condition) {
                            Ok(variant) => format!("{}", variant),
                            Err(error) => Markup::into_string(error_markup(error)),
                        }) }
                    }
                    tr {
                        td { "status" }
                        td { (match ItemStatus::try_from_repr(item.status) {
                            Ok(variant) => format!("{}", variant),
                            Err(error) => Markup::into_string(error_markup(error)),
                        }) }
                    }
                    tr {
                        td { "price_cents" }
                        td { (item.price_cents) }
                    }
                    tr {
                        td { "priority" }
                        td { (item.priority) }
                    }
                    tr {
                        td { "note" }
                        td { (format!("{:?}", item.note)) }
                    }
                    tr {
                        td { "acquisition_datetime" }
                        td { (item.acquisition_datetime) }
                    }
                    tr {
                        td { "acquisition_price_cents" }
                        td { (format!("{:?}", item.acquisition_price_cents)) }
                    }
                    tr {
                        td { "acquisition_location" }
                        td { (format!("{:?}", item.acquisition_location)) }
                    }
                    tr {
                        td { "created" }
                        td { (item.created) }
                    }
                    tr {
                        td { "updated" }
                        td { (item.updated) }
                    }
                }
            }
        }
    }
}

fn listing_details() -> Markup {
    html! {
        div #(LISTING_DETAILS_CONTAINER_ID) style="display: none;" {
            hr {}
            h2 { "Listing details" }
            table {
                tbody {
                    @for field in &LISTING_FIELDS {
                        tr {
                            td { (field) }
                            td #(String::from(LISTING_DETAIL_ID_PREFIX) + field) { }
                        }
                    }
                }
            }
            button onclick=(reactivity::hide_element_handler(LISTING_DETAILS_CONTAINER_ID)) { "Close" }
        }
    }
}

fn listing_update() -> Markup {
    html! {
        div #(LISTING_UPDATE_CONTAINER_ID) style="display: none;" {
            hr {}
            (form::form("Listing modifications", "", reqwest::Method::PUT, html! {
                label {
                    label {
                        "ID"
                        input type="text" name="id" readonly[true];
                    }
                    label {
                        "URI"
                        input type="text" name="uri";
                    }

                    // Need to use number/datetime-local instead of hidden type for form_data mutators; see submit_form.js;
                    input style="display: none;" type="datetime-local" name="updated" value=(form::get_current_datetime_string());
                    input style="display: none;" type="text" name="item_id";
                    input style="display: none;" type="text" name="marketplace_id";
                    input style="display: none;" type="datetime-local" name="created";
                    input type="submit";
                }
            }))
            button onclick=(reactivity::hide_element_handler(LISTING_UPDATE_CONTAINER_ID)) { "Close" }
        }
    }
}

fn activate_listing_details_script(listing: &ListingSerial) -> String {
    let activate: String = reactivity::activate_element_handler(LISTING_DETAILS_CONTAINER_ID);
    let modify: String = reactivity::set_content_by_prefix_from_serialize(LISTING_DETAIL_ID_PREFIX, listing);
    activate + &modify
}

fn activate_listing_update_form_script(listing: &ListingSerial) -> String {
    let activate: String = reactivity::activate_element_handler(LISTING_UPDATE_CONTAINER_ID);
    let modify: String = reactivity::update_form_from_serialize(&format!("/listing/{}", listing.id), listing);
    activate + &modify
}
