use std::fmt::Debug;
use std::ops::Neg;
use std::ops::Shl;

trait Widget: Debug {}

struct WidgetParser {}

impl WidgetParser {
    fn doprint() -> bool {
        true
    }
    fn open_widget<T>(w: &T)
    where
        T: Widget,
    {
        // you can use thread_local here as self
        if Self::doprint() {
            println!("Widget {:?} is opened", w);
        }
    }
    fn close_widgets(n: u32) {
        if Self::doprint() {
            println!("< {} --", n);
        }
    }
    fn enter_builder(cache_id: u64) {
        if Self::doprint() {
            println!("Entered a new builder, id: {}", cache_id);
        };
    }
    fn leave_builder() {
        if Self::doprint() {
            println!("Left the last builder");
        }
    }
    fn register_param<T>(param: &T)
    where
        T: std::fmt::Debug,
    {
        if Self::doprint() {
            println!("Registered param {:?}", param);
        };
    }
}

#[derive(Debug)]
struct WidgetAdder {
    depth: u32,
}

impl WidgetAdder {
    pub fn new() -> WidgetAdder {
        WidgetAdder { depth: 1 }
    }
}

impl Drop for WidgetAdder {
    fn drop(&mut self) {
        if self.depth > 0 {
            WidgetParser::close_widgets(self.depth);
        }
    }
}

impl Shl<()> for WidgetAdder {
    type Output = ();

    fn shl(self, _rhs: ()) -> Self::Output {
        ()
    }
}

impl Shl<WidgetAdder> for WidgetAdder {
    type Output = WidgetAdder;

    fn shl(mut self, mut rhs: WidgetAdder) -> Self::Output {
        rhs.depth = 0;
        self.depth += 1;
        self
    }
}

macro_rules! impl_widget {
    ($t:ty) => {
        impl Neg for $t {
            type Output = WidgetAdder;

            fn neg(self) -> Self::Output {
                WidgetParser::open_widget(&self);
                WidgetAdder::new()
            }
        }

        impl Widget for $t {}
    };
}

#[derive(Debug, Default)]
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

#[allow(unused_must_use)]
impl UIBuilder for UIExperiment {
    #[glui_proc::cache]
    fn build(&self) {
        call(move || {
            -Button {
                text: "Imma button inna call".to_owned(),
            };
        });

        -Layout { height: 200.0 }
            << for i in 1..6 {
                button_builder(format!("button text is {}", i));
            };

        -Layout { height: 400.0 } << {
            -Button {
                text: format!("A_{}", self.data),
            };
            -Button {
                text: format!("B_{}", self.data),
            };
        };

        -Layout { height: 100.0 }
            << -Layout { height: 200.0 }
            << -Button {
                text: format!("A_{}", self.data),
            };
    }
}

#[allow(unused_must_use)]
#[glui_proc::cache]
fn button_builder(text: String) {
    -Button { text };
}

#[test]
fn do_test() {
    let builder = UIExperiment { data: 5 };

    builder.build();
}
