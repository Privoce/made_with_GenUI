use makepad_widgets::*;

// use crate::{
//     commands::ls,
//     components::{main_view::MainViewWidgetRefExt, personal_view::PersonalWidgetRefExt},
//     state::State,
// };

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import gen_components::components::*;
    import crate::views::start_page::*;
    import crate::views::sigin_page::*;

    BOLD_FONT = dep("crate://self/resources/JuliaMono-BlackItalic.ttf");

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
                background_color: #F5F5F5,
                background_visible: false,
                clip_x: true,
                clip_y: true,
                body = <GVLayout>{
                    // <StartPage>{}
                    height: Fill,
                    // <SiginPage>{}
                    navigation = <StackNavigation>{
                        start_page = <StackNavigationView> {
                            height: Fill,
                            draw_bg: {
                                color: #F5F5F5,
                            }
                            header = <View>{
                                height: 0.0,
                                visible: false,
                            }
                            body = {
                                margin: 0.0,
                                start_screen = <StartPage>{}
                            }
                        }
                        sigin_page_view = <StackNavigationView> {
                            header = <View>{
                                height: 0.0,
                                visible: false,
                            }
                            body = {
                                margin: 0.0,
                                sigin_screen = <SiginPage>{
                                    
                                }
                            }
                        }
                    }
                    // navigation = <StackNavigation>{
                    //     root_view = {
                    //         width: Fill,
                    //         height: Fill,
                    //         application_pages = <View> {
                    //             margin: 0.0,
                    //             padding: 0.0

                    //             tab1_frame = <StartPage>{visible: true}
                    //         }
                    //     }
                    // }
                    // mobile_menu = <RoundedView> {
                    //     width: Fill,
                    //     height: 60.0,
                    //     mobile_modes = <View> {

                    //     }
                    // }
                    // height: All,
                    // width: All,
                    // <HeaderView>{}
                    // <ToolsView>{}
                    // <BodyView>{}
                    // <GHLayout>{
                    //     height: Fill,
                    //     width: Fill,
                    //     spacing: 2.0,
                    //     personal = <Personal>{
                    //         height: Fill,
                    //         width: 280.0,
                    //         background_color: #394049,
                    //         border_radius: 0.0,
                    //     }
                    //     main_view = <MainView>{
                    //         height: Fill,
                    //         width: Fill,
                    //         background_color: #ADBAC7,
                    //         border_radius: 0.0,
                    //     }
                    // }
                    // footer = <GCard>{
                    //     theme: Dark,
                    //     height: 160.0,
                    //     width: Fill,
                    //     flow: Down,
                    //     spacing: 0.0,
                    //     border_radius: 0.0,
                    //     <GHLayout>{
                    //         height: 42.0,
                    //         width: Fill,
                    //         align: {x: 0.0, y: 0.5},
                    //         padding: {left: 6.0, right: 6.0},
                    //         <GLabel>{
                    //             font_size: 10.0,
                    //             text: "AWS Terminal Message",
                    //         }
                    //         <GHLayout>{
                    //             height: Fill,
                    //             width: Fill,
                    //             align: {x: 1.0, y: 0.5},
                    //             spacing: 6.0,
                    //             fresh_btn = <GButton>{
                    //                 slot: <GHLayout>{
                    //                     height: Fit,
                    //                     width: Fill,
                    //                     spacing: 4.0,
                    //                     align: {x: 0.5, y: 0.0},
                    //                     <GIcon>{
                    //                         theme: Dark,
                    //                         height: 14.0,
                    //                         width: 14.0,
                    //                         icon_type: Delete,
                    //                         stroke_width: 1.0,
                    //                         animation_open: true,
                    //                     }
                    //                     <GLabel>{
                    //                         text: "ReCheck Toolkit",
                    //                     }
                    //                 }
                    //             }
                    //             delete_btn = <GButton>{
                    //                 theme: Error,
                    //                 slot: <GHLayout>{
                    //                     height: Fit,
                    //                     width: Fill,
                    //                     spacing: 4.0,
                    //                     align: {x: 0.5, y: 0.0},
                    //                     <GIcon>{
                    //                         theme: Dark,
                    //                         height: 14.0,
                    //                         width: 14.0,
                    //                         icon_type: Delete,
                    //                         stroke_width: 1.0,
                    //                         animation_open: true,
                    //                     }
                    //                     <GLabel>{
                    //                         text: "Delete Message",
                    //                     }
                    //                 }
                    //             }
                    //         }

                    //     }
                    //     msg_output = <GInput>{
                    //         theme: Dark,
                    //         border_radius: 0.0,
                    //         width: Fill,
                    //         text: "This is Message Output",
                    //         read_only: true,
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
    root: WidgetRef,
    // #[rust]
    // pub state: State,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        crate::makepad_widgets::live_design(cx);
        crate::gen_components::live_design(cx);
        crate::views::start_page::live_design(cx);
        crate::views::sigin_page::live_design(cx);
        // crate::components::personal_view::live_design(cx);
        // crate::components::main_view::live_design(cx);
        // crate::components::header::live_design(cx);
        // crate::components::tools::live_design(cx);
        // crate::components::body::live_design(cx);
    }
}

impl MatchEvent for App {
    fn handle_startup(&mut self, cx: &mut Cx) {
        log!("App started!");
        let uid = self.root.widget_uid();
        cx.widget_action(
            uid,
            &Scope::empty().path,
            StackNavigationAction::NavigateTo(live_id!(start_page)),
        );

        // // check if the toolkit is available
        // let _ = self.state.check_toolkit();
        // let ginput = self.root.ginput(id!(msg_output));
        // ginput.set_text_and_redraw(cx, self.state.msg.as_str());

        // // check credentials and config
        // self.state.check_config_credentials();
        // let _ = self
        //     .root
        //     .personal(id!(personal))
        //     .borrow_mut()
        //     .map(|mut personal| {
        //         personal.set_state(cx, self.state.clone());
        //     });
        // ginput.set_text_and_redraw(cx, self.state.msg.as_str());
        // // then ls the bucket
        // // dbg!("ls result");
        // match ls(&self.state.bucket) {
        //     Ok(res) => {
        //         let main_view = self.root.main_view(id!(main_view));

        //         main_view.borrow_mut().map(|mut v| {
        //             v.set_bucket(cx, &self.state.bucket, Some(res));
        //         });
        //     }
        //     Err(e) => {
        //         ginput.set_text_and_redraw(cx, &e);
        //     }
        // }
    }
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        let mut navigation = self.root.stack_navigation(id!(navigation));

        navigation.handle_stack_view_actions(cx, &actions);
        // let fresh_btn = self.root.gbutton(id!(fresh_btn));
        // let delete_btn = self.root.gbutton(id!(delete_btn));
        // let ginput = self.root.ginput(id!(msg_output));

        // if fresh_btn.clicked(actions).is_some() {
        //     self.state.msg.clear();
        //     self.state.check_toolkit();
        //     ginput.set_text_and_redraw(cx, self.state.msg.as_str());
        // }
        // if delete_btn.clicked(actions).is_some() {
        //     self.state.msg.clear();
        //     ginput.set_text_and_redraw(cx, self.state.msg.as_str());
        // }
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.root.handle_event(cx, event, &mut Scope::empty());
    }
}

app_main!(App);
