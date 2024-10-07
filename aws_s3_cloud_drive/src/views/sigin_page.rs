use gen_components::{
    components::{
        button::GButtonWidgetExt,
        input::GInputWidgetExt,
        label::GLabelWidgetExt,
        radio::group::GRadioGroupWidgetExt,
        router::GRouter,
        select::GSelectWidgetExt,
        view::{GView, GViewWidgetExt},
    },
    utils::lifetime::{Executor, Lifetime},
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
        background_color: #16191F,
        align: {
            x: 0.5,
            y: 0.0
        },
        spacing: 16.0,
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
                margin: {bottom: 16.0},
            }
            <GVLayout>{
                height: Fit,
                spacing: 4.0,
                align: {x: 0.0, y: 0.5},
                padding: {left: 16.0, right: 16.0},
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
                spacing: 4.0,
                align: {x: 0.0, y: 0.5},
                padding: {left: 16.0, right: 16.0},
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
                spacing: 4.0,
                align: {x: 0.0, y: 0.5},
                padding: {left: 16.0, right: 16.0},
                <GLabel>{
                    font_family: (BOLD_FONT),
                    font_size: 10.0,
                    text: "Region:",
                }
                region_select = <GSelect>{
                    theme: Dark,
                    width: Fill,
                    background_visible: true,
                    color: #1F1F1F,
                    select_options: {
                        height: 180.0,
                        width: 380.0
                    }
                }
            }
            <GVLayout>{
                height: Fit,
                spacing: 4.0,
                align: {x: 0.0, y: 0.5},
                padding: {left: 16.0, right: 16.0},
                <GLabel>{
                    font_family: (BOLD_FONT),
                    font_size: 10.0,
                    text: "Output Format:",
                }
                output_group = <GRadioGroup>{
                    width: Fill,
                    spacing: 16.0,
                    selected: 0,
                    <GRadio>{
                        theme: Dark,
                        font_family: (BOLD_FONT),
                        color: #FFF,
                        text: "json"
                    }
                    <GRadio>{
                        theme: Dark,
                        font_family: (BOLD_FONT),
                        color: #FFF,
                        text: "yaml"
                    }
                    <GRadio>{
                        theme: Dark,
                        font_family: (BOLD_FONT),
                        color: #FFF,
                        text: "text"
                    }
                }
            }
            res_str = <GLabel>{
                color: #FF7043,
                font_family: (BOLD_FONT2),
                font_size: 8.0,
                text: "Connect Result:",
                wrap: Word,
                width: Fill,
                margin: {
                    left: 8.0,
                    right: 8.0
                }
            }

            <GVLayout>{
                height: Fit,
                spacing: 12.0,
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
                        text: "Change Config",
                    }
                }
                back_setting = <GButton>{
                    width: 260.0,
                    theme: Dark,
                    slot: {
                        font_family: (BOLD_FONT2),
                        font_size: 9.0,
                        text: "Back",
                    }
                }
            }
        }
    }
}

#[derive(Live, Widget)]
pub struct SiginPage {
    #[deref]
    pub super_widget: GView,
    #[rust]
    pub lifetime: Lifetime,
}

