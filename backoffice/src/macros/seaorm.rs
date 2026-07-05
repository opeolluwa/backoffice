/// # Description
/// Adds a ts_export rename attribute in the format of: "{literal_name}Entity"
/// to the given entity. This macro assumes the `[sea_orm(table_name = "...")]` or
/// `[sea_orm(entity = "...")]`  attribute is present and will error if not found.
/// # Attribute Arguments
/// Optional string literal, if given the literal provided will be inserted after the
/// identified literal name instead of the default "Entity".
#[proc_macro_attribute]
pub fn ts_rs_export_sea_orm_entity_name(args: TokenStream, input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    match ts_rs_export_sea_orm_entity_name::expand(derive_input, args) {
        Ok(token_stream) => token_stream.into(),
        Err(e) => e.to_compile_error().into(),
    }
}


pub(crate) fn expand(
    mut input: DeriveInput,
    args: TokenStream,
) -> syn::Result<proc_macro2::TokenStream> {
    let mut literal_name = None;

    // Append attribute argument or default value of "Entity"
    let entity_name_append = syn::parse::<Option<LitStr>>(args)?
        .map(|x| x.value())
        .unwrap_or(String::from("Entity"));

    input
        .attrs
        .iter()
        .filter(|attr| attr.path().is_ident("sea_orm"))
        .try_for_each(|attr| {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("table_name") || meta.path.is_ident("entity") {
                    literal_name = Some(meta.value()?.parse::<LitStr>()?);
                }
                Ok(())
            })
        })?;

    let literal_name = literal_name.ok_or(syn::Error::new(
        input.ident.span(),
        "Must provide a #[sea_orm(table_name = \"...\")] or #[sea_orm(entity = \"...\")] attribute",
    ))?;

    // Get the string value stored in the literal
    let ts_name = LitStr::new(
        &(convert_snake_to_pascal(&literal_name.value()) + &entity_name_append),
        literal_name.span(),
    );

    let mut attrs: Vec<Attribute> = vec![
        parse_quote_spanned!(input.ident.span() => #[derive(::ts_rs::TS)]),
        parse_quote_spanned!(literal_name.span() => #[ts(export, rename = #ts_name)]),
    ];

    let input_attrs: &mut Vec<Attribute> = input.attrs.as_mut();

    input_attrs.append(&mut attrs);

    Ok(quote!(#input))
}

/// #Description
/// Convert a snake case string to pascal case
pub fn convert_snake_to_pascal(input: &str) -> String {
    // We will over allocate here but not by much in most cases
    let mut ret = String::with_capacity(input.len());
    let mut chars = input.chars();

    // Ensure the first character is upper case
    if let Some(c) = chars.next() {
        ret.push_str(&c.to_uppercase().to_string());
    } else {
        return ret;
    };

    while let Some(c) = chars.next() {
        if c == '_' {
            if let Some(next_char) = chars.next() {
                ret.push_str(&next_char.to_uppercase().to_string())
            } else {
                break;
            }
        } else {
            ret.push(c)
        }
    }
    ret
}

