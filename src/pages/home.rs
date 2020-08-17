use crate::types::{Product, CartProduct};
use crate::components::ProductCard;
use crate::api;
use yew::prelude::*;
use yew::services::fetch::FetchTask;
use yew::format::Json;
use anyhow::Error;

pub struct Home {
    link: ComponentLink<Self>,
    props: Props,
    state: State,
    task: Option<FetchTask>
}

pub struct State {
    products: Vec<Product>,
    get_products_error: Option<Error>,
    get_products_loaded: bool,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub cart_products: Vec<CartProduct>,
    pub on_add_to_cart: Callback<Product>,
}

pub enum Msg {
    RequestProduct,
    RequestProductSuccess(Vec<Product>),
    RequestProductFailure(Error),
    EmitAddToCart(Product),
}

impl Component for Home {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::RequestProduct);
        Self {
            task: None,
            state: State {
                products: vec![],
                get_products_error: None,
                get_products_loaded: false,
            },
            link ,
            props
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Msg::EmitAddToCart(product) => {
                self.props.on_add_to_cart.emit(product);
                true 
            }
            Msg::RequestProduct => {
                self.state.get_products_loaded = false;
                let handler =
                    self.link
                    .callback(move|response: api::FetchResponse<Vec<Product>>| {
                        let (_ , Json(data)) = response.into_parts();
                        match data {
                            Ok(product) => Msg::RequestProductSuccess(product),
                            Err(err) => Msg::RequestProductFailure(err),
                        }
                    });
                self.task = Some(api::get_products(handler));
                true
            }
            Msg::RequestProductSuccess(products) => {
                self.state.products = products;
                self.state.get_products_loaded = true;
                true
            }
            Msg::RequestProductFailure(error) => {
                self.state.get_products_error = Some(error);
                self.state.get_products_loaded = true;
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let products: Vec<Html> = self
            .state
            .products
            .iter()
            .map(|product: &Product| {
                let handler = self
                    .link
                    .callback(|product: Product| {
                        Msg::EmitAddToCart(product)
                    });
                html! {
                    <ProductCard product={product} on_add_to_cart=handler />
                }
            })
            .collect();

        if !self.state.get_products_loaded {
            html! { <div>{"Loading..."}</div> }
        } else {
            html! {
                <div>
                    <span class="products">{products}</span>
                </div>
            }
        }
    }
}
