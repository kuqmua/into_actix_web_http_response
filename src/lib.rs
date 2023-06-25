#[proc_macro_derive(IntoActixWebHttpResponse)]
pub fn into_actix_web_http_response(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_helpers::panic_location::panic_location(); //panic_location function from https://github.com/kuqmua/proc_macro_helpers
    let ast: syn::DeriveInput = syn::parse(input)
        .unwrap_or_else(|_| panic!("let ast: syn::DeriveInput = syn::parse(input) failed"));
    let ident = &ast.ident;
    let gen = quote::quote! {
        impl From<#ident> for actix_web::HttpResponse {
            fn from(val: #ident) -> Self {
                let mut actix_web_http_response: actix_web::HttpResponseBuilder = (&val).into();
                actix_web_http_response.json(actix_web::web::Json(val))
            }
        }
    };
    //println!("{gen}");
    gen.into()
}
