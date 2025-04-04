use gen_components::{
    *,
    utils::lifetime::{Executor, Lifetime},
};
use makepad_widgets::*;

use super::{bucket_page::BucketPageEvent, upload_page::UploadPageWidgetExt};

live_design! {
    // import makepad_widgets::base::*;
    // import makepad_widgets::theme_desktop_dark::*;
    // import gen_components::components::*;
    // import crate::views::start_page::*;
    // import crate::views::sigin_page::*;
    // import crate::views::settings_page::*;
    // import crate::views::bucket_page::*;
    // import crate::views::upload_page::*;
    use link::widgets::*;
    use link::gen_components::*;
    use link::shaders::*;
    use crate::views::start_page::*;
    use crate::views::sigin_page::*;
    use crate::views::settings_page::*;
    use crate::views::bucket_page::*;
    use crate::views::upload_page::*;

    pub AppMainPage = {{AppMainPage}}{
        height: Fill,
        width: Fill,
        flow: Down,
        background_visible: false,
        border_radius: 0.0,
        app_router = <GRouter>{
            bar_pages = {
                flow: Down,
                bucket_frame = <GBarPage>{
                    page = <BucketPage>{}
                }
                upload_frame = <GBarPage>{
                    upload_page = <UploadPage>{}
                }
                setting_frame = <GBarPage>{
                    <SettingsPage> {}
                }
                tabbar = <GTabbar>{
                    background_color: #191111,
                    tab1 = <GTabbarItem>{
                        width: Fill
                        icon_slot: {
                            color: #FFF,
                            src: dep("crate://self/resources/home.svg"),
                        }
                        text_slot: {
                            color: #FFF,
                            font_family: (BOLD_FONT2),
                            text: "Home"
                        }
                    }
                    tab2 = <GTabbarItem>{
                        width: Fill
                        icon_slot: {
                            color: #FFF,
                            src: dep("crate://self/resources/upload.svg"),
                        }
                        text_slot: {
                            color: #FFF,
                            font_family: (BOLD_FONT2),
                            text: "Upload"
                        }
                    }
                    tab3 = <GTabbarItem>{
                        width: Fill
                        icon_slot: {
                            color: #FFF,
                            src: dep("crate://self/resources/setting_2.svg"),
                        }
                        text_slot: {
                            color: #FFF,
                            font_family: (BOLD_FONT2),
                            text: "Settings"
                        }
                    }
                }
            }
            nav_pages = {
                start_screen = <GNavPage>{
                    header = {
                        visible: false,
                    }
                    body = <StartPage>{}
                }
                sigin_screen = <GNavPage>{
                    header = {
                        title_wrap = {
                            title = {
                                font_family: (BOLD_FONT2),
                                text: "Settings"
                            }
                        }
                    }
                    body = <SiginPage>{}
                }
            }
        }
    }
}

#[derive(Live, Widget)]
pub struct AppMainPage {
    #[deref]
    pub deref_widget: GView,
    #[rust]
    lifetime: Lifetime,
}

impl LiveHook for AppMainPage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for AppMainPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);
        self.lifetime
            .init()
            .execute(|| {
                let router = self.grouter(id!(app_router));

                router.borrow_mut().map(|mut router| {
                    let _ = router
                        .init(
                            ids!(bucket_frame, upload_frame, setting_frame),
                            Some(ids!(start_screen, sigin_screen)),
                            None,
                        )
                        .active(id!(start_screen))
                        .nav_actions(|router, cx| {
                            // do when nav to
                            if router.eq_active_page(id!(bucket_frame)) {
                                cx.widget_action(
                                    router.widget_uid(),
                                    &Scope::empty().path,
                                    BucketPageEvent::Update,
                                );
                            }
                            if router.eq_active_page(id!(upload_frame)){
                                router.upload_page(id!(upload_page)).borrow_mut().map(|mut page|{
                                    page.lifetime = Lifetime::Init;
                                });
                            }
                        })
                        .build(cx);
                });
            })
            .map(|_| {
                let router = self.grouter(id!(app_router));
                router.borrow().map(|router| {
                    if router.scope_path.is_some() {
                        // if is empty do not do next
                        self.lifetime.next();
                    }
                })
            });
        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));

        self.grouter(id!(app_router)).borrow_mut().map(|mut route| {
            route.handle_nav_events(cx, &actions);
        });
    }
}
