use crate::admin::api::wrapped_get;
use crate::admin::structure::breadcrumb::BreadcrumbItem;
use crate::admin::structure::error_text::error_markup;
use crate::admin::structure::form;
use crate::admin::structure::page::Page;
use crate::admin::structure::{page, split};
use crate::admin::{item_page, reactivity};
use crate::registry::REGISTRY;
use crate::{admin, unwrap_result_else_markup};
use actix_web::web::ServiceConfig;
use actix_web::{guard, web};
use admin::structure::pagination_control::pagination_control;
use inventory::category::CategorySerial;
use inventory::inventory_location::InventoryLocationSerial;
use inventory::item::ItemCondition;
use inventory::pagination::{pagination_guard, KeysetPaginationOptionsForString, KeysetPaginationResultForString};
use inventory::product::ProductSerial;
use maud::{html, Markup};
use reqwest::Method;
use serde_json::json;

pub const PAGE: Page = Page {
    name: "Product",
    relative_path: "/admin/product",
    configurer,
};

const HEADINGS: [&str; 6] = ["id", "display_name ⏶", "internal_name", "upc", "release_date", "actions"];
const DELETE_FORM_CONTAINER_ID: &str = "delete_form_container";
const CREATE_ITEM_FORM_CONTAINER_ID: &str = "create_item_form_container";
const CATEGORY_DETAIL_CONTAINER_ID: &str = "category_detail_container";

pub fn configurer(config: &mut ServiceConfig) {
    config
        .route("/product",
               web::get()
                   .guard(guard::fn_guard(pagination_guard))
                   .to(handle_paginated))
        .route("/product", web::get().to(handle_unpaginated))
    ;
}

async fn handle_unpaginated() -> Markup {
    render(None).await
}

async fn handle_paginated(
    query: web::Query<KeysetPaginationOptionsForString>,
) -> Markup {
    render(Some(query.into_inner())).await
}

async fn render(pagination_options: Option<KeysetPaginationOptionsForString>) -> Markup {
    page::page(
        &vec!(BreadcrumbItem::from(PAGE)),
        html! {
            script src="/page/product.js" {}
        },
        split::split(left(pagination_options).await, right().await),
    )
}

async fn left(pagination_options: Option<KeysetPaginationOptionsForString>) -> Markup {
    let pagination_options = pagination_options.unwrap_or_default();
    let query_params = match serde_urlencoded::to_string(&pagination_options) {
        Ok(pagination_options) => pagination_options,
        Err(error) => return error_markup(error),
    };

    let (product_vec, pagination_result) = unwrap_result_else_markup!(
        wrapped_get::<(Vec<ProductSerial>, KeysetPaginationResultForString)>(
            format!("/product?{}", query_params).as_str(),
        ).await
    );

    html! {
        h2 { "Products" }
        (pagination_control(PAGE.relative_path, &pagination_options, &pagination_result))
        @if product_vec.is_empty() {
            p { "None" }
        } @else {
            (table(&product_vec).await)
        }
        (pagination_control(PAGE.relative_path, &pagination_options, &pagination_result))
    }
}

async fn right() -> Markup {
    html! {
        (create_form())
        (delete_form())
        (create_item_form().await)
        (category_details().await)
    }
}

async fn table(elements: &Vec<ProductSerial>) -> Markup {
    // Ascending sort: U+23F6 (⏶)
    // Descending sort: U+23F7 (⏷)
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
                        td { (element.display_name) }
                        td { (element.internal_name) }
                        td { (format!("{:?}", element.upc)) }
                        td { (format!("{:?}", element.release_date)) }
                        td {
                            a
                                href=(item_page::PAGE.relative_path.replace("{product_id}", element.id.to_string().as_str()))
                                { button { "View items" } }
                            button onclick=(activate_delete_form_script(DELETE_FORM_CONTAINER_ID, &element)) { "Delete" }
                            button onclick=(activate_create_item_form_script(CREATE_ITEM_FORM_CONTAINER_ID, &element)) { "Create item" }
                            button onclick=(activate_categories_script(CATEGORY_DETAIL_CONTAINER_ID, &element).await) { "Categories" }
                        }
                    }
                }
            }
        }
    }
}

fn create_form() -> Markup {
    form::form("Create product", "/product", Method::POST, html! {
        label {
            "Display name"
            input type="text" name="display_name";
        }
        label {
            "Internal name"
            input type="text" name="internal_name";
        }
        label {
            "Universal product code (optional)"
            input type="text" name="upc";
        }
        label {
            "Release date (optional)"
            input type="date" name="release_date";
        }
        input type="submit";
    })
}

