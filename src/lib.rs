extern crate proc_macro;
extern crate syn;

use proc_macro::TokenStream;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// use syn::Block;
// use syn::Expr;
// use syn::Expr::*;
// use syn::Ident;
// use syn::Member;

use syn::export::*;
use syn::FnArg;
use syn::Pat;
use syn::Stmt;

use quote::quote;

// use syn::spanned::Spanned;
// use quote::quote_spanned;

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn arg_name(arg: &FnArg) -> String {
    match arg {
        FnArg::Receiver(_) => "self".to_owned(),
        FnArg::Typed(pat) => match &*pat.pat {
            Pat::Ident(patindent) => patindent.ident.to_string(),
            _ => "".to_owned(),
        },
    }
}

#[proc_macro_attribute]
pub fn cache(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut funsyn_tree = syn::parse_macro_input!(item as syn::ItemFn);
    let param_registrator = funsyn_tree
        .sig
        .inputs
        .iter()
        .map(|arg| {
            let name = arg_name(arg);
            syn::parse2::<Stmt>(
                ("WidgetParser::register_param(&".to_owned() + &name + ");")
                    .parse()
                    .unwrap(),
            )
            .unwrap()
        })
        .collect::<Vec<Stmt>>();

    funsyn_tree.block.stmts = [
        vec![syn::parse2::<Stmt>(
            ("WidgetParser::enter_builder(".to_owned()
                + &format!("{}", calculate_hash(&funsyn_tree.sig))
                + ");")
                .parse()
                .unwrap(),
        )
        .unwrap()],
        param_registrator,
        funsyn_tree.block.stmts,
        vec![syn::parse2::<Stmt>(("WidgetParser::leave_builder();").parse().unwrap()).unwrap()],
    ]
    .concat();
    quote!(
        #funsyn_tree
    )
    .into()
}

