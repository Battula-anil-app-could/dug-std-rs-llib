// Code generated by the dharithri-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            6
// Async Callback (empty):               1
// Total number of exported functions:   8

#![no_std]
#![allow(internal_features)]
#![feature(lang_items)]

dharithri_sc_wasm_adapter::allocator!();
dharithri_sc_wasm_adapter::panic_handler!();

dharithri_sc_wasm_adapter::endpoints! {
    single_value_repeat
    (
        init => init
        add => add
        count => count
        remove => remove
        add_struct => add_struct
        count_struct => count_struct
        remove_struct => remove_struct
    )
}

dharithri_sc_wasm_adapter::async_callback_empty! {}
