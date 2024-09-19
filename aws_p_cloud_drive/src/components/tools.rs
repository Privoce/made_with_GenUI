use gen_components::components::card::GCard;
use makepad_widgets::*;

live_design!{
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import gen_components::components::*;
    import crate::components::personal_view::*;
    import crate::components::main_view::*;

    BOLD_FONT = dep("crate://self/resources/JuliaMono-BlackItalic.ttf");
    ToolsView = {{ToolsView}}{
        height: 60.0,
        width: Fill,
        flow: Right,
        spacing: 12.0,
        border_radius: 0.0,
        background_color: #F6F9FE,
        background_visible: true,
        spread_radius: 2.0,
        shadow_offset: vec2(0.0, 1.0),
        shadow_color: #222,
        blur_radius: 4.6,
        clip_x: false,
        clip_y: false,
        align: {
            x: 0.5,
            y: 0.5
        },
        config = <GVLayout>{
            height: 60.0,
            width: 80.0,
            align: {
                x: 0.5,
                y: 0.5
            },
            spacing: 6.0,
            <GIcon>{
                icon_type: Setting3,
                theme: Dark,
                stroke_width: 1.5,
                height: 26.0,
                width: 30.0,
            }
            <GLabel>{
                color: #555,
                font_size: 8.0,
                text: "Config AWS",
                font_family: (BOLD_FONT),
            }
        }
        my_s3 = <GVLayout>{
            height: 60.0,
            width: 80.0,
            align: {
                x: 0.5,
                y: 0.5
            },
            spacing: 6.0,
            <GIcon>{
                icon_type: Note,
                theme: Dark,
                stroke_width: 1.5,
                height: 26.0,
                width: 26.0,
            }
            <GLabel>{
                color: #555,
                font_size: 8.0,
                text: "My S3",
                font_family: (BOLD_FONT),
            }
        }
        upload = <GVLayout>{
            height: 60.0,
            width: 80.0,
            align: {
                x: 0.5,
                y: 0.5
            },
            spacing: 6.0,
            <GIcon>{
                icon_type: Upload,
                theme: Dark,
                stroke_width: 1.5,
                height: 26.0,
                width: 26.0,
            }
            <GLabel>{
                color: #555,
                font_size: 8.0,
                text: "Upload",
                font_family: (BOLD_FONT),
            }
        }
        notice = <GVLayout>{
            height: 60.0,
            width: 80.0,
            align: {
                x: 0.5,
                y: 0.5
            },
            spacing: 6.0,
            <GIcon>{
                icon_type: Notice,
                theme: Dark,
                stroke_width: 1.5,
                height: 26.0,
                width: 26.0,
            }
            <GLabel>{
                color: #555,
                font_size: 8.0,
                text: "Notice",
                font_family: (BOLD_FONT),
            }
        }
    }
}

#[derive(Live, Widget)]
pub struct ToolsView {
    #[deref]
    pub super_widget: GCard,
    
}

impl LiveHook for ToolsView {
    
}

impl Widget for ToolsView {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.super_widget.draw_walk(cx, scope, walk)
    }
}