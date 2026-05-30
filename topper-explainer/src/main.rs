#![recursion_limit = "100000"]
use models::ExplainerModel;
mod bindings;
mod colored_lines;
mod explainer;
mod links;
mod models;
mod msg;
mod sect_parser;

fn main() {
    yew::Renderer::<ExplainerModel>::new().render();
}
