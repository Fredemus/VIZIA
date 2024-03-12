use vizia::prelude::*;

use crate::DemoRegion;

pub fn list(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, "List").class("title");
        Label::new(cx, "").class("paragraph");

        Divider::new(cx).top(Pixels(12.0)).bottom(Pixels(12.0));

        DemoRegion::new(
            cx,
            |cx| {
                Label::new(cx, "Hello Vizia");
            },
            r#"Label::new(cx, "Hello Vizia");"#,
        );
    })
    .class("panel");
}
