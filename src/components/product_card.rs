use crate::route::Route;
use crate::types::Product;
use yew::prelude::*;
use yew_router::components::RouterAnchor;
use super::AtcButton;

pub struct ProductCard {
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub product: Product,
    pub on_add_to_cart: Callback<Product>,
}

impl Component for ProductCard {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        type Anchor = RouterAnchor<Route>;

        html! {
            <div class="product-card" aria-name="product card">
                <Anchor route=Route::ProductDetail(self.props.product.id) classes="product_card_anchor">
                    <div class="product-image">
                        <img class="image" src={&self.props.product.image} alt={&self.props.product.name} />
                    </div>
                    <div class="product-desc">
                        <h3>{&self.props.product.name}</h3>
                        <p>{"$"} {&self.props.product.price}</p>
                    </div>
                </Anchor>
                <AtcButton on_add_to_cart=self.props.on_add_to_cart.clone() product=self.props.product.clone() />
            </div>
        }
    }
}
