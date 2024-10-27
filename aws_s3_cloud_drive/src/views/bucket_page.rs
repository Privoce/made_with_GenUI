use gen_components::{
    components::{
        button::GButtonWidgetExt,
        file_upload::GUploadWidgetExt,
        link::GLinkWidgetRefExt,
        router::GRouter,
        view::{GView, GViewWidgetExt},
    },
    utils::lifetime::{Executor, Lifetime},
};
use makepad_widgets::*;

use crate::utils::{State, APP_STATE};

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import gen_components::components::*;

    BOLD_FONT2 = dep("crate://self/resources/FiraCode-Bold.ttf");
    BucketPage = {{BucketPage}}{
        height: Fill,
        width: Fill,
        flow: Down,
        border_radius: 0.0,
        background_color: #1F1616,
        link_template: <GLink>{
            color: #ED4A26,
            underline_color: #ED4A26,
            underline_hover_color: #F56241,
            underline_focus_color: #F56241,
            text_hover_color: #F56241,
            text_focus_color: #F56241,
            font_size: 8.0,
            font_family: (BOLD_FONT2),
            text: "",
            href: "",
        }
        <GHLayout>{
            height: Fit,
            width: Fill,
            spacing: 16.0,
            align: {y:0.5},
            padding: 12.0,
            <GLabel>{
                width: Fill,
                font_size: 14.0,
                font_family: (BOLD_FONT2),
                text: "HOME",
            }
        }
        <GDivider>{
            theme: Dark,
            height: 4.0,
        }
        <GHLayout>{
            background_color: #1E1E1E,
            height: 60.0,
            width: Fill,
            border_radius: 0.0,
            padding: 8.0,
            align: {
                y: 0.5
            },
            spacing: 8.0,
            u_d_set_btn = <GHLayout>{
                height: 32.0,
                width: Fit,
                align: {
                    y: 0.5
                },
                spacing: 4.0,
                <GView>{
                    height: 32.0,
                    width: 32.0,
                    align: {x: 0.5, y: 0.5},
                    background_color: #ED4A26,
                    up = <GUpload>{
                        height: 16.0,
                        width: 16.0,
                        mode: Folder,
                        clear: true,
                        icon: {
                            theme: Warning,
                            height: 16.0,
                            width: 16.0
                        }
                    }
                }
                <GLabel>{
                    font_size: 10.0,
                    font_family: (BOLD_FONT2),
                    text: "Set Download Folder"
                }
            }
        }
        <GView>{
            background_color: #31171B,
            height: 220.0,
            width: Fill,
            border_radius: 0.0,
            padding: 12.0,
            flow: Down,
            margin: {
                bottom: 12.0
            },
            <GHLayout>{
                height: 16.0,
                width: Fill,
                spacing: 8.0,
                align: {
                    y: 0.5,
                },
                <GImage>{
                    height: 16.0,
                    width: 16.0,
                    src: dep("crate://self/resources/share.png")
                }
                <GLabel>{
                    width: Fill,
                    font_family: (BOLD_FONT2),
                    text: "Shared Links"
                }
                <GButton>{
                    padding: 0.0,
                    height: 16.0,
                    spread_radius: 0.0,
                    background_visible: false,
                    slot: {
                        color: #ED4A26,
                        font_family: (BOLD_FONT2),
                        text: "Get All"
                    }
                }
            }
            share_link_wrap = <GVLayout>{
                align: {
                    x: 0.0,
                    y: 0.0
                },
                padding: 8.0,
                spacing: 12.0,
            }
        }
        <GView>{
            background_color: #31171B,
            height: 220.0,
            width: Fill,
            border_radius: 0.0,
            padding: 12.0,
            margin: {
                bottom: 12.0
            },
            flow: Down,
            spacing: 8.0,
            <GHLayout>{
                margin: {
                    top: 4.0,
                    bottom: 4.0,
                },
                height: 16.0,
                width: Fill,
                spacing: 8.0,
                align: {
                    y: 0.5,
                },
                <GImage>{
                    height: 16.0,
                    width: 16.0,
                    src: dep("crate://self/resources/doc.png")
                }
                <GLabel>{
                    width: Fill,
                    font_family: (BOLD_FONT2),
                    text: "AWS Docs"
                }
                <GButton>{
                    padding: 0.0,
                    height: 16.0,
                    spread_radius: 0.0,
                    background_visible: false,
                    slot: {
                        font_family: (BOLD_FONT2),
                        color: #ED4A26,
                        text: "Get All"
                    }
                }
            }
            <GVLayout>{
                height: Fit,
                spacing: 6.0,
                <GLabel>{
                    font_family: (BOLD_FONT2),
                    font_size: 9.0,
                    text: "Amazon Simple Storage Service Documentation",
                }
                <GLabel>{
                    color: #D5DBDB,
                    width: Fill,
                    font_family: (BOLD_FONT2),
                    font_size: 8.0,
                    text: "Amazon Simple Storage Service (Amazon S3) is storage for the internet",
                    margin: {left: 4.0}
                }
                <GLabel>{
                    color: #D5DBDB,
                    width: Fill,
                    font_family: (BOLD_FONT2),
                    font_size: 8.0,
                    margin: {left: 4.0}
                    text: "Amazon S3 to store and retrieve any amount of data at any time, from anywhere on the web.",
                }
            }
            <GVLayout>{
                align: {
                    x: 0.0,
                    y: 0.5
                },
                padding: {
                    left: 8.0,
                },
                spacing: 12.0,
                <GLink>{
                    color: #ED4A26,
                    underline_color: #ED4A26,
                    underline_hover_color: #F56241,
                    underline_focus_color: #F56241,
                    text_hover_color: #F56241,
                    text_focus_color: #F56241,
                    font_size: 8.0,
                    font_family: (BOLD_FONT2),
                    text: "AWS S3 Documentation",
                    href: "https://docs.aws.amazon.com/s3/?icmpid=docs_homepage_featuredsvcs"
                }
                <GLink>{
                    color: #ED4A26,
                    underline_color: #ED4A26,
                    underline_hover_color: #F56241,
                    underline_focus_color: #F56241,
                    text_hover_color: #F56241,
                    text_focus_color: #F56241,
                    font_size: 8.0,
                    font_family: (BOLD_FONT2),
                    text: "User Guide Documentation (PDF)",
                    href: "https://docs.aws.amazon.com/pdfs/AmazonS3/latest/userguide/s3-userguide.pdf"
                }
                <GLink>{
                    color: #ED4A26,
                    underline_color: #ED4A26,
                    underline_hover_color: #F56241,
                    underline_focus_color: #F56241,
                    text_hover_color: #F56241,
                    text_focus_color: #F56241,
                    font_size: 8.0,
                    font_family: (BOLD_FONT2),
                    text: "AWS Cli Documentation",
                    href: "https://docs.aws.amazon.com/zh_cn/general/latest/gr/Welcome.html"
                }
            }
        }
        <GHLayout>{
            align: {x: 0.5, y: 0.2}
            to_main = <GButton>{
                theme: Warning,
                background_color: #EC4925,
                hover_color: #Fa4319,
                focus_color: #E36741,
                slot: {
                    font_size: 10.0,
                    font_family: (BOLD_FONT2),
                    text: "Let's Upload!"
                }
            }
        }
    }
}

