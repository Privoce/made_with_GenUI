use gen_components::components::card::GCard;
use makepad_widgets::*;

live_design!{
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import gen_components::components::*;
    import crate::components::personal_view::*;
    import crate::components::main_view::*;

    BOLD_FONT = dep("crate://self/resources/JuliaMono-BlackItalic.ttf");
    BodyView = {{BodyView}}{
        height: Fill,
        width: Fill,
        flow: Down,
        border_radius: 0.0,
        background_color: #F6F9FE,
        <GVLayout>{
            height: 60.0,
            padding: 6.0,
            spacing: 6.0,
            <GHLayout>{
                height: 32.0,
                align: {x: 0.0, y: 0.5},
                spacing: 6.0,
                <GDropDown>{
                    mode: ToolTip,
                    height: Fit,
                    width: Fit,
                    trigger_mode: Hover,
                    position: BottomLeft,
                    trigger = <GIcon>{
                        icon_type: Help,
                        theme: Error,
                        height: 20.0,
                        width: 20.0,
                        stroke_width: 1.2,
                    }
                    popup :<GToolTip> {
                        theme: Dark,
                        height: 150.0,
                        width: 220.0,
                        padding: {
                            top: 12.0,
                            
                        },
                        container: {
                            height: 150.0,
                            width: 220.0,
                            flow: Down,
                            <GLabel>{
                                margin: 6.0,
                                height: Fit,
                                width: 220.0,
                                text:"AWS Configurations is a file that stores your settings for AWS CLI, including your security credentials, default output format, and default region.",
                            }
                        }
                    }
                }
                
                <GLabel>{
                    color: #E36640,
                    font_family: (BOLD_FONT),
                    text: "Processing Progress..."
                }
            }
            <GProgress>{
                theme: Dark,
                value: 0.5,
                read_only: false,
                width: Fill,
            }
        }
    }
}

#[derive(Live, Widget)]
pub struct BodyView {
    #[deref]
    pub super_widget: GCard,
    
}

impl LiveHook for BodyView {
    
}

impl Widget for BodyView {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.super_widget.draw_walk(cx, scope, walk)
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.super_widget.handle_event(cx, event, scope)
    }
}