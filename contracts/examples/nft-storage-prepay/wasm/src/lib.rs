// Code generated by the dharithri-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            8
// Async Callback (empty):               1
// Total number of exported functions:  10

#![no_std]
#![allow(internal_features)]
#![feature(lang_items)]

dharithri_sc_wasm_adapter::allocator!();
dharithri_sc_wasm_adapter::panic_handler!();

dharithri_sc_wasm_adapter::endpoints! {
    nft_storage_prepay
    (
        init => init
        setCostPerByte => set_cost_per_byte
        reserveFunds => reserve_funds
        claim => claim
        depositPaymentForStorage => deposit_payment_for_storage
        withdraw => withdraw
        getCostForSize => get_cost_for_size
        getDepositAmount => get_deposit_amount
        getCostPerByte => cost_per_byte
    )
}

dharithri_sc_wasm_adapter::async_callback_empty! {}