#[derive(Live, Widget)]
pub struct BucketPage {
    #[deref]
    pub super_widget: GView,
    #[live]
    pub link_template: Option<LivePtr>,
    #[rust]
    lifetime: Lifetime,
}

impl LiveHook for BucketPage {}

impl Widget for BucketPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if self.visible {
            let _ = self.super_widget.draw_walk(cx, scope, walk);

            self.lifetime.init().execute(|| self.init(cx)).map(|_| {
                self.lifetime.next();
            });
        } else {
            self.lifetime = Lifetime::Init;
        }
        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.lifetime
            .in_process()
            .execute(|| {
                if let Event::Actions(actions) = event {
                    for action in actions {
                        if let Some(action) = action.as_widget_action() {
                            if let BucketPageEvent::Update = action.cast() {
                                self.init(cx);
                            }
                        }
                    }
                }

                let actions =
                    cx.capture_actions(|cx| self.super_widget.handle_event(cx, event, scope));
                self.to_main_page(cx, scope, &actions).map(|_| {
                    return;
                });
                self.set_u_d_dir(&actions).map(|_| {
                    return;
                });
            })
            .map(|_| {
                return;
            });
    }
    fn is_visible(&self) -> bool {
        self.visible
    }
}

impl BucketPage {
    pub fn init(&mut self, cx: &mut Cx) {
        self.gview(id!(share_link_wrap))
            .borrow_mut()
            .map(|mut wrap| {
                let state = APP_STATE.lock().unwrap();
                state.shares.as_ref().map(|shares| {
                    let mut children = vec![];
                    for (index, share) in shares.iter().enumerate() {
                        children.push((
                            LiveId(index as u64),
                            WidgetRef::new_from_ptr(cx, self.link_template),
                        ));
                        children.last_mut().map(|(_, child)| {
                            child.as_glink().borrow_mut().map(|mut link| {
                                link.set_text_and_redraw(cx, &share.name);
                                link.href.replace(share.url.to_string());
                            });
                        });
                    }
                    wrap.children = children;
                    wrap.redraw(cx);
                });
            });
    }
    pub fn redraw(&mut self, cx: &mut Cx) {
        self.super_widget.redraw(cx);
    }
    pub fn to_main_page(
        &mut self,
        cx: &mut Cx,
        scope: &mut Scope,
        actions: &Actions,
    ) -> Option<()> {
        self.gbutton(id!(to_main)).borrow().map(|x| {
            if x.clicked(actions).is_some() {
                GRouter::nav_to_path(cx, self.widget_uid(), scope, id!(upload_frame));
            }
        })
    }
    pub fn set_u_d_dir(&mut self, actions: &Actions) -> Option<()> {
        self.gupload(id!(up)).borrow().map(|up_btn| {
            if let Some(e) = up_btn.selected(actions) {
                // set conf
                if !e.paths.is_empty() {
                    let mut state = APP_STATE.lock().unwrap();
                    state.download_dir.replace(e.paths[0].clone());
                    let _ = State::sync_download_conf(false);
                }
            }
        })
    }
}

#[derive(DefaultNone, Debug, Clone)]
pub enum BucketPageEvent {
    Update,
    None,
}
