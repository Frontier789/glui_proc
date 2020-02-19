#![allow(dead_code)]

trait Widget {
    fn on_create(&self) {}
}

struct GuiContext {}

impl GuiContext {
    fn open_widget<T>(&mut self, w: &T) -> bool
    where
        T: Widget,
    {
        println!("A widget is opened");
        
        w.on_create();
        
        true
    }
    fn close_widget<T>(&mut self, _w: &T)
    where
        T: Widget,
    {
        println!("A widget is closed");
    }
}

struct Layout {
    height: usize,
}

impl Widget for Layout {
    fn on_create(&self) {
        println!("A layout of height {} is created", self.height);
    }
}

macro_rules! register_gui_element_struct_init {
    ( $class:ident { $( $field_in:ident : $value_in:expr ),* } @ ) => {
        $class {
            $( $field_in : $value_in ),*
        }
    };
    ( $class:ident { $( $field_in:ident : $value_in:expr ),* } @ children : $value_c:block $(, $field:ident : $value:expr )* $(,)? ) => {
        register_gui_element_struct_init! (
            $class { $( $field_in : $value_in ),* } @
            $( $field:ident : $value:expr ),*
        )
    };
    ( $class:ident { $( $field_in:ident : $value_in:expr ),* } @ $field_c:ident : $value_c:expr , $( $rest:tt )* ) => {
        register_gui_element_struct_init! (
            $class { $( $field_in : $value_in ),* $field_c : $value_c } @
            $( $rest )*
        )
    };
    ( $class:ident { $( $field_in:ident : $value_in:expr ),* } @ $field_c:ident : $value_c:expr ) => {
        register_gui_element_struct_init! (
            $class { $( $field_in : $value_in ),* $field_c : $value_c } @
        )
    };
}

macro_rules! register_gui_element_children {
    ( children : $f:expr, $( $x:tt),* $(,)? ) => {
        $f
    };
    ( $field_c:ident : $value_c:expr, $( $field:ident : $value:expr),* $(,)? ) => {
        register_gui_element_children! {
            $( $field : $value, )*
        }
    };
    ( $( $field:ident : $value:expr),* $(,)? ) => {
    }
}

macro_rules! register_gui_element {
    ($class:ident, $context:ident @ $( $x:tt )* ) => {
        {
            let tmp = register_gui_element_struct_init! { $class {} @ $( $x )* };
            if $context.open_widget(&tmp) {
                register_gui_element_children! { $( $x )* }
            }
            $context.close_widget(&tmp);
        }
    };
}

struct Button {
    text: String,
}

impl Widget for Button {
    fn on_create(&self) {
        println!("A button with text \"{}\" is created", self.text);
    }
}

fn call<T>(mut f: T)
where
    T: FnMut() -> (),
{
    f();
}

fn example(context: &mut GuiContext, data: i32) {
    call(|| {
        register_gui_element! { Button, context @
            text: "Im smart too".to_owned(),
        };
    });
    register_gui_element! { Layout, context @
        height: 200,
        children: {
            for i in 1..6 {
                register_gui_element! { Button, context @
                    text: format!("{}", i),
                };
            }
        },
    };
    register_gui_element! { Layout, context @
        height: 400,
        children: {
            register_gui_element! { Button, context @
                text: format!("A_{}", data),
            };
            register_gui_element! { Button, context @
                text: format!("B_{}", data),
            };
        }
    };
}

#[glui::builder]
fn tagged_function(context: &mut GuiContext, data: i32) {
    call(|| {
        Button {
            text: "Im smart too".to_owned(),
        };
    });
    Layout {
        height: 200,
        children: {
            for i in 1..6 {
                Button {
                    text: format!("{}", i),
                };
            }
        },
    };
    Layout {
        height: 400,
        children: {
            Button {
                text: format!("A_{}", data),
            };
            Button {
                text: format!("B_{}", data),
            };
        }
    };
}

#[test]
fn do_test() {
    tagged_function(&mut GuiContext {}, 1);
}
