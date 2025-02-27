use gen_components::*;
use makepad_widgets::*;

live_design! {
    // import makepad_widgets::base::*;
    // import makepad_widgets::theme_desktop_dark::*;
    // import gen_components::components::*;
    use link::widgets::*;
    use link::gen_components::*;
    use link::shaders::*;

    pub BOLD_FONT2 = dep("crate://self/resources/FiraCode-Bold.ttf");
    pub SettingsPage = {{SettingsPage}}{
        height: Fill,
        width: Fill,
        flow: Down,
        border_radius: 0.0,
        background_color: #1F1616,
        align: {
            x: 0.5,
            y: 0.0
        },
        spacing: 24.0,
        padding: {
            left: 16.0,
            right: 16.0,
            top: 24.0,
            bottom: 24.0
        }
        header_view = <GView>{
            theme: Dark,
            block_child_events: true,
            border_radius: 6.0,
            height: 280.0,
            width: Fill,
            spacing: 24.0,
            padding: {
                left: 16.0,
                right: 16.0,
                top: 32.0,
                bottom: 32.0
            },
            align: {x: 0.5, y: 0.5},
            flow: Down,
            spread_radius: 0.0,
            blur_radius: 6.2,
            shadow_color: #aa6228,
            cursor: Hand,
            animation_key: true,
            clip_x: false,
            clip_y: false,
            draw_view: {
                instance center: vec2(0.96, 0.96)
                fn get_spread_radius(self) -> float {
                    return mix(
                        0.0,
                        5.2,
                        self.hover
                    )
                }

                fn get_background_color(self) -> vec4 {
                    // let gradient_angle = 30.0;
                    // let gdirection = vec2(cos(radians(gradient_angle)), sin(radians(gradient_angle)));
                    // let gfactor = dot(self.pos, gdirection);

                    // let gcolor0 = #21252C;   // #00FF00
                    // let gstop0 = 0.72;

                    // let gcolor1 = #52241C;   // #FF00FF
                    // let gstop1 = 0.86;

                    // // let color2 = #ff9900;   // #121212
                    // let gcolor2 = #82440F;
                    // let gstop2 = 0.96;

                    // let g_color =  mix(
                    //     gcolor0,
                    //     mix(
                    //         gcolor1,
                    //         mix(gcolor2, gcolor2, smoothstep(gstop2, gstop2, gfactor)),
                    //         smoothstep(gstop1, gstop2, gfactor)
                    //     ),
                    //     // color1,
                    //     smoothstep(gstop0, gstop1, gfactor)
                    // );
                    let center = self.center;
                    let distance = distance (self.pos , center) ;
                    let factor = clamp (distance , 0.0 , 1.0) ;
                    let color0 = #82440F;
                    let stop0 = 0.0 ;
                    let color1 = #52241C;
                    let stop1 = 0.3;
                    let color2 = #1F1616;
                    let stop2 = 1.0 ;
                    // return mix(
                    //     mix (color0 , mix (color1 , color2 , smoothstep (stop1 , stop2 , factor)) , smoothstep (stop0 , stop1 , factor)) ,
                    //     g_color,

                    //     self.hover
                    // );
                    return mix (color0 , mix (color1 , color2 , smoothstep (stop1 , stop2 , factor)) , smoothstep (stop0 , stop1 , factor));
                }

            }
            <GVLayout>{
                height: Fit,
                width: Fit,
                align: {x: 0.5, y: 0.5},
                spacing: 16.0,
                <GImage>{
                    height: 52.0,
                    width: 52.0,
                    src: dep("crate://self/resources/aws.png"),
                }
                <GLabel>{
                    font_size: 12.0,
                    font_family: (BOLD_FONT2),
                    text: "AWS S3 User",
                }

            }
            <GVLayout>{
                height: Fill,
                width: Fill,
                spacing: 16.0,
                <GLabel>{
                    font_size: 10.0,
                    font_family: (BOLD_FONT2),
                    text: "My S3 Bucket Size",
                }
                <GHLayout>{
                    height: Fit,
                    align: {x: 1.0, y: 0.5},
                    size_total = <GLabel>{
                        font_size: 9.0,
                        font_family: (BOLD_FONT2),
                        text: "621GB",
                        color: #E36741,
                    }
                }
                <GProgress>{
                    theme: Dark,
                    value: 0.4,
                    width: Fill,
                    margin: {bottom: 8.0},
                }
                <GLabel>{
                    font_size: 9.0,
                    font_family: (BOLD_FONT2),
                    text: "Availible: 1436GB",
                }
            }
        }
        <GHLayout>{
            height: Fit,
            width: Fill,
            spacing: 16.0,
            change_btn = <GButton>{
                height: 42.0,
                width: Fill,
                theme: Warning,
                background_color: #EC4925,
                hover_color: #E97056,
                focus_color: #C73514,
                slot: {
                    text: "Change Config",
                    font_size: 10.0,
                    font_family: (BOLD_FONT2),
                }
            }
            <GButton>{
                height: 42.0,
                width: Fill,
                theme: Info,
                slot: <GHLayout>{
                    spacing: 6.0,
                    align: {x: 0.5, y: 0.5},
                    <GIcon>{
                        icon_type: Help,
                        theme: Info,
                        height: 18.0,
                        width: 18.0,
                    }
                    <GLabel>{
                        text: "Help?",
                        font_size: 10.0,
                        font_family: (BOLD_FONT2),
                    }
                }
            }
        }
        qt = <GButton>{
            height: 42.0,
            width: Fill,
            theme: Error,
            background_color: #EC4925,
            hover_color: #E97056,
            focus_color: #C73514,
            // draw_button: {
            //     fn get_color(self) -> vec4 {
            //         let gradient_angle = 0.0;
            //         let direction = vec2(cos(radians(gradient_angle)), sin(radians(gradient_angle)));
            //         let factor = dot(self.pos, direction);

            //         let color0 = #F04438;   // #00FF00
            //         let stop0 = 0.4;

            //         let color1 = #4F1311;   // #FF00FF
            //         let stop1 = 0.8;

            //         // let color2 = #ff9900;   // #121212
            //         let color2 = #431412;
            //         let stop2 = 0.96;

            //         let hover_color = mix(
            //             color0,
            //             mix(
            //                 color1,
            //                 mix(color2, color2, smoothstep(stop2, stop2, factor)),
            //                 smoothstep(stop1, stop2, factor)
            //             ),
            //             // color1,
            //             smoothstep(stop0, stop1, factor)
            //         );

            //         let focus_color = mix(
            //             color2,
            //             mix(
            //                 color1,
            //                 mix(color0, color0, smoothstep(stop2, stop2, factor)),
            //                 smoothstep(stop1, stop2, factor)
            //             ),
            //             // color1,
            //             smoothstep(stop0, stop1, factor)
            //         );

            //         return mix(
            //             mix(
            //                 #1F1616,
            //                 hover_color,
            //                 self.hover
            //             ),
            //             focus_color,
            //             self.focus
            //         );
            //     }
            // }
            slot: <GHLayout>{
                spacing: 12.0,
                align: {x: 0.5, y: 0.5},
                <GIcon>{
                    icon_type: Exit,
                    theme: Info,
                    height: 18.0,
                    width: 18.0,
                    stroke_width: 1.5,
                }
                <GLabel>{
                    text: "Quit AWS S3 App",
                    font_size: 10.0,
                    font_family: (BOLD_FONT2),
                }
            }
        }

    }
}

