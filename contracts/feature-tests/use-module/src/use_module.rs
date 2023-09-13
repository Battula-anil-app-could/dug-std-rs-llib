#![no_std]

mod contract_base_full_path_mod;
mod contract_base_mod;
mod internal_mod_a;
mod internal_mod_b;
mod internal_mod_c;
mod internal_mod_d;
mod internal_mod_init;
mod ongoing_operation_mod_example;
mod only_admin_derived_mod;
mod only_admin_mod;
mod only_owner_derived_mod;
mod only_owner_mod;
pub mod token_merge_mod_impl;

dharithri_sc::imports!();

/// Contract that tests that using modules works correctly.
/// Also provides testing for the most common modules:
/// - DnsModule
/// - FeaturesModule
/// - EsdtModule
/// - GovernanceModule
/// - PauseModule
#[dharithri_sc::contract]
pub trait UseModule:
    ContractBase
    + contract_base_full_path_mod::ContractBaseFullPathTestModule
    + contract_base_mod::ContractBaseTestModule
    + internal_mod_a::InternalModuleA
    + internal_mod_b::InternalModuleB
    + internal_mod_c::InternalModuleC
    + internal_mod_init::InternalModuleInit
    + only_owner_mod::OnlyOwnerTestModule
    + only_owner_derived_mod::OnlyOwnerDerivedTestModule
    + only_admin_mod::OnlyAdminTestModule
    + only_admin_derived_mod::OnlyAdminDerivedTestModule
    + ongoing_operation_mod_example::OngoingOperationModExample
    + token_merge_mod_impl::TokenMergeModImpl
    + dharithri_sc_modules::claim_developer_rewards::ClaimDeveloperRewardsModule
    + dharithri_sc_modules::dns::DnsModule
    + dharithri_sc_modules::esdt::EsdtModule
    + dharithri_sc_modules::features::FeaturesModule
    + dharithri_sc_modules::governance::GovernanceModule
    + dharithri_sc_modules::governance::governance_configurable::GovernanceConfigurablePropertiesModule
    + dharithri_sc_modules::governance::governance_events::GovernanceEventsModule
    + dharithri_sc_modules::pause::PauseModule
    + dharithri_sc_modules::staking::StakingModule
    + dharithri_sc_modules::token_merge::TokenMergeModule
    + dharithri_sc_modules::token_merge::merged_token_setup::MergedTokenSetupModule
    + dharithri_sc_modules::default_issue_callbacks::DefaultIssueCallbacksModule
    + dharithri_sc_modules::only_admin::OnlyAdminModule
    + dharithri_sc_modules::ongoing_operation::OngoingOperationModule
{
    /// Validates that the "featureName" feature is on.
    /// Uses the `feature_guard!` macro.
    #[endpoint(checkFeatureGuard)]
    fn check_feature_guard(&self) {
        self.check_feature_on(b"featureName", true);
    }

    #[endpoint(checkPause)]
    fn check_pause(&self) -> SCResult<bool> {
        Ok(self.is_paused())
    }
}