fn delete_form() -> Markup {
    html! {
        div #(DELETE_FORM_CONTAINER_ID) style=(concat!("display: none;")) {
            hr {}
            (form::form("Delete product", "js", Method::DELETE, html! {
                label {
                    "ID"
                    input type="text" name="id" disabled[true];
                }
                input type="submit";
            }))
            button onclick=(reactivity::hide_element_handler(DELETE_FORM_CONTAINER_ID)) { "Cancel" }
        }
    }
}

async fn create_item_form() -> Markup {
    let inventory_location_vec: Vec<InventoryLocationSerial> = unwrap_result_else_markup!(
        wrapped_get::<Vec<InventoryLocationSerial>>("/inventory_location").await
    );

    html! {
        div #(CREATE_ITEM_FORM_CONTAINER_ID) style=(concat!("display: none;")) {
            hr {}
            (form::form("Create item", "js", Method::POST, html! {
                label {
                    "Product ID"
                    input type="text" name="product_id" readonly[true];
                }
                label {
                    "Inventory location"
                    select name="inventory_location_id" {
                        option value="" { "_required " }
                        @for inventory_location in inventory_location_vec {
                            option value=(inventory_location.id) { (inventory_location.display_name) }
                        }
                    }
                }
                label {
                    "Condition"
                    select style="display: block;" name="condition" {
                        option value="" { "_required"}
                        (form::enum_options::<ItemCondition>())
                    }
                }
                label {
                    "Price (\u{00A2})"
                    input type="number" name="price_cents";
                }
                label {
                    "Priority"
                    input type="number" name="priority" value="0";
                }
                label {
                    "Note (optional)"
                    textarea name="note" rows="3" wrap="soft" {};
                }
                label {
                    "Acquisition date & time (UTC)"
                    input type="datetime-local" name="acquisition_datetime" value=(form::get_current_datetime_string());
                }
                label {
                    "Acquisition price (\u{00A2}) (optional)"
                    input type="number" name="acquisition_price_cents";
                }
                label {
                    "Acquisition location (optional)"
                    input type="text" name="acquisition_location";
                }
                input type="hidden" name="status" value="1";
                input type="hidden" name="created" value=(form::get_current_datetime_string());
                input type="hidden" name="updated" value=(form::get_current_datetime_string());
                input type="submit";
            }))
            button onclick=(reactivity::hide_element_handler(CREATE_ITEM_FORM_CONTAINER_ID)) { "Cancel" }
        }
    }
}

async fn category_details() -> Markup {
    let all_categories: Vec<CategorySerial> = unwrap_result_else_markup!(
        wrapped_get::<Vec<CategorySerial>>("/category").await
    );

    html! {
        div #(CATEGORY_DETAIL_CONTAINER_ID) style=(concat!("display: none;")) {
            hr {}
            h2 { "Categories" }
            section {}
            hr {}
            (form::form("Add category", "js", Method::POST, html! {
                label {
                    // Placed here for UX, not actually consumed in the HTTP request
                    "Product ID"
                    input type="text" name="id" readonly[true];
                }
                label {
                    "Category"
                    select name="category_id" {
                        option value="" { "_required " }
                        @for category in all_categories {
                            option value=(category.id) { (category.display_name) }
                        }
                    }
                }
                input type="submit";
            }))
            button onclick=(reactivity::hide_element_handler(CATEGORY_DETAIL_CONTAINER_ID)) { "Close" }
        }
    }
}

// These scripts could be defined as global functions in a .js file instead
fn activate_delete_form_script(element_id: &str, product: &ProductSerial) -> String {
    let activate_form: String = reactivity::activate_element_handler(element_id);
    let update_form: String = reactivity::update_form_from_serialize("/item", product);
    activate_form + &update_form
}

fn activate_create_item_form_script(element_id: &str, product: &ProductSerial) -> String {
    let activate_form: String = reactivity::activate_element_handler(element_id);
    let modify_form: String = {
        let mut json_map = serde_json::Map::with_capacity(1);
        json_map.insert(String::from("product_id"), json!(product.id));
        reactivity::update_form_from_serialize("/item", &json_map)
    };
    activate_form + &modify_form
}

async fn activate_categories_script(element_id: &str, product: &ProductSerial) -> String {
    let product_categories: Vec<CategorySerial> = match wrapped_get::<Vec<CategorySerial>>(
        &format!("/product/{}/category", product.id)
    ).await {
        Ok(value) => value,
        Err(markup) => return Markup::into_string(markup)
    };

    let activate: String = reactivity::activate_element_handler(element_id);
    let inject: String = format!(
        r#"{{
        const categories = JSON.parse('{}');
        activate_categories(element, "{}", "{}", categories); // Defined in product.js
        }}"#,
        serde_json::to_string(&product_categories).unwrap(),
        REGISTRY.inventory_external_path,
        product.id,
    );
    let modify_form: String = reactivity::update_form_from_serialize(&format!("/product/{}/category", product.id), product);

    activate + &inject + &modify_form
}
