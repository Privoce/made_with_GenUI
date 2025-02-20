use std::env::current_dir;

use gen_components::{GButtonWidgetRefExt, GImageWidgetRefExt, GLabelWidgetRefExt};
// App.rs
use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use link::gen_components::*;
    use crate::new_page::*;
    IMG_DEFAULT_AVATAR = dep("crate://self/resources/default_avatar.png")

    

    App = {{App}} {
        ui: <Root>{
            main_window = <Window>{
                body = <ScrollXYView>{
                    flow: Down,
                    spacing: 16,
                    align: {
                        x: 0.5,
                        y: 0.5
                    },
                    show_bg: true,
                    draw_bg:{
                        fn pixel(self) -> vec4 {
                            return #2A2E37;
                        }
                    }
                    lb1 = <GLabel>{
                        font_size: 16,
                        text: "Hello World",
                    }
                    <GInput>{
                        height: 30,
                        width: 240,
                    }
                    <GTag>{
                        text: "Tag",
                    }
                    <GTag>{
                        theme: Error,
                        text: "Hello World"
                    }
                    icon = <GSvg>{
                        src: dep("crate://self/resources/icon_add.svg"),
                    }
                    <GLoading>{
                        theme: Error,
                    }
                    <GToggle>{
                        theme: Success
                    }
                    <GCollapse>{
                        width: 200,
                        header: {
                            <GLabel>{
                                text: "Collapse"
                            }
                        }
                    }
                    <ToolTipPage>{
                        
                    }
                    <GDropDown>{
                        offset: 6.0,
                        height: Fit,
                        width: Fit,
                        position: Right,
                        trigger = <GButton>{
                            slot: {
                                text:"Position: Right"
                            }
                        },
                        popup :<GPopup> {
                            height: 150.0,
                            width: 200.0,
                            container: <GPopupContainer> {
                                height: Fill,
                                width: Fill,
                                flow: Down,
                                spacing: 10.0,
                                padding: 10.0,
                                <GLabel>{
                                    text:"This is a popup",
                                }
                                <GLabel>{
                                    text:"This is a popup",
                                }
                            }
                        }
                    }
                    // img = <GImage>{
                    //     height: 100,
                    //     width: 100,
                    //     // src: File("/Users/shengyifei/projects/gen_ui/made_with_GenUI/components_test/resources/default_avatar.png"),
                    //     src: Url("https://avatars.githubusercontent.com/u/67356158?s=48&v=4")
                    // }
                    btn1 = <GButton> {
                        slot: {
                            text: "Local"
                        }
                    }
                    // btn2 = <GButton> {
                    //     slot: {
                    //         text: "base64"
                    //     }
                    // }
                    // btn3 = <GButton> {
                    //     slot: {
                    //         text: "http-url"
                    //     }
                    // }
                }
            }
        }
    }
}

#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        crate::makepad_widgets::live_design(cx);
        crate::gen_components::live_design(cx, None);
        crate::new_page::live_design(cx);
    }
}

impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        let img = self.ui.gimage(id!(img));
        let btn1 = self.ui.gbutton(id!(btn1));
        let icon = self.ui.icon(id!(icon));
        if let Some(_) = btn1.clicked(actions) {
            let current = current_dir().unwrap().join("resources/dek.png");
            // img.load(cx, current.display().to_string().as_str())
            //     .unwrap();

            // let _ = self.ui.glabel(id!(lb1)).set_color(cx, "#ff0000".to_string());
            // let _ = btn1.set_background_color(cx, "#00FF00".to_string());
            // self.ui.redraw(cx);

            // if let Some(mut icon) = icon.borrow_mut() {
            //     icon.apply_over(cx, live!{
                    
            //     });
            // }
            // icon.redraw(cx);

        }

        if let Some(_) = self.ui.gbutton(id!(btn3)).clicked(actions) {
            img.load(
                cx,
                "https://avatars.githubusercontent.com/u/67356158?s=48&v=4",
            )
            .unwrap();
            // self.ui.redraw(cx);
        }
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

app_main!(App);
