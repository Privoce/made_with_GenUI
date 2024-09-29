use gen_components::components::{card::GCard, label::GLabelWidgetExt};
use makepad_widgets::*;

use crate::state::DATA;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import gen_components::components::*;


    StartPage = {{StartPage}}{
        height: Fill,
        width: Fill,
        flow: Down,
        border_radius: 0.0,
        background_color: #161616,
        align: {
            x: 0.5,
            y: 0.4
        },
        spacing: 24.0,

        <GVLayout>{
            height: Fit,
            width: Fill,
            spacing: 16.0,
            align: {
                x: 0.5,
                y: 0.5
            },
            a = <GLabel>{
                font_size: 18.0,

                text: "AWS CloudS3Drive",
            }
            b = <GLabel>{

                text: "Productivity Tool",
            }
        }


    }
}

#[derive(Live, Widget)]
pub struct StartPage {
    #[deref]
    pub super_widget: GCard,
}

impl LiveHook for StartPage {
    fn before_apply(
        &mut self,
        cx: &mut Cx,
        _apply: &mut Apply,
        _index: usize,
        _nodes: &[LiveNode],
    ) {
        dbg!("before");
        let data = DATA.lock().unwrap();
        self.glabel(id!(a)).set_text_and_redraw(cx, &data.data1);
        self.glabel(id!(b)).set_text_and_redraw(cx, &data.data2.to_string());
    }

    fn after_apply(
        &mut self,
        _cx: &mut Cx,
        _apply: &mut Apply,
        _index: usize,
        _nodes: &[LiveNode],
    ) {
        dbg!("after apply");
    }
    fn after_new_from_doc(&mut self, _cx: &mut Cx) {
        dbg!("new from doc");
    }

    fn after_update_from_doc(&mut self, _cx: &mut Cx) {
        dbg!("upadte from doc");
    }

    fn after_apply_from_doc(&mut self, _cx: &mut Cx) {
        dbg!("apply from doc");
    }

    fn after_new_before_apply(&mut self, _cx: &mut Cx) {
        dbg!("new before apply");
    }
}

impl Widget for StartPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.super_widget.draw_walk(cx, scope, walk)
    }
    fn handle_event(&mut self, _cx: &mut Cx, _event: &Event, _scope: &mut Scope) {}
}
