use gen_components::components::view::GView;
use makepad_widgets::*;

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
    }
}

#[derive(Live, Widget)]
pub struct Tw {
    #[deref]
    pub super_widget: GView
}

impl LiveHook for Tw {
    fn before_apply(
        &mut self,
        cx: &mut Cx,
        _apply: &mut Apply,
        _index: usize,
        _nodes: &[LiveNode],
    ) {
        dbg!("before");
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

impl Widget for Tw {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        dbg!("draw walk");
        self.super_widget.draw_walk(cx, scope, walk)
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        match event.hits(cx, self.area()){
            Hit::FingerUp(_) =>{
                dbg!("finger up, do apply over");
                self.apply_over(cx, live!{
                    background_color: #FF0000
                });
            }
            _ =>{

            }
        }
    }
}
