use gen_components::components::card::GCard;
use makepad_widgets::*;

live_design!{
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import gen_components::components::*;
    import crate::components::personal_view::*;
    import crate::components::main_view::*;

    BOLD_FONT = dep("crate://self/resources/JuliaMono-BlackItalic.ttf");
    HeaderView = {{HeaderView}}{
        height: 246.0,
        width: Fill,
        flow: Down,
        border_radius: 0.0,
        background_color: #F6F9FE,
        <GHLayout>{
            spread_radius: 2.4,
            shadow_offset: vec2(0.0, 1.0),
            shadow_color: #222,
            clip_x: false,
            clip_y: false,
            blur_radius: 4.6,
            height: 46.0,
            spacing: 12.0,
            align: {
                x: 0.0,
                y: 0.5
            },
            padding: {left: 12.0, right: 12.0},
            background_visible: true,
            background_color: #F6F9FE,
            <GImage>{
                height: 32.0,
                width: 32.0,
                src: dep("crate://self/resources/aws.png"),
            }
            <GLabel>{
                font_size: 10.0,
                text: "Aws Personal Cloud Drive",
                color: #555,
                font_family: (BOLD_FONT),
            }
        }
        <GImage>{
            margin: {left: 12.0, right: 12.0},
            height: 260.0,
            width: Fill,
            src: dep("crate://self/resources/store.png"),
        }
    }
}

#[derive(Live, Widget)]
pub struct HeaderView {
    #[deref]
    pub super_widget: GCard,
    
}

impl LiveHook for HeaderView {
    
}

impl Widget for HeaderView {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.super_widget.draw_walk(cx, scope, walk)
    }
}