use gen_components::components::{
    button::GButtonWidgetExt, card::GCard, file_upload::GUploadWidgetExt,
};
use makepad_widgets::*;

use crate::utils::set_conf;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import gen_components::components::*;

    BOLD_FONT = dep("crate://self/resources/JuliaMono-BlackItalic.ttf");
    BOLD_FONT2 = dep("crate://self/resources/FiraCode-Bold.ttf");
    BucketPage = {{BucketPage}}{
        height: Fill,
        width: Fill,
        flow: Down,
        border_radius: 0.0,
        background_color: #161616,
        <GHLayout>{
            height: Fit,
            width: Fill,
            spacing: 16.0,
            align: {y:0.5},
            padding: 12.0,
            <GLabel>{
                width: Fill,
                font_size: 14.0,
                font_family: (BOLD_FONT),
                text: "HOME",
            }
            <GIcon>{
                cursor: Hand,
                theme: Dark,
                icon_type: Upload,
                height: 16.0,
                width: 16.0
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
            // quick_s3_btn = <GButton>{
            //     theme: Dark,
            //     height: 32.0,
            //     slot: {
            //         font_size: 8.0,
            //         font_family: (BOLD_FONT2),
            //         text: "Set Quick S3 Bucket"
            //     }
            // }
            u_d_set_btn = <GHLayout>{
                height: 32.0,
                width: Fit,
                align: {
                    y: 0.5
                },
                spacing: 4.0,
                <GCard>{
                    height: 32.0,
                    width: 32.0,
                    align: {x: 0.5, y: 0.5},
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
        <GCard>{
            background_color: #1E1E1E,
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
                        color: #3B82F6,
                        font_family: (BOLD_FONT2),
                        text: "Get All"
                    }
                }
            }
            <GVLayout>{
                align: {
                    x: 0.0,
                    y: 0.0
                },
                padding: 8.0,
                spacing: 12.0,
                <GLink>{
                    font_size: 8.0,
                    font_family: (BOLD_FONT),
                    text: "IMG_20240905_225067.png 4.5MB",
                    href: "https://docs.aws.amazon.com/s3/?icmpid=docs_homepage_featuredsvcs"
                }
                <GLink>{
                    font_size: 8.0,
                    font_family: (BOLD_FONT),
                    text: "Video_0_20240912.mp4 864KB",
                    href: "https://docs.aws.amazon.com/pdfs/AmazonS3/latest/userguide/s3-userguide.pdf"
                }
                <GLink>{
                    font_size: 8.0,
                    font_family: (BOLD_FONT),
                    text: "IMG_20240916_225067.png 10.2MB",
                    href: "https://docs.aws.amazon.com/zh_cn/general/latest/gr/Welcome.html"
                }
            }
        }
        <GCard>{
            background_color: #1E1E1E,
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
                        color: #3B82F6,
                        text: "Get All"
                    }
                }
            }
            <GVLayout>{
                height: Fit,
                spacing: 6.0,
                <GLabel>{
                    font_family: (BOLD_FONT),
                    font_size: 9.0,
                    text: "Amazon Simple Storage Service Documentation",
                }
                <GLabel>{
                    color: #D5DBDB,
                    width: Fill,
                    font_family: (BOLD_FONT),
                    font_size: 8.0,
                    text: "Amazon Simple Storage Service (Amazon S3) is storage for the internet",
                    margin: {left: 4.0}
                }
                <GLabel>{
                    color: #D5DBDB,
                    width: Fill,
                    font_family: (BOLD_FONT),
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
                    font_size: 8.0,
                    font_family: (BOLD_FONT),
                    text: "AWS S3 Documentation",
                    href: "https://docs.aws.amazon.com/s3/?icmpid=docs_homepage_featuredsvcs"
                }
                <GLink>{
                    font_size: 8.0,
                    font_family: (BOLD_FONT),
                    text: "User Guide Documentation (PDF)",
                    href: "https://docs.aws.amazon.com/pdfs/AmazonS3/latest/userguide/s3-userguide.pdf"
                }
                <GLink>{
                    font_size: 8.0,
                    font_family: (BOLD_FONT),
                    text: "AWS Cli Documentation",
                    href: "https://docs.aws.amazon.com/zh_cn/general/latest/gr/Welcome.html"
                }
            }
        }
        <GHLayout>{
            align: {x: 0.5, y: 0.2}
            to_main = <GButton>{
                slot: {
                    font_size: 10.0,
                    font_family: (BOLD_FONT),
                    text: "Let's Upload!"
                }
            }
        }
    }
}

#[derive(Live, Widget)]
pub struct BucketPage {
    #[deref]
    pub super_widget: GCard,
}

impl LiveHook for BucketPage {}

impl Widget for BucketPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.super_widget.draw_walk(cx, scope, walk)
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.super_widget.handle_event(cx, event, scope));
        self.to_main_page(cx, scope, &actions).map(|_| {
            return;
        });
        self.set_u_d_dir(&actions).map(|_| {
            return;
        });
    }
}

impl BucketPage {
    pub fn to_main_page(
        &mut self,
        cx: &mut Cx,
        scope: &mut Scope,
        actions: &Actions,
    ) -> Option<()> {
        self.gbutton(id!(to_main)).borrow().map(|x| {
            if x.clicked(actions).is_some() {
                cx.widget_action(
                    self.widget_uid(),
                    &scope.path,
                    StackNavigationAction::NavigateTo(live_id!(application_pages.main_frame)),
                );
            }
        })
    }
    pub fn set_u_d_dir(&mut self, actions: &Actions) -> Option<()> {
        self.gupload(id!(up)).borrow().map(|up_btn| {
            if let Some(dirs) = up_btn.after_select(actions) {
                // set conf
                let _ = set_conf(dirs[0].to_str().unwrap());
            }
        })
    }
    // pub fn set_s3_bucket(&mut self,actions: &Actions,)->Option<()>{
    //     self.gbutton(id!(quick_s3_btn)).borrow().map(|x|{
    //         if x.clicked(actions).is_some(){

    //         }
    //     })
    // }
}
