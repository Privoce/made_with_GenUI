use gen_components::components::{
    button::GButtonWidgetExt,
    card::{GCard, GCardWidgetExt},
    input::GInputWidgetExt,
    label::GLabelWidgetExt,
};
use makepad_widgets::*;

use crate::utils::APP_STATE;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import gen_components::components::*;

    BOLD_FONT = dep("crate://self/resources/JuliaMono-BlackItalic.ttf");
    BOLD_FONT2 = dep("crate://self/resources/FiraCode-Bold.ttf");
    SiginPage = {{SiginPage}}{
        height: Fill,
        width: Fill,
        flow: Down,
        border_radius: 0.0,
        background_color: #161616,
        align: {
            x: 0.5,
            y: 0.0
        },
        spacing: 18.0,
        <GVLayout>{
            height: 260.0,
            background_visible: true,
            background_color: #161923,
            <GImage>{
                margin: {left: 12.0, right: 12.0},
                height: 300.0,
                width: Fill,
                src: dep("crate://self/resources/store.png"),
            }
        }
        <GVLayout>{
            width: Fill,
            align: {x: 0.5, y: 0.0},
            spacing: 8.0,
            <GLabel>{
                font_family: (BOLD_FONT2),
                font_size: 20.0,
                text: "Connect And Config"
                margin: {bottom: 64.0},
            }
            <GVLayout>{
                height: Fit,
                spacing: 8.0,
                align: {x: 0.0, y: 0.5},
                padding: 16.0,
                <GLabel>{
                    font_family: (BOLD_FONT),
                    font_size: 10.0,
                    text: "AWS Access Key ID:",
                }
                accsee_key_input = <GInput>{
                    theme: Dark,
                    height: 32.0,
                    width: Fill,
                    placeholder: "Please input your access key",
                }
            }
            <GVLayout>{
                height: Fit,
                spacing: 8.0,
                align: {x: 0.0, y: 0.5},
                padding: 16.0,
                <GLabel>{
                    font_family: (BOLD_FONT),
                    font_size: 10.0,
                    text: "AWS Access Secret:",
                }
                secret_key_input = <GInput>{
                    theme: Dark,
                    height: 32.0,
                    width: Fill,
                    placeholder: "Please input your secret key",
                }
            }
            <GVLayout>{
                height: Fit,
                spacing: 8.0,
                align: {x: 0.0, y: 0.5},
                padding: 16.0,
                <GLabel>{
                    font_family: (BOLD_FONT),
                    font_size: 10.0,
                    text: "Region:",
                }
                accsee_key_input = <GInput>{
                    theme: Dark,
                    height: 32.0,
                    width: Fill,
                    placeholder: "Please input your access key",
                }
            }
            <GVLayout>{
                height: Fit,
                spacing: 8.0,
                align: {x: 0.0, y: 0.5},
                padding: 16.0,
                <GLabel>{
                    font_family: (BOLD_FONT),
                    font_size: 10.0,
                    text: "Output Format:",
                }
                
            }
            res_str = <GLabel>{
                color: #FF7043,
                font_family: (BOLD_FONT2),
                font_size: 8.0,
                text: "Connect Result:",
                wrap: Word,
            }

            <GVLayout>{
                height: Fit,
                spacing: 16.0,
                align: {x: 0.5, y: 0.5},
                margin: {top: 24.0},
                download_btn = <GHLayout>{
                    visible: false,
                    height: Fit,
                    width: Fit,
                    spacing: 6.0,
                    align: {x: 0.5, y: 0.5},
                    <GIcon>{
                        height: 14.0,
                        width: 16.0,
                        icon_type: Download,
                        theme: Dark,
                    }
                    <GLink>{
                        theme: Error,
                        font_family: (BOLD_FONT),
                        font_size: 9.0,
                        text: "Download!",
                        href: "https://aws.amazon.com/cli/"
                    }
                }
                auto_connect = <GButton>{
                    theme: Warning,
                    width: 260.0,
                    slot: <GHLayout>{
                        height: Fit,
                        width: Fit,
                        spacing: 6.0,
                        align: {x: 0.5, y: 0.5},
                        <GIcon>{
                            height: 14.0,
                            width: 16.0,
                            icon_type: Connect,
                            theme: Dark,
                        }
                        <GLabel>{
                            font_family: (BOLD_FONT2),
                            font_size: 9.0,
                            text: "Auto Connect",
                        }
                    }
                }
                try_connect = <GButton>{
                    width: 260.0,
                    theme: Error,
                    slot: {
                        font_family: (BOLD_FONT2),
                        font_size: 9.0,
                        text: "Try Connect",
                    }
                }
            }
        }
    }
}

#[derive(Live, Widget)]
pub struct SiginPage {
    #[deref]
    pub super_widget: GCard,
}

impl LiveHook for SiginPage {}

impl Widget for SiginPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.super_widget.draw_walk(cx, scope, walk)
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.super_widget.handle_event(cx, event, scope));
        if self.gbutton(id!(auto_connect)).clicked(&actions).is_some() {
            // get state and call
            let mut state = APP_STATE.lock().unwrap();
            // check if the toolkit is available
            if !state.check_toolkit() {
                let _ = self.gcard(id!(download_btn)).borrow_mut().map(|mut x| {
                    x.visible = true;
                });
                self.redraw(cx);
            }
            self.glabel(id!(res_str))
                .set_text_and_redraw(cx, &state.msg);

            if state.check {
                state.msg.clear();
                // check config and credentials
                let _ = state.check_config_credentials();
                if state.login {
                    self.ginput(id!(accsee_key_input))
                        .borrow_mut()
                        .map(|mut x| {
                            x.text = state.accsee_key.to_string();
                        });
                    self.ginput(id!(secret_key_input))
                        .borrow_mut()
                        .map(|mut x| {
                            x.text = state.secret_key.to_string();
                        });
                    // todo nav to main page
                }
            }
            self.glabel(id!(res_str))
                .set_text_and_redraw(cx, &state.msg);
        }

        if self.gbutton(id!(try_connect)).clicked(&actions).is_some() {
            let mut state = APP_STATE.lock().unwrap();
            self.ginput(id!(accsee_key_input)).borrow().map(|x| {
                state.accsee_key = x.text.to_string();
            });
            self.ginput(id!(secret_key_input)).borrow().map(|x| {
                state.secret_key = x.text.to_string();
            });
            state.sso_login();
        }
    }
}
