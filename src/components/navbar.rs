use yew::prelude::*;
use crate::types::CartProduct;

pub struct Navbar {
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub cart_products: Vec<CartProduct>,
}

impl Component for Navbar {
    type Message = ();
    type Properties = Props;
    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            props,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let cart_value = self.props.cart_products
            .iter()
            .fold(0.0, |acc, cp| acc + (cp.quantity as f64 * cp.product.price));
        html! {
            <div class="navbar">
                <div class="cart">
                    {format!("Cart Value: {:.2}", cart_value)}
                </div>
            </div>
        }
    }
}

