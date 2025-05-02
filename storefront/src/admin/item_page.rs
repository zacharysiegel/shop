use crate::admin::api::wrapped_get;
use crate::admin::structure::error_text::error_text;
use crate::admin::structure::{page, split};
use crate::admin::{product_page, reactivity};
use crate::unwrap_result_else_markup;
use actix_web::web;
use actix_web::web::ServiceConfig;
use inventory::inventory_location::InventoryLocationSerial;
use inventory::item::{ItemCondition, ItemSerial, ItemStatus};
use inventory::product::ProductSerial;
use maud::{html, Markup};

pub const RELATIVE_PATH: &str = "/admin/product/{product_id}/item";
/// U+00A2 is the "cent" sign.
const HEADINGS: [&str; 6] = ["id", "location", "condition", "status", "price (\u{00A2})", "actions"];
const ITEM_DETAILS_CONTAINER_ID: &str = "item_details_container";
const ITEM_DETAIL_ID_PREFIX: &str = "item_detail_";

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
                            button onclick=(activate_item_details_script(element)) { "Details" }
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
                        @let field = "id";
                        td { (field) }
                        td #(String::from(ITEM_DETAIL_ID_PREFIX) + (field)) {  }
                    }
                    tr {
                        @let field = "product_id";
                        td { (field) }
                        td #(String::from(ITEM_DETAIL_ID_PREFIX) + field) {  }
                    }
                    tr {
                        @let field = "inventory_location_id";
                        td { (field) }
                        td #(String::from(ITEM_DETAIL_ID_PREFIX) + field) {  }
                    }
                    tr {
                        @let field = "condition";
                        td { (field) }
                        td #(String::from(ITEM_DETAIL_ID_PREFIX) + field) {  }
                    }
                    tr {
                        @let field = "status";
                        td { (field) }
                        td #(String::from(ITEM_DETAIL_ID_PREFIX) + field) {  }
                    }
                    tr {
                        @let field = "price_cents";
                        td { (field) }
                        td #(String::from(ITEM_DETAIL_ID_PREFIX) + field) {  }
                    }
                    tr {
                        @let field = "priority";
                        td { (field) }
                        td #(String::from(ITEM_DETAIL_ID_PREFIX) + field) {  }
                    }
                    tr {
                        @let field = "note";
                        td { (field) }
                        td #(String::from(ITEM_DETAIL_ID_PREFIX) + field) {  }
                    }
                    tr {
                        @let field = "acquisition_datetime";
                        td { (field) }
                        td #(String::from(ITEM_DETAIL_ID_PREFIX) + field) {  }
                    }
                    tr {
                        @let field = "acquisition_price_cents";
                        td { (field) }
                        td #(String::from(ITEM_DETAIL_ID_PREFIX) + field) {  }
                    }
                    tr {
                        @let field = "acquisition_location";
                        td { (field) }
                        td #(String::from(ITEM_DETAIL_ID_PREFIX) + field) {  }
                    }
                    tr {
                        @let field = "created";
                        td { (field) }
                        td #(String::from(ITEM_DETAIL_ID_PREFIX) + field) {  }
                    }
                    tr {
                        @let field = "updated";
                        td { (field) }
                        td #(String::from(ITEM_DETAIL_ID_PREFIX) + field) {  }
                    }
                }
            }
            button style="margin-top: .5rem;"
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

fn activate_item_details_script(item: &ItemSerial) -> String {
    let json: String = match serde_json::to_string(item) {
        Ok(value) => value,
        Err(error) => return format!(r#"console.err("Error serializing value", {:?}, {:#});"#, item, error).to_string(),
    };

    let value_setter: String = format!(r#"
        const item = JSON.parse('{}');
        for (let [key, value] of Object.entries(item)) {{
            const element = document.getElementById("{}" + key);
            element.innerText = value;
        }}
    "#, json, ITEM_DETAIL_ID_PREFIX);

    {
        let mut script: String = reactivity::activate_element_handler(ITEM_DETAILS_CONTAINER_ID);
        script.push_str(&value_setter);
        script
    }
}
