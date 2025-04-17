use proc_macro::{TokenStream, TokenTree};

#[proc_macro]
pub fn function(item: TokenStream) -> TokenStream {
    let _ = item.clone().into_iter().map(|x| match x {
        TokenTree::Ident(a) => println!("Ident: {}", a),
        TokenTree::Group(g) => println!("Group: {}", g),
        TokenTree::Literal(l) => println!("Literal: {}", l),
        TokenTree::Punct(p) => println!("Punctuation: {}", p),
    });
    println!("{}", item);
    TokenStream::new()
}
