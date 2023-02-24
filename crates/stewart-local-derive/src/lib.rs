use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, Attribute, DeriveInput, Path};

#[proc_macro_derive(Factory, attributes(factory))]
pub fn derive_factory(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let DeriveInput { ident, attrs, .. } = input;

    let attr = find_attr(attrs);

    let output = quote! {
        impl stewart_local::Factory for #ident {
            fn start(
                self: Box<Self>,
                actor_id: usize,
                dispatcher: std::sync::Arc<dyn stewart_local::Dispatcher>,
                address: usize,
            ) -> Result<Box<dyn stewart_local::AnyActor>, anyhow::Error> {
                let ctx = stewart_local::Context::from_raw(actor_id, dispatcher);
                let address = stewart_local::Address::from_raw(address);
                let actor = #attr(ctx, address, *self)?;
                Ok(Box::new(actor))
            }
        }
    };
    output.into()
}

fn find_attr(attrs: Vec<Attribute>) -> Path {
    for attr in attrs {
        if !attr.path.is_ident("factory") {
            continue;
        }

        let path: Path = attr
            .parse_args()
            .expect("wrong format of \"factory\" attribute");
        return path;
    }

    panic!("unable to find \"factory\" attribute")
}
