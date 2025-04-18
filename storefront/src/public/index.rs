use maud::{Markup, html};

pub async fn render() -> Markup {
	super::page::page(html! {
		section class="hero" {
			div class="hero-content" {
				h1 { "Rare Star Wars Collectibles" }
				p { "Discover unique items from a galaxy far, far away" }
				a href="/category/all" class="button" { "Browse Collection" }
			}
		}

		section class="featured-categories" {
			h2 { "Browse Categories" }
			div class="category-grid" {
				"__grid__"
			}
		}

		section class="featured-products" {
			h2 { "Featured Items" }
			div class="product-grid" {
				"__featured-products__"
			}
		}
	})
	.await
}