impl LiveHook for SiginPage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        let _ = self.super_widget.after_apply(cx, apply, index, nodes);
        let _ = self.gselect(id!(region_select)).borrow_mut().map(|mut x| {
            x.options = vec![
                ("美国东部(俄亥俄) us-east-2", "us-east-2").into(),
                ("美国东部(弗吉尼亚州北部) us-east-1", "us-east-1").into(),
                ("美国西部(加利福尼亚州北部) us-west-1", "us-west-1").into(),
                ("美国西部(俄勒冈) us-west-2", "us-west-2").into(),
                ("亚太地区(首尔) ap-northeast-2", "ap-northeast-2").into(),
                ("亚太地区(东京) ap-northeast-1", "ap-northeast-1").into(),
                ("亚太地区(大阪) ap-northeast-3", "ap-northeast-3").into(),
                ("亚太地区(香港) ap-east-1", "ap-east-1").into(),
                ("亚太地区(孟买) ap-south-1", "ap-south-1").into(),
                ("亚太地区(新加坡) ap-southeast-1", "ap-southeast-1").into(),
                ("亚太地区(悉尼) ap-southeast-2", "ap-southeast-2").into(),
                ("加拿大(中部) ca-central-1", "ca-central-1").into(),
                ("中国(北京) cn-north-1", "cn-north-1").into(),
                ("中国(宁夏) cn-northwest-1", "cn-northwest-1").into(),
                ("欧洲(法兰克福) eu-central-1", "eu-central-1").into(),
                ("欧洲(爱尔兰) eu-west-1", "eu-west-1").into(),
                ("欧洲(伦敦) eu-west-2", "eu-west-2").into(),
                ("欧洲(巴黎) eu-west-3", "eu-west-3").into(),
                ("欧洲(斯德哥尔摩) eu-north-1", "eu-north-1").into(),
                ("中东(巴林) me-south-1", "me-south-1").into(),
                ("南美洲(圣保罗) sa-east-1", "sa-east-1").into(),
                ("非洲(开普敦) af-south-1", "af-south-1").into(),
                ("AWS GovCloud(美国东部)", "us-gov-east-1").into(),
                ("AWS GovCloud(美国西部)", "us-gov-west-1").into(),
            ]
        });
    }
}

impl Widget for SiginPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.super_widget.draw_walk(cx, scope, walk);
        // self.get(cx);
        // self.lifetime = Lifetime::InProcess;
        self.lifetime.init().execute(|| self.get(cx)).map(|_| {
            self.lifetime.next();
        });

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.super_widget.handle_event(cx, event, scope));

        if self.gbutton(id!(auto_connect)).clicked(&actions).is_some() {
            // get state and call
            // let mut state = APP_STATE.lock().unwrap();
            // check if the toolkit is available
            self.check(cx);
            self.get(cx);
            return;
        }

        if self.gbutton(id!(try_connect)).clicked(&actions).is_some() {
            self.update();
            return;
        }
        if self.gbutton(id!(back_setting)).clicked(&actions).is_some() {
            self.lifetime = Lifetime::Destroy;
            // todo nav to main page
            self.lifetime
                .destroy()
                .execute(|| {
                    // GRouter::nav_to_path(cx, self.widget_uid(), scope, id!(setting_frame));
                    GRouter::nav_back(cx, self.widget_uid(), scope);
                })
                .map(|_| {
                    self.lifetime.next();
                });
        }
    }
}

impl SiginPage {
    pub fn check(&mut self, cx: &mut Cx) {
        let mut state = APP_STATE.lock().unwrap();
        if !state.check_toolkit() {
            let _ = self.gview(id!(download_btn)).borrow_mut().map(|mut x| {
                x.visible = true;
            });
            self.redraw(cx);
            self.glabel(id!(res_str))
                .set_text_and_redraw(cx, &state.msg);
        }
    }
    pub fn get(&mut self, cx: &mut Cx) {
        let state = APP_STATE.lock().unwrap();
        if state.check && state.login {
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
            self.gselect(id!(region_select)).borrow_mut().map(|mut x| {
                for (index, option) in x.options.clone().iter().enumerate() {
                    if option.value == state.region {
                        x.selected = index;
                    }
                }
            });
            self.gradio_group(id!(output_group))
                .borrow_mut()
                .map(|mut x| {
                    x.set_selected(
                        cx,
                        match state.output.as_str() {
                            "json" => 0,
                            "yaml" => 1,
                            "text" => 2,
                            _ => 0,
                        },
                    );
                });
            self.glabel(id!(res_str))
                .set_text_and_redraw(cx, &state.msg);
        }
    }
    pub fn update(&mut self) {
        let mut state = APP_STATE.lock().unwrap();
        self.ginput(id!(accsee_key_input)).borrow().map(|x| {
            state.accsee_key = x.text.to_string();
        });
        self.ginput(id!(secret_key_input)).borrow().map(|x| {
            state.secret_key = x.text.to_string();
        });
        self.gselect(id!(region_select)).borrow().map(|x| {
            state.region = x.options[x.selected].value.to_string();
        });
        self.gradio_group(id!(output_group)).borrow().map(|x| {
            state.output = ["json", "yaml", "text"][x.selected as usize].to_string();
        });
    }
}
