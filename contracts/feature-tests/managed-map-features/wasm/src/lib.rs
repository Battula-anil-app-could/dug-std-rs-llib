// Code generated by the dharithri-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            3
// Async Callback (empty):               1
// Total number of exported functions:   5

#![no_std]
#![allow(internal_features)]
#![feature(lang_items)]

dharithri_sc_wasm_adapter::allocator!();
dharithri_sc_wasm_adapter::panic_handler!();

dharithri_sc_wasm_adapter::endpoints! {
    managed_map_features
    (
        init => init
        mm_get => mm_get
        mm_contains => mm_contains
        mm_remove_get => mm_remove_get
    )
}

dharithri_sc_wasm_adapter::async_callback_empty! {}
