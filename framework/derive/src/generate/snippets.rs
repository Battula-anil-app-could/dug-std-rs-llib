pub fn contract_object_def() -> proc_macro2::TokenStream {
    quote! {
        pub struct ContractObj<A>
        where
            A: dharithri_sc::api::VMApi,
        {
            _phantom: core::marker::PhantomData<A>,
        }
    }
}

pub fn impl_contract_base() -> proc_macro2::TokenStream {
    quote! {
        impl<A> dharithri_sc::contract_base::ContractBase for ContractObj<A>
        where
            A: dharithri_sc::api::VMApi,
        {
            type Api = A;
        }
    }
}

pub fn new_contract_object_fn() -> proc_macro2::TokenStream {
    quote! {
        pub fn contract_obj<A>() -> ContractObj<A>
        where
            A: dharithri_sc::api::VMApi,
        {
            ContractObj {
                _phantom: core::marker::PhantomData,
            }
        }

        pub struct ContractBuilder;

        impl dharithri_sc::contract_base::CallableContractBuilder for self::ContractBuilder {
            fn new_contract_obj<A: dharithri_sc::api::VMApi + Send + Sync>(
                &self,
            ) -> dharithri_sc::types::heap::Box<dyn dharithri_sc::contract_base::CallableContract> {
                dharithri_sc::types::heap::Box::new(ContractObj::<A> {
                    _phantom: core::marker::PhantomData,
                })
            }
        }
    }
}

// TODO: explore auto-implementations of supertraits
#[allow(dead_code)]
pub fn impl_auto_impl() -> proc_macro2::TokenStream {
    quote! {
        impl<A> AutoImpl for ContractObj<A> where
            A: dharithri_sc::contract_base::ContractBase
                + dharithri_sc::api::ErrorApi
                + dharithri_sc::api::EndpointArgumentApi
                + dharithri_sc::api::EndpointFinishApi
                + dharithri_sc::api::ManagedTypeApi
        {
        }
    }
}

pub fn impl_callable_contract() -> proc_macro2::TokenStream {
    quote! {
        impl<A> dharithri_sc::contract_base::CallableContract for ContractObj<A>
        where
            A: dharithri_sc::api::VMApi + Send + Sync,
        {
            fn call(&self, fn_name: &str) -> bool {
                EndpointWrappers::call(self, fn_name)
            }
        }
    }
}

pub fn proxy_object_def() -> proc_macro2::TokenStream {
    quote! {
        pub struct Proxy<A>
        where
            A: dharithri_sc::api::VMApi + 'static,
        {
            pub address: dharithri_sc::types::ManagedOption<A, dharithri_sc::types::ManagedAddress<A>>,
        }

        impl<A> dharithri_sc::contract_base::ProxyObjBase for Proxy<A>
        where
            A: dharithri_sc::api::VMApi + 'static,
        {
            type Api = A;

            fn new_proxy_obj() -> Self {
                Proxy {
                    address: dharithri_sc::types::ManagedOption::none(),
                }
            }

            fn contract(mut self, address: dharithri_sc::types::ManagedAddress<Self::Api>) -> Self {
                self.address = dharithri_sc::types::ManagedOption::some(address);
                self
            }

            fn extract_opt_address(
                &mut self,
            ) -> dharithri_sc::types::ManagedOption<
                Self::Api,
                dharithri_sc::types::ManagedAddress<Self::Api>,
            > {
                core::mem::replace(&mut self.address, dharithri_sc::types::ManagedOption::none())
            }

            fn extract_address(&mut self) -> dharithri_sc::types::ManagedAddress<Self::Api> {
                self.extract_opt_address().unwrap_or_sc_panic(dharithri_sc::err_msg::RECIPIENT_ADDRESS_NOT_SET)
            }
        }
    }
}

pub fn callback_proxy_object_def() -> proc_macro2::TokenStream {
    quote! {
        pub struct CallbackProxyObj<A>
        where
            A: dharithri_sc::api::VMApi + 'static,
        {
            _phantom: core::marker::PhantomData<A>,
        }

        impl<A> dharithri_sc::contract_base::CallbackProxyObjBase for CallbackProxyObj<A>
        where
            A: dharithri_sc::api::VMApi + 'static,
        {
            type Api = A;

            fn new_cb_proxy_obj() -> Self {
                CallbackProxyObj {
                    _phantom: core::marker::PhantomData,
                }
            }
        }
    }
}

pub fn call_method_api_static_init() -> proc_macro2::TokenStream {
    quote! {
        <Self::Api as dharithri_sc::api::VMApi>::init_static();
    }
}
