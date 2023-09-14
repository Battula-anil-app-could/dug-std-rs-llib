dharithri_sc::imports!();

#[dharithri_sc::module]
pub trait ContractBaseTestModule: ContractBase {
    #[endpoint]
    fn call_contract_base_endpoint(&self) {}
}
