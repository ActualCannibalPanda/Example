use proc_macro::{TokenStream, TokenTree};
use shaderz::shader::{ArgumentList, ShaderFunction, ShaderType};

#[proc_macro]
pub fn function(item: TokenStream) -> TokenStream {
    let mut i = item.into_iter();
    let mut f = i.next();
    let mut name: Option<String> = None;
    let mut rettype: Option<ShaderType> = None;
    let mut arguments: Option<ArgumentList> = None;
    while f.is_some() {
        let item = f.unwrap();
        match &item {
            TokenTree::Ident(i) => {
                if rettype.is_none() {
                    rettype = match i.to_string().as_str() {
                        "int" => Some(ShaderType::INT),
                        "float" => Some(ShaderType::FLOAT),
                        _ => panic!("Failed to parse return type"),
                    };
                } else {
                    name = Some(i.to_string());
                }
            }
            TokenTree::Group(g) => {
                let mut i = g.stream().into_iter();
                while let Some(value) = i.next() {
                    let mut t: Option<ShaderType> = None;
                    let mut name: Option<String> = None;
                    t = match value {
                        TokenTree::Ident(ident) => match ident.to_string().as_str() {
                            "int" => Some(ShaderType::INT),
                            "float" => Some(ShaderType::FLOAT),
                            _ => None,
                        },
                        _ => None,
                    };
                    let item = i.next();
                    if let Some(ref token) = item {
                        name = match &token {
                            TokenTree::Ident(ident) => Some(ident.to_string()),
                            _ => None,
                        };
                    }
                    if t.is_some() && name.is_some() {
                        match arguments {
                            Some(ref mut arglist) => {
                                arglist.arguments.push((t.unwrap(), name.unwrap()));
                            }
                            _ => {
                                arguments = Some(ArgumentList {
                                    arguments: vec![(t.unwrap(), name.unwrap())],
                                });
                            }
                        }
                    }
                    let item = i.next();
                    match item {
                        Some(_) => {}
                        _ => break,
                    }
                }
            }
            _ => {}
        };
        f = i.next();
    }
    let func = ShaderFunction {
        t: rettype.unwrap(),
        name: name.unwrap(),
        arguments: arguments.unwrap(),
    };
    println!("{}", func);
    TokenStream::new()
}
