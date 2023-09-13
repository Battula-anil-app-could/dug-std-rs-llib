/// Used to generate the EndpointType in the ABI.
#[derive(Debug, Clone)]
pub enum EndpointTypeMetadata {
    Init,
    Endpoint,
    PromisesCallback,
}

impl EndpointTypeMetadata {
    pub fn to_tokens(&self) -> proc_macro2::TokenStream {
        match self {
            EndpointTypeMetadata::Init => {
                quote! { dharithri_sc::abi::EndpointTypeAbi::Init }
            },
            EndpointTypeMetadata::Endpoint => {
                quote! { dharithri_sc::abi::EndpointTypeAbi::Endpoint }
            },
            EndpointTypeMetadata::PromisesCallback => {
                quote! { dharithri_sc::abi::EndpointTypeAbi::PromisesCallback }
            },
        }
    }
}
