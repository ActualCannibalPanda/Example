use procmacros::function;
// use shaderz::shader::{ArgumentList, ShaderFunction};

// macro_rules! my_function {
//     ( $rt:ident $name:ident ( $($argt:ident $argname:ident),* ) ) => {
//         ShaderFunction {
//             t: $rt,
//             name: $name,
//             arguments: ArgumentList{
//                 arguments: vec![$(($argt, $argname)),*]
//             }
//         }
//     };
// }

fn main() {
    function!(int hello_world(float a, int b));
}
