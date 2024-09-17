use gen_components::components::{
    button::GButtonWidgetRefExt, file_upload::GUploadWidgetRefExt, input::GInputWidgetRefExt,
};
use makepad_widgets::*;

use crate::{
    commands::ls,
    components::{main_view::MainViewWidgetRefExt, personal_view::PersonalWidgetRefExt},
    state::State,
};

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import gen_components::components::*;
    import crate::components::personal_view::*;
    import crate::components::main_view::*;

    App = {{App}}{
        root: <Root>{
            main_window = <Window>{
                show_bg: true,
                width: Fill,
                height: Fill,
                draw_bg: {color: #22272f},
                window: {inner_size: vec2(800, 600)},
                body = <ScrollYView>{
                    height: All,
                    width: All,
                    flow: Down,
                    <GHLayout>{
                        height: Fill,
                        width: Fill,
                        spacing: 2.0,
                        personal = <Personal>{
                            height: Fill,
                            width: 280.0,
                            background_color: #394049,
                            border_radius: 0.0,
                        }
                        main_view = <MainView>{
                            height: Fill,
                            width: Fill,
                            background_color: #ADBAC7,
                            border_radius: 0.0,
                        }
                    }
                    footer = <GCard>{
                        theme: Dark,
                        height: 160.0,
                        width: Fill,
                        flow: Down,
                        spacing: 0.0,
                        border_radius: 0.0,
                        <GHLayout>{
                            height: 42.0,
                            width: Fill,
                            align: {x: 0.0, y: 0.5},
                            padding: {left: 6.0, right: 6.0},
                            <GLabel>{
                                font_size: 10.0,
                                text: "AWS Terminal Message",
                            }
                            <GHLayout>{
                                height: Fill,
                                width: Fill,
                                align: {x: 1.0, y: 0.5},
                                spacing: 6.0,
                                fresh_btn = <GButton>{
                                    slot: <GHLayout>{
                                        height: Fit,
                                        width: Fill,
                                        spacing: 4.0,
                                        align: {x: 0.5, y: 0.0},
                                        <GIcon>{
                                            theme: Dark,
                                            height: 14.0,
                                            width: 14.0,
                                            icon_type: Delete,
                                            stroke_width: 1.0,
                                            animation_open: true,
                                        }
                                        <GLabel>{
                                            text: "ReCheck Toolkit",
                                        }
                                    }
                                }
                                delete_btn = <GButton>{
                                    theme: Error,
                                    slot: <GHLayout>{
                                        height: Fit,
                                        width: Fill,
                                        spacing: 4.0,
                                        align: {x: 0.5, y: 0.0},
                                        <GIcon>{
                                            theme: Dark,
                                            height: 14.0,
                                            width: 14.0,
                                            icon_type: Delete,
                                            stroke_width: 1.0,
                                            animation_open: true,
                                        }
                                        <GLabel>{
                                            text: "Delete Message",
                                        }
                                    }
                                }
                            }

                        }
                        msg_output = <GInput>{
                            theme: Dark,
                            border_radius: 0.0,
                            width: Fill,
                            text: "This is Message Output",
                        }
                    }
                }
            }
        }
    }
}

#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    root: WidgetRef,
    #[rust]
    pub state: State,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        crate::makepad_widgets::live_design(cx);
        crate::gen_components::live_design(cx);
        crate::components::personal_view::live_design(cx);
        crate::components::main_view::live_design(cx);
    }
}

impl MatchEvent for App {
    fn handle_startup(&mut self, cx: &mut Cx) {
        log!("App started!");
        // check if the toolkit is available
        let _ = self.state.check_toolkit();
        let ginput = self.root.ginput(id!(msg_output));
        ginput.set_text_and_redraw(cx, self.state.msg.as_str());

        // check credentials and config
        self.state.check_config_credentials();
        let _ = self
            .root
            .personal(id!(personal))
            .borrow_mut()
            .map(|mut personal| {
                personal.set_state(cx, self.state.clone());
            });
        ginput.set_text_and_redraw(cx, self.state.msg.as_str());
        // then ls the bucket
        // dbg!("ls result");
        match ls(&self.state.bucket) {
            Ok(res) => {
                let main_view = self.root.main_view(id!(main_view));

                main_view.borrow_mut().map(|mut v| {
                    v.set_bucket(cx, &self.state.bucket, Some(res));
                });
            }
            Err(e) => {
                ginput.set_text_and_redraw(cx, &e);
            }
        }
    }
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        let fresh_btn = self.root.gbutton(id!(fresh_btn));
        let delete_btn = self.root.gbutton(id!(delete_btn));
        let ginput = self.root.ginput(id!(msg_output));

        if fresh_btn.clicked(actions).is_some() {
            self.state.msg.clear();
            self.state.check_toolkit();
            ginput.set_text_and_redraw(cx, self.state.msg.as_str());
        }
        if delete_btn.clicked(actions).is_some() {
            self.state.msg.clear();
            ginput.set_text_and_redraw(cx, self.state.msg.as_str());
        }
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.root.handle_event(cx, event, &mut Scope::empty());
    }
}

app_main!(App);
