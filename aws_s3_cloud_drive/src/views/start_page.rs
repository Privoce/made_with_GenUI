use gen_components::components::view::GView;
use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import gen_components::components::*;

    BOLD_FONT = dep("crate://self/resources/JuliaMono-BlackItalic.ttf");
    BOLD_FONT2 = dep("crate://self/resources/FiraCode-Bold.ttf");
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
        <GImage>{
            margin: {left: 12.0, right: 12.0, bottom: 60.0},
            height: 200.0,
            width: 200.0,
            src: dep("crate://self/resources/pag.png"),
        }
        <GVLayout>{
            height: Fit,
            width: Fill,
            spacing: 16.0,
            align: {
                x: 0.5,
                y: 0.5
            },
            <GLabel>{
                font_size: 18.0,
                font_family: (BOLD_FONT),
                text: "AWS CloudS3Drive",
            }
            <GLabel>{
                font_family: (BOLD_FONT2),
                text: "Productivity Tool",
            }
        }

        <GDropDown>{
            mode: Popup,
            height: Fit,
            width: Fit,
            trigger_mode: Hover,
            position: Bottom,
            clip_x: false,
            clip_y: false,
            enter_btn = <GButton>{
                margin: {top: 60.0},
                height: Fit,
                width: Fit,
                padding: 0.0,
                background_visible: false,
                border_radius: 3.0,
                theme: Error,
                slot: <GImage>{
                    height: 36.0,
                    width: 36.0,
                    src: dep("crate://self/resources/aws.png"),
                }
            }
            popup :<GPopup> {
                theme: Dark,
                height: 60.0,
                width: 60.0,
                padding: {
                    top: 12.0,
                },
                background_visible: false,
                container: {
                    align: {x: 0.5, y: 0.5},
                    <GLabel>{
                        text:"Go!",
                        font_family: (BOLD_FONT2),
                    }
                }
            }
        }
        loading = <GLoading>{
            theme: Dark,
        }
    }
}

#[derive(Live, Widget)]
pub struct StartPage {
    #[deref]
    pub super_widget: GView,
}

impl LiveHook for StartPage {}

impl Widget for StartPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.super_widget.draw_walk(cx, scope, walk)
    }
    fn handle_event(&mut self, _cx: &mut Cx, _event: &Event, _scope: &mut Scope) {

        // 暂时不需要 -----------------------------------------------------------------------------
        // let actions = cx.capture_actions(|cx| self.super_widget.handle_event(cx, event, scope));
        // filter_widget_actions(&actions, self.widget_uid()).map(|actions| {
        //     actions.iter().for_each(|action| {
        //         if let GViewEvent::FingerUp(_) = action.cast() {
        //             let uid = self.widget_uid();
        //             cx.widget_action(
        //                 uid,
        //                 &scope.path,
        //                 StackNavigationAction::NavigateTo(live_id!(sigin_page_view)),
        //             );
        //         }
        //     });
        // });
    }
    fn is_visible(&self) -> bool {
        self.visible
    }
}
