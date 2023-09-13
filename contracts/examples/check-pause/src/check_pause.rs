#![no_std]

dharithri_sc::imports!();

use dharithri_sc_modules::pause;

#[dharithri_sc::contract]
pub trait CheckPauseContract: pause::PauseModule {
    #[init]
    fn init(&self) {}

    #[endpoint(checkPause)]
    fn check_pause(&self) -> SCResult<bool> {
        Ok(self.is_paused())
    }
}
