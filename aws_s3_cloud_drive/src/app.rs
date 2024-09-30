use makepad_widgets::*;

use crate::utils::APP_STATE;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import gen_components::components::*;
    import crate::views::start_page::*;
    import crate::views::sigin_page::*;
    import crate::views::settings_page::*;
    // import crate::views::main_page::*;
    import crate::views::bucket_page::*;
    import crate::views::upload_page::*;

    BOLD_FONT = dep("crate://self/resources/JuliaMono-BlackItalic.ttf");
    AppTab = <RadioButton> {
        height: Fill
        width: Fill
        flow: Down
        spacing: 5.0
        align: {x: 0.5, y: 0.5}

        icon_walk: {margin: { left: 0. }, width: 20, height: 20}
        label_walk: {
            width: Fit, height: Fit,
            margin: { left: 0. }
        }
        label_align: { y: 0.0 }

        draw_radio: {
            radio_type: Tab,
            fn pixel(self) -> vec4 {
                return vec4(0.0);
            }
        }
        draw_text: {
            // text_style: <APP_NAVIGATION_FONT> {}
            color_selected: #FF7043,
            color_unselected: #000,
            color_unselected_hover: #111,
            text_style: {
                font_size: 8.0,
                font: {
                    path: (BOLD_FONT)
                }
            }

            fn get_color(self) -> vec4 {
                return mix(
                    mix(
                        self.color_unselected,
                        self.color_unselected_hover,
                        self.hover
                    ),
                    self.color_selected,
                    self.selected
                )
            }
        }
        draw_icon: {
            fn get_color(self) -> vec4 {
                return mix(
                    #000,
                    #FF7043,
                    self.selected
                )
            }
        }
    }
    App = {{App}}{
        root: <Root>{
            main_window = <GWindow>{
                os_type: Linux,
                show_icon: false,
                window_bar = {
                    window_title = {
                        align: {x: 0.5},
                    }
                },
                width: Fill,
                height: Fill,
                window: {inner_size: vec2(420, 820)},
                background_color: #161616,
                background_visible: true,
                clip_x: true,
                clip_y: true,
                body = <GVLayout>{
                    // <StartPage>{}
                    // height: Fill,
                    // <SettingsPage>{}
                    // margin: {
                    //     top: 32.0
                    // }
                    // <SiginPage>{}
                    // <MainPage>{}
                    // <BucketPage>{}
                    // <UploadPage>{}
                    navigation = <StackNavigation>{
                        root_view = {
                            width: Fill
                            height: Fill
                            padding: 0
                            flow: Down
                            align: {x: 0.0, y: 0.0}
                            spacing: 0
                            application_pages = <View> {
                                margin: 0.0,
                                padding: 0.0
                                flow: Overlay
                                bucket_frame = <BucketPage>{visible: true}
                                upload_frame = <UploadPage>{visible: false}
                                setting_frame = <SettingsPage> {visible: false}
                            }
                            menu = <GView>{
                                border_radius: 0.0,
                                theme: Dark,
                                height: 46.0,
                                width: Fill,
                                modes = <GHLayout>{
                                    height: Fill,
                                    width: Fill,
                                    align: {x: 0.5, y: 0.5},
                                    spacing: 16.0,
                                    tab1 = <AppTab>{
                                        animator: {selected = {default: on}}
                                        text: "Home"
                                        draw_icon: {
                                            svg_file: dep("crate://self/resources/home.svg"),
                                        }
                                    }
                                    tab2 = <AppTab>{
                                        animator: {selected = {default: off}}
                                        text: "Upload"
                                        draw_icon: {
                                            svg_file: dep("crate://self/resources/upload.svg"),
                                        }
                                    }
                                    tab3 = <AppTab>{
                                        animator: {selected = {default: off}}
                                        text: "Settings"
                                        draw_icon: {
                                            svg_file: dep("crate://self/resources/setting.svg"),
                                        }
                                    }
                                }
                            }
                        }

                        start_page = <StackNavigationView> {
                            height: Fill,
                            draw_bg: {
                                color: #F5F5F500,
                            }
                            header = <View>{
                                height: 0.0,
                                visible: false,
                            }
                            body = {
                                margin: {top: 32.0},
                                start_screen = <StartPage>{}
                            }
                        }
                        sigin_page_view = <StackNavigationView> {
                            draw_bg: {
                                color: #F5F5F500,
                            }
                            header = <View>{
                                height: 0.0,
                                visible: false,
                            }
                            body = {
                                margin: {top: 32.0},
                                sigin_screen = <SiginPage>{

                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Live)]
pub struct App {
    #[live]
    root: WidgetRef,
    #[rust]
    pub timer: Timer,
}

impl LiveHook for App {
    fn after_apply(
        &mut self,
        _cx: &mut Cx,
        _apply: &mut Apply,
        _index: usize,
        _nodes: &[LiveNode],
    ) {
        // get configs
        let mut state = APP_STATE.lock().unwrap();
        let _ = state.get_confih_credentials();
        let _ = state.ls();
    }
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        crate::makepad_widgets::live_design(cx);
        crate::gen_components::live_design(cx);
        crate::views::start_page::live_design(cx);
        crate::views::sigin_page::live_design(cx);
        crate::views::settings_page::live_design(cx);
        crate::views::bucket_page::live_design(cx);
        crate::views::upload_page::live_design(cx);
    }
}

impl MatchEvent for App {
    fn handle_timer(&mut self, cx: &mut Cx, _e: &TimerEvent) {
        let uid = self.root.widget_uid();
        cx.widget_action(
            uid,
            &Scope::empty().path,
            StackNavigationAction::NavigateTo(live_id!(root_view)),
        );

        cx.stop_timer(self.timer);
    }
    fn handle_startup(&mut self, cx: &mut Cx) {
        dbg!("start up");
        self.timer = cx.start_timeout(10.0);
        let uid = self.root.widget_uid();
        cx.widget_action(
            uid,
            &Scope::empty().path,
            StackNavigationAction::NavigateTo(live_id!(start_page)),
        );
    }
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        self.root
            .radio_button_set(ids!(modes.tab1, modes.tab2, modes.tab3))
            .selected_to_visible(
                cx,
                &self.root,
                &actions,
                ids!(
                    application_pages.bucket_frame,
                    application_pages.upload_frame,
                    application_pages.setting_frame
                ),
            );

        let mut navigation = self.root.stack_navigation(id!(navigation));
        navigation.handle_stack_view_actions(cx, &actions);
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.root.handle_event(cx, event, &mut Scope::empty());
    }
}

app_main!(App);
