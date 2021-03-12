use yew::agent::Threaded;
use crate::agents::coffee_summary_store::CoffeeSummaryStore;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    CoffeeSummaryStore::register();
}