// struct MacroWrapper {
//     depth: usize,
//     build_data_type: syn::Type,
// }
//
// impl MacroWrapper {
//     #[allow(dead_code)]
//     fn lead_ws(&self) -> String {
//         String::from_utf8(vec![b' '; self.depth]).unwrap()
//     }
//
//     fn block(&mut self, block: &mut Block) {
//         self.depth += 1;
//         // eprintln!("{}block>",self.lead_ws());
//         for stmt in block.stmts.iter_mut() {
//             self.stmt(stmt);
//         }
//         // eprintln!("{}block<",self.lead_ws());
//     }
//
//     fn stmt(&mut self, stmt: &mut Stmt) {
//         self.depth += 1;
//         // eprint!("{}stmt>",self.lead_ws());
//         match stmt {
//             Stmt::Local(local) => {
//                 // eprintln!("{}-type: Local",self.lead_ws());
//                 if local.init != None {
//                     self.expr(&mut local.init.as_mut().unwrap().1, false);
//                 }
//             }
//             Stmt::Expr(expr) => {
//                 // eprintln!("{}-type: Expr",self.lead_ws());
//                 self.expr(expr, false);
//             }
//             Stmt::Semi(expr, _) => {
//                 // eprintln!("{}-type: Semi",self.lead_ws());
//                 self.expr(expr, true);
//             }
//             _ => {} // do not care about definitions currently (maybe later)
//         }
//         // eprintln!("{}stmt<",self.lead_ws());
//     }
//
//     fn expr(&mut self, expr: &mut Expr, semi: bool) {
//         self.depth += 1;
//         // eprint!("{}expr>",self.lead_ws());
//         match expr {
//             // Async(expr_async) => {},
//             // Await(expr_await) => {},
//             // Yield(expr_yield) => {},
//             Block(expr_block) => {
//                 // eprintln!("{}-type: Block",self.lead_ws());
//                 self.block(&mut expr_block.block);
//             }
//             Call(expr_call) => {
//                 // eprintln!("{}-type: Call",self.lead_ws());
//                 for arg in expr_call.args.iter_mut() {
//                     self.expr(arg, false);
//                 }
//             }
//             Closure(expr_closure) => {
//                 // eprintln!("{}-type: Closure",self.lead_ws());
//                 if expr_closure.asyncness == None {
//                     self.expr(&mut expr_closure.body, false);
//                 }
//             }
//             MethodCall(expr_method_call) => {
//                 // eprintln!("{}-type: MethodCall",self.lead_ws());
//                 self.expr(&mut expr_method_call.receiver, false);
//                 for arg in expr_method_call.args.iter_mut() {
//                     self.expr(arg, false);
//                 }
//             }
//             ForLoop(expr_for_loop) => {
//                 // eprintln!("{}-type: ForLoop",self.lead_ws());
//                 self.block(&mut expr_for_loop.body);
//             }
//             Group(expr_group) => {
//                 // eprintln!("{}-type: Group",self.lead_ws());
//                 self.expr(&mut expr_group.expr, false);
//             }
//             If(expr_if) => {
//                 // eprintln!("{}-type: If",self.lead_ws());
//                 self.block(&mut expr_if.then_branch);
//                 if expr_if.else_branch != None {
//                     self.expr(&mut expr_if.else_branch.as_mut().unwrap().1, false);
//                 }
//             }
//             Let(expr_let) => {
//                 // eprintln!("{}-type: Let",self.lead_ws());
//                 self.expr(&mut expr_let.expr, false);
//             }
//             Loop(expr_loop) => {
//                 // eprintln!("{}-type: Loop",self.lead_ws());
//                 self.block(&mut expr_loop.body);
//             }
//             Match(expr_match) => {
//                 // eprintln!("{}-type: Match",self.lead_ws());
//                 for arm in expr_match.arms.iter_mut() {
//                     self.expr(&mut arm.body, false);
//                 }
//             }
//             // pub attrs: Vec<Attribute>,
//             // pub path: Path,
//             // pub brace_token: token::Brace,
//             // pub fields: Punctuated<FieldValue, Token![,]>,
//             // pub dot2_token: Option<Token![..]>,
//             // pub rest: Option<Box<Expr>>,
//             Struct(expr_struct) => {
//                 // eprintln!("{}-type: Struct",self.lead_ws());
//                 for field_value in expr_struct.fields.iter_mut() {
//                     let mut parse_as_expr = true;
//
//                     if let Member::Named(ident) = &field_value.member {
//                         if ident.to_string() == "child".to_owned() {
//                             parse_as_expr = false;
//                             field_value.member = Member::Named(Ident::new("children", ident.span()));
//
//                             let expr = &mut field_value.expr;
//                             self.expr(expr, true);
//
//                             match syn::parse2::<Expr>(quote!(
//                                 {
//                                     #expr;
//                                 }
//                             )) {
//                                 Ok(new_expr) => {
//                                     field_value.expr = new_expr;
//                                 }
//                                 Err(e) => {
//                                     eprintln!("Failed to parse child -> chilren syn tree modification, e: {}", e);
//                                 }
//                             };
//
//                             // eprintln!("Found child member: {:?}", field_value);
//                         }
//                     }
//
//                     if parse_as_expr {
//                         self.expr(&mut field_value.expr, false);
//                     }
//                 }
//                 if semi {
//                     let path = &expr_struct.path;
//                     let fields = &expr_struct.fields;
//                     let dot2_token = &expr_struct.dot2_token;
//                     let rest = &expr_struct.rest;
//                     let build_data_type = self.build_data_type.clone();
//
//                     match syn::parse2::<Expr>(quote!(
//                         register_gui_element! { #path, #build_data_type, PostBox, WidgetParser @
//
//                             #fields
//                             #dot2_token
//                             #rest
//
//                         }
//                     )) {
//                         Ok(new_expr) => {
//                             *expr = new_expr;
//                         }
//                         Err(e) => {
//                             eprintln!("Failed to parse modified syn tree e: {}", e);
//                         }
//                     }
//                 }
//             }
//             Unsafe(expr_unsafe) => {
//                 // eprintln!("{}-type: Unsafe",self.lead_ws());
//                 self.block(&mut expr_unsafe.block);
//             }
//             While(expr_while) => {
//                 // eprintln!("{}-type: While",self.lead_ws());
//                 self.block(&mut expr_while.body);
//             }
//             _ => {}
//         }
//         // eprintln!("{}expr<",self.lead_ws());
//     }
// }

#[proc_macro_derive(System)]
pub fn derive_system(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = &input.ident;

    proc_macro::TokenStream::from(quote! {
        impl System for #name {}
    })
}

#[proc_macro_derive(Message)]
pub fn derive_message(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = &input.ident;

    proc_macro::TokenStream::from(quote! {
        impl Message for #name {}
    })
}

#[proc_macro_derive(Component)]
pub fn derive_component(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = &input.ident;

    // let cloner = clone_members(&input.data);

    proc_macro::TokenStream::from(quote! {
        impl Component for #name {
            // fn clone(&self) -> Self {
            //     Self {
            //         #cloner
            //     }
            // }
        }
    })
}

// fn clone_members(data: &syn::Data) -> TokenStream2 {
//     match *data {
//         syn::Data::Struct(ref data) => match data.fields {
//             syn::Fields::Named(ref fields) => {
//                 let recurse = fields.named.iter().map(|f| {
//                     let name = &f.ident;
//                     quote_spanned! { f.span() =>
//                         #name: self.#name.clone()
//                     }
//                 });
//                 quote! {
//                     #(#recurse, )*
//                 }
//             }
//             _ => unimplemented!(),
//         },
//         syn::Data::Enum(_) | syn::Data::Union(_) => unimplemented!(),
//     }
// }
