#![allow(dead_code)]

trait Widget {
    fn build(&self, context: &GuiContext);
}

struct GuiContext {}

impl GuiContext {
    fn add_widget<T>(&self, w: &T)
    where
        T: Widget,
    {
        w.build(self);
    }
}

struct Layout<T>
where
    T: Fn() -> (),
{
    height: usize,
    content: T,
}

impl<T> Widget for Layout<T>
where
    T: Fn() -> (),
{
    fn build(&self, _context: &GuiContext) {
        println!("Entering layout with height {}", self.height);
        (self.content)();

        println!("Exiting layout with height {}", self.height);
    }
}

struct Button {
    text: String,
}

impl Widget for Button {
    fn build(&self, _context: &GuiContext) {
        println!("Button with text: {}", self.text);
    }
}

fn call<T>(f: T)
where
    T: Fn() -> (),
{
    f();
}

#[glui::builder]
fn tagged_function(context: &GuiContext, data: i32) {
    call(|| {
        Button {
            text: "Im smart too".to_owned(),
        };
    });
    Layout {
        height: 200,
        content: || {
            for i in 1..6 {
                Button {
                    text: format!("{}", i),
                };
            }
        },
    };
    Layout {
        height: 400,
        content: || {
            Button {
                text: format!("A_{}", data),
            };
            Button {
                text: format!("B_{}", data),
            };
        },
    };
}

#[test]
fn do_test() {
    tagged_function(&GuiContext{}, 1);
}
