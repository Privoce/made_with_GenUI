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
                background_color: #161616,
                background_visible: true,
                clip_x: true,
                clip_y: true,
                body = <GVLayout>{
                    // <StartPage>{}
                    height: Fill,
                    // margin: {
                    //     top: 32.0
                    // }
                    // <SiginPage>{}
                    navigation = <StackNavigation>{
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

#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    root: WidgetRef,
    #[rust]
    pub timer: Timer,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        crate::makepad_widgets::live_design(cx);
        crate::gen_components::live_design(cx);
        crate::views::start_page::live_design(cx);
        crate::views::sigin_page::live_design(cx);

    }
}

impl MatchEvent for App {
    fn handle_timer(&mut self, cx: &mut Cx, _e:&TimerEvent) {
        let uid = self.root.widget_uid();
        cx.widget_action(
            uid,
            &Scope::empty().path,
            StackNavigationAction::NavigateTo(live_id!(sigin_page_view)),
        );
        cx.stop_timer(self.timer);
    }
    fn handle_startup(&mut self, cx: &mut Cx) {
        self.timer = cx.start_timeout(12.0);
        let uid = self.root.widget_uid();
        cx.widget_action(
            uid,
            &Scope::empty().path,
            StackNavigationAction::NavigateTo(live_id!(start_page)),
        );
    }
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
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
