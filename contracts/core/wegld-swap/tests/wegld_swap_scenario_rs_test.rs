use dharithri_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/core/wegld-swap");

    blockchain.register_contract(
        "file:output/dharithri-wegld-swap-sc.wasm",
        dharithri_wegld_swap_sc::ContractBuilder,
    );
    blockchain
}

#[test]
fn unwrap_egld_rs() {
    world().run("scenarios/unwrap_egld.scen.json");
}

#[test]
fn wrap_egld_rs() {
    world().run("scenarios/wrap_egld.scen.json");
}