#[derive(Live, Widget)]
pub struct SettingsPage {
    #[deref]
    pub super_widget: GView,
}

impl LiveHook for SettingsPage {}

impl Widget for SettingsPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.super_widget.draw_walk(cx, scope, walk)
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.super_widget.handle_event(cx, event, scope));
        self.gbutton(id!(change_btn)).borrow().map(|x| {
            if x.clicked(&actions).is_some() {
                GRouter::nav_to_path(cx, self.widget_uid(), scope, id!(sigin_screen));
            }
        });
        let qt = self.gbutton(id!(qt));
        if qt.clicked(&actions).is_some() {
            cx.quit();
        }
        let header_view = self.gview(id!(header_view));
        if let Some(e) = header_view.hover_over(&actions) {
            let rect = header_view.borrow().unwrap().area().rect(cx);
            let pos = rect.pos;
            let size = rect.size;
            // here center is abs pos, need to calculate relative pos to the view
            let center = e.e.abs;
            // the real center pos is in [0.0, 1.0]
            let x = (center.x - pos.x) / size.x;
            let y = (center.y - pos.y) / size.y;
            let center = vec2(x as f32, y as f32);
            header_view.borrow_mut().unwrap().draw_view.apply_over(
                cx,
                live! {
                    center: (center)
                },
            );
        } else if header_view.hover_out(&actions).is_some() {
            header_view.borrow_mut().unwrap().draw_view.apply_over(
                cx,
                live! {
                    center: (vec2(0.96,0.96))
                },
            );
        }
    }
}
