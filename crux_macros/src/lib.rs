
mod effect;

use effect::effect_impl;
use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro_derive(Effect, attributes(effect))]
pub fn effect(input: TokenStream) -> TokenStream {
    effect_impl(&parse_macro_input!(input)).into()
}