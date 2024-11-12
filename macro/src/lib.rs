use std::{env, fs, io::Write, path::PathBuf};

use proc_macro::TokenStream;
use proc_macro_crate::FoundCrate;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, LitInt, LitStr, Token,
};

struct MacroInput {
    path: LitStr,
    _comma: Token![,],
    compression_level: LitInt,
}

impl Parse for MacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let path = input.parse()?;
        let _comma = input.parse()?;
        let compression_level = input.parse()?;

        Ok(Self {
            path,
            _comma,
            compression_level,
        })
    }
}

#[proc_macro]
#[doc(hidden)]
pub fn include_zstd_inner(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as MacroInput);

    let path = PathBuf::from(input.path.value());
    let compression_level = input.compression_level.base10_parse().unwrap();

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let path = if path.is_relative() {
        manifest_dir.join(path)
    } else {
        path
    };

    let bytes = fs::read(path).unwrap();
    let mut compressed_bytes = Vec::new();
    let mut encoder = zstd::Encoder::new(&mut compressed_bytes, compression_level).unwrap();
    encoder.write_all(&bytes).unwrap();
    encoder.finish().unwrap();

    let compressed_bytes_len = compressed_bytes.len();

    let crate_name = proc_macro_crate::crate_name("include-zstd").unwrap();
    let crate_name = match crate_name {
        FoundCrate::Itself => "include_zstd".to_string(),
        FoundCrate::Name(name) => name,
    };

    format!(
        r#"unsafe {{ {crate_name}::EmbeddedZstd::<{compressed_bytes_len}>::new_unchecked({}) }}"#,
        String::from("[")
            + &compressed_bytes
                .into_iter()
                .map(|b| b.to_string())
                .collect::<Vec<_>>()
                .join(",")
            + "]",
    )
    .parse()
    .unwrap()
}
