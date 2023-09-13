dharithri_sc::imports!();

#[dharithri_sc::module]
pub trait DummyModule {
    fn some_function(&self) -> BigUint {
        BigUint::zero()
    }
}
