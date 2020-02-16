extern crate proc_macro;
extern crate syn;
use proc_macro::TokenStream;
use quote::quote;
use syn::export::*;
use syn::Block;
use syn::Expr;
use syn::Expr::*;
use syn::Stmt;

#[proc_macro_attribute]
pub fn builder(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut funsyn_tree = syn::parse_macro_input!(item as syn::ItemFn);

    make_gui_block(&mut funsyn_tree.block, 0);
    quote! (
        #funsyn_tree
    )
    .into()
}

#[allow(dead_code)]
fn lead_ws(depth: usize) -> String {
    String::from_utf8(vec![b' '; depth]).unwrap()
}

fn make_gui_block(block: &mut Block, depth: usize) {
    // eprintln!("{}block>",lead_ws(depth));
    for stmt in block.stmts.iter_mut() {
        make_gui_stmt(stmt, depth + 1);
    }
    // eprintln!("{}block<",lead_ws(depth));
}

fn make_gui_stmt(stmt: &mut Stmt, depth: usize) {
    // eprint!("{}stmt>",lead_ws(depth));
    match stmt {
        Stmt::Local(local) => {
            // eprintln!("{}-type: Local",lead_ws(depth));
            if local.init != None {
                make_gui_expr(&mut local.init.as_mut().unwrap().1, false, depth + 1);
            }
        }
        Stmt::Expr(expr) => {
            // eprintln!("{}-type: Expr",lead_ws(depth));
            make_gui_expr(expr, false, depth + 1);
        }
        Stmt::Semi(expr, _) => {
            // eprintln!("{}-type: Semi",lead_ws(depth));
            make_gui_expr(expr, true, depth + 1);
        }
        _ => {} // do not care about definitions currently (maybe later)
    }
    // eprintln!("{}stmt<",lead_ws(depth));
}

fn make_gui_expr(expr: &mut Expr, semi: bool, depth: usize) {
    // eprint!("{}expr>",lead_ws(depth));
    match expr {
        // Async(expr_async) => {},
        // Await(expr_await) => {},
        // Yield(expr_yield) => {},
        Block(expr_block) => {
            // eprintln!("{}-type: Block",lead_ws(depth));
            make_gui_block(&mut expr_block.block, depth + 1);
        }
        Call(expr_call) => {
            // eprintln!("{}-type: Call",lead_ws(depth));
            for arg in expr_call.args.iter_mut() {
                make_gui_expr(arg, false, depth + 1);
            }
        }
        Closure(expr_closure) => {
            // eprintln!("{}-type: Closure",lead_ws(depth));
            if expr_closure.asyncness == None {
                make_gui_expr(&mut expr_closure.body, false, depth + 1);
            }
        }
        MethodCall(expr_method_call) => {
            // eprintln!("{}-type: MethodCall",lead_ws(depth));
            make_gui_expr(&mut expr_method_call.receiver, false, depth + 1);
            for arg in expr_method_call.args.iter_mut() {
                make_gui_expr(arg, false, depth + 1);
            }
        }
        ForLoop(expr_for_loop) => {
            // eprintln!("{}-type: ForLoop",lead_ws(depth));
            make_gui_block(&mut expr_for_loop.body, depth + 1);
        }
        Group(expr_group) => {
            // eprintln!("{}-type: Group",lead_ws(depth));
            make_gui_expr(&mut expr_group.expr, false, depth + 1);
        }
        If(expr_if) => {
            // eprintln!("{}-type: If",lead_ws(depth));
            make_gui_block(&mut expr_if.then_branch, depth + 1);
            if expr_if.else_branch != None {
                make_gui_expr(
                    &mut expr_if.else_branch.as_mut().unwrap().1,
                    false,
                    depth + 1,
                );
            }
        }
        Let(expr_let) => {
            // eprintln!("{}-type: Let",lead_ws(depth));
            make_gui_expr(&mut expr_let.expr, false, depth + 1);
        }
        Loop(expr_loop) => {
            // eprintln!("{}-type: Loop",lead_ws(depth));
            make_gui_block(&mut expr_loop.body, depth + 1);
        }
        Match(expr_match) => {
            // eprintln!("{}-type: Match",lead_ws(depth));
            for arm in expr_match.arms.iter_mut() {
                make_gui_expr(&mut arm.body, false, depth + 1);
            }
        }
        Struct(expr_struct) => {
            // eprintln!("{}-type: Struct",lead_ws(depth));
            for field_value in expr_struct.fields.iter_mut() {
                make_gui_expr(&mut field_value.expr, false, depth + 1);
            }
            if semi {
                match syn::parse2::<Expr>(quote!(
                    context.add_widget(&#expr_struct)
                )) {
                    Ok(new_expr) => {
                        *expr = new_expr;
                    }
                    _ => {}
                }
            }
        }
        Unsafe(expr_unsafe) => {
            // eprintln!("{}-type: Unsafe",lead_ws(depth));
            make_gui_block(&mut expr_unsafe.block, depth + 1);
        }
        While(expr_while) => {
            // eprintln!("{}-type: While",lead_ws(depth));
            make_gui_block(&mut expr_while.body, depth + 1);
        }
        _ => {}
    }
    // eprintln!("{}expr<",lead_ws(depth));
}
