use crate::types::Product;
use yew::prelude::*;

pub struct AtcButton {
    props: Props,
    link: ComponentLink<Self>,
}

pub enum Msg {
    AddToCart,
}

#[derive(Properties, Clone, Debug)]
pub struct Props {
    pub product: Product,
    pub on_add_to_cart: Callback<Product>,
}

impl Component for AtcButton {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender{
        match msg {
            Msg::AddToCart => {
                self.props.on_add_to_cart.emit(self.props.product.clone());
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let onclick = self.link.callback(|_| Msg::AddToCart);
        html! {
            <div class="product-action">
                <button onclick=onclick >{"Add to Cart"}</button>
            </div>
        }
    }
}
