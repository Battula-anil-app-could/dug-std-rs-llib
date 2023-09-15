// Code generated by the dharitri-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           10
// Async Callback (empty):               1
// Promise callbacks:                    1
// Total number of exported functions:  13

#![no_std]
#![allow(internal_features)]
#![feature(lang_items)]

dharitri_sc_wasm_adapter::allocator!();
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    forwarder_queue
    (
        init => init
        queued_calls => queued_calls
        add_queued_call_sync => add_queued_call_sync
        add_queued_call_legacy_async => add_queued_call_legacy_async
        add_queued_call_transfer_execute => add_queued_call_transfer_execute
        add_queued_call_transfer_dct => add_queued_call_transfer_dct
        add_queued_call_promise => add_queued_call_promise
        add_queued_call => add_queued_call
        forward_queued_calls => forward_queued_calls
        callback_count => callback_count
        callback_payments => callback_payments
        promises_callback_method => promises_callback_method
    )
}

dharitri_sc_wasm_adapter::async_callback_empty! {}
