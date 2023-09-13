dharithri_sc::imports!();

#[dharithri_sc::module]
pub trait ContractBaseFullPathTestModule: dharithri_sc::contract_base::ContractBase {
    #[endpoint]
    fn call_contract_base_full_path_endpoint(&self) {}
}
