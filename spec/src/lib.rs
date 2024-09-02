use darling::ast::NestedMeta;
use darling::{Error, FromMeta};
use proc_macro::TokenStream;

#[derive(Debug, FromMeta)]
struct SpecArguments {
    number: String,
    text: String,
}

#[proc_macro_attribute]
pub fn spec(args: TokenStream, item: TokenStream) -> TokenStream {
    let attr_args = match NestedMeta::parse_meta_list(args.into()) {
        Ok(v) => v,
        Err(e) => return TokenStream::from(Error::from(e).write_errors()),
    };
    let spec_args = match SpecArguments::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => return TokenStream::from(e.write_errors()),
    };

    if spec_args.number.is_empty() {
        return TokenStream::from(
            Error::custom("You need to pass the spec number to the spec macro").write_errors(),
        );
    }

    if spec_args.text.is_empty() {
        return TokenStream::from(
            Error::custom("You need to pass the spec text to the spec macro").write_errors(),
        );
    }

    item
}
