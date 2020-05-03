use std::fmt::Debug;
use std::ops::Neg;
use std::ops::Shr;

trait Widget: Debug {}

struct WidgetParser {}

impl WidgetParser {
    fn doprint() -> bool { false }
    fn open_widget<T>(w: &T)
        where
            T: Widget,
    {
        // you can use thread_local here as self
        if Self::doprint() { println!("A widget is opened"); }
    }
    fn close_widget<T>(_w: &T)
        where
            T: Widget,
    {
        if Self::doprint() { println!("A widget is closed"); }
    }
    fn enter_builder(cache_id: u64) {
        if Self::doprint() { println!("Entered a new builder, id: {}", cache_id); };
    }
    fn leave_builder() {
        if Self::doprint() { println!("Left the last builder"); }
    }
    fn register_param<T>(param: &T)
        where
            T: std::fmt::Debug,
    {
        if Self::doprint() { println!("Registered param {:?}", param); };
    }
}

#[derive(Debug)]
struct WidgetAdder {}

impl WidgetAdder {
    pub fn new() -> WidgetAdder {
        // println!("-- children =>");
        WidgetAdder {}
    }
}

impl Drop for WidgetAdder {
    fn drop(&mut self) {
        println!("< children --")
    }
}

impl Shr<()> for WidgetAdder {
    type Output = ();

    fn shr(self, _rhs: ()) -> Self::Output {
        ()
    }
}

#[derive(Debug)]
struct WidgetAdderLeaf {}

impl Drop for WidgetAdderLeaf {
    fn drop(&mut self) {
        println!("< children --")
    }
}

impl Shr<()> for WidgetAdderLeaf {
    type Output = WidgetAdderLeaf;

    fn shr(self, _rhs: ()) -> Self::Output {
        self
    }
}

impl Shr<WidgetAdder> for WidgetAdder {
    type Output = WidgetAdderLeaf;

    fn shr(self, _rhs: WidgetAdder) -> Self::Output {
        WidgetAdderLeaf{}
    }
}

macro_rules! impl_widget {
    ($t:ty) => {
        impl Neg for $t {
            type Output = WidgetAdder;
        
            fn neg(self) -> Self::Output {
                println!("Constructed {:?}", self);
                WidgetAdder::new()
            }
        }
        
        impl Widget for $t {}
    }
}

#[derive(Debug)]
struct Layout {
    height: f32,
}
impl_widget!(Layout);

#[derive(Debug)]
struct Button {
    text: String,
}
impl_widget!(Button);

fn call<T>(mut f: T)
    where
        T: FnMut() -> (),
{
    f();
}

#[derive(Debug)]
struct UIExperiment {
    data: i32,
}

trait UIBuilder {
    fn build(&self);
}

impl UIBuilder for UIExperiment {
    #[glui_proc::cache]
    fn build(&self) {
        call(|| {
            -Button {
                text: "Imma button inna call".to_owned(),
            };
        });

        -Layout {
            height: 200.0,
        } >> for i in 1..6 {
            button_builder(format!("button text is {}", i));
        };

        -Layout {
            height: 400.0,
        } >> {
            -Button {
                text: format!("A_{}", self.data),
            };
            -Button {
                text: format!("B_{}", self.data),
            };
        };

        -Layout {
            height: 100.0,
        } >> {
            -Layout {
                height: 200.0,
            } >> -Button {
                text: format!("A_{}", self.data),
            };
        };
    }
}

#[glui_proc::cache]
fn button_builder(text: String) {
    -Button { text };
}

#[test]
fn do_test() {
    let builder = UIExperiment { data: 5 };

    builder.build();
}
