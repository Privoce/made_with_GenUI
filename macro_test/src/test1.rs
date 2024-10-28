use gen_components::components::view::GView;
use gen_macros::prop;
use makepad_widgets::*;

use crate::events::{TestE2, A2};

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import gen_components::components::*;


    Tw = {{Tw}}{
        height: Fill,
        width: Fill,
        flow: Down,
        border_radius: 0.0,
        background_visible: true,
        background_color: #FFFFFF,
        align: {
            x: 0.5,
            y: 0.4
        },
        spacing: 24.0,
        <GLabel>{
            color: #000000,
            font_size: 18.0,
            text: "Hello World",
        }
    }
}

#[prop]
#[derive(LiveHook)]
pub struct Tw {
    #[live(100.0)]
    counter: f32,
}


impl Widget for Tw {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        dbg!("draw walk");
        self.deref_widget.draw_walk(cx, scope, walk)
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        match event.hits(cx, self.area()) {
            Hit::FingerUp(_) => {
                dbg!("finger up, do apply over");
                self.apply_over(
                    cx,
                    live! {
                        background_color: #FF0000
                    },
                );

                cx.widget_action(
                    self.widget_uid(),
                    &scope.path,
                    TestE2::TestA2(A2 { a: 19.0 }),
                );
            }
            _ => {}
        }
    }
}
