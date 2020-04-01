#![allow(dead_code)]

trait Widget {
    fn on_create(&self) {}
}

struct WidgetParser {}

impl WidgetParser {
    fn open_widget<T>(w: &T)
    where
        T: Widget,
    {
        // you can use thread_local here as self
        println!("A widget is opened");
        
        w.on_create();
    }
    fn close_widget<T>(_w: &T)
    where
        T: Widget,
    {
        println!("A widget is closed");
    }
    fn push_cache(cache_id: u64) {
        println!("Pushed a new cache id: {}", cache_id);
    }
    fn pop_cache() {
        println!("Popped the last cache id");
    }
    fn register_param<T>(param: &T) where T: std::fmt::Display {
        println!("Registered param {}", param);
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

macro_rules! register_gui_element_outer {
    ( children : $f:expr, $( $x:tt),* $(,)? ) => {
        $f
    };
    ( $field_c:ident : $value_c:expr, $( $field:ident : $value:expr),* $(,)? ) => {
        register_gui_element_outer! {
            $( $field : $value, )*
        }
    };
    ( $( $field:ident : $value:expr),* $(,)? ) => {
    }
}

macro_rules! register_gui_element {
    ($class:ident, $build_param:ty, $parser:ident @ $( $x:tt )* ) => {
        {
            let tmp = register_gui_element_struct_init! { $class {} @ $( $x )* };
            
            $parser::open_widget(&tmp);
            
            register_gui_element_outer! { $( $x )* }
            
            $parser::close_widget(&tmp);
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

struct Data {}

impl Data {}

#[glui::builder(Data)]
fn tagged_function(data: i32) {
    call(|| {
        Button {
            text: "Im smart too".to_owned(),
        };
    });
    Layout {
        height: 200,
        children: {
            for i in 1..6 {
                button_builder(format!("button text is {}", i));
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

#[glui::builder(Data)]
fn button_builder(text: String) {
    Button {
        text: text,
    };
}

#[test]
fn do_test() {
    tagged_function(1);
}
