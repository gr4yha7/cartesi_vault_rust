use yew::prelude::*;
use cartesi_rust_frontend::landing::Landing;
// use alloy_chains::{Chain, ChainKind};
// use alloy_contract::ContractInstance;
// use alloy_provider::HttpProvider;
// use alloy_core::json_abi::JsonAbi;

// fn ethereum() {
//     let sepolia = Chain(ChainKind::Named(alloy_chains::NamedChain::Sepolia));
//     let contract = ContractInstance::new(address, provider, interface)
// }

#[function_component]
fn App() -> Html {
    html!{
        <div>
            <Landing />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
