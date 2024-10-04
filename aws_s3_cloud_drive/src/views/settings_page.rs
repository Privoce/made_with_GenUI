use gen_components::components::{button::GButtonWidgetExt, router::GRouter, view::GView};
use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import gen_components::components::*;

    BOLD_FONT = dep("crate://self/resources/JuliaMono-BlackItalic.ttf");
    BOLD_FONT2 = dep("crate://self/resources/FiraCode-Bold.ttf");
    SettingsPage = {{SettingsPage}}{
        height: Fill,
        width: Fill,
        flow: Down,
        border_radius: 0.0,
        background_color: #161616,
        align: {
            x: 0.5,
            y: 0.0
        },
        spacing: 24.0,
        padding: {
            left: 16.0,
            right: 16.0,
            top: 24.0,
            bottom: 24.0
        }
        <GView>{
            theme: Dark,
            border_radius: 6.0,
            height: 180.0,
            width: Fill,
            spacing: 24.0,
            padding: 16.0
            flow: Down,
            <GHLayout>{
                height: Fit,
                width: Fill,
                align: {x: 0.0, y: 0.5},
                spacing: 16.0,
                <GImage>{
                    height: 36.0,
                    width: 36.0,
                    src: dep("crate://self/resources/aws.png"),
                }
                <GLabel>{
                    font_size: 12.0,
                    font_family: (BOLD_FONT),
                    text: "AWS S3 User",
                }

            }
            <GVLayout>{
                height: Fill,
                width: Fill,
                spacing: 12.0,
                <GLabel>{
                    font_size: 10.0,
                    font_family: (BOLD_FONT2),
                    text: "My S3 Bucket Size",
                }
                <GHLayout>{
                    height: Fit,
                    align: {x: 1.0, y: 0.5},
                    size_total = <GLabel>{
                        font_size: 8.0,
                        font_family: (BOLD_FONT2),
                        text: "621GB",
                        color: #E36741,
                    }
                }
                <GProgress>{
                    theme: Dark,
                    value: 0.4,
                    width: Fill,
                }
                <GLabel>{
                    font_size: 8.0,
                    font_family: (BOLD_FONT2),
                    text: "Availible: 1436GB",
                }
            }
        }
        <GHLayout>{
            height: Fit,
            width: Fill,
            spacing: 16.0,
            change_btn = <GButton>{
                height: 42.0,
                width: Fill,
                theme: Warning,
                slot: {
                    text: "Change Config",
                    font_size: 10.0,
                    font_family: (BOLD_FONT2),
                }
            }
            <GButton>{
                height: 42.0,
                width: Fill,
                theme: Dark,
                slot: <GHLayout>{
                    spacing: 6.0,
                    align: {x: 0.5, y: 0.5},
                    <GIcon>{
                        icon_type: Help,
                        theme: Dark,
                        height: 18.0,
                        width: 18.0,
                    }
                    <GLabel>{
                        text: "Help?",
                        font_size: 10.0,
                        font_family: (BOLD_FONT2),
                    }
                }
            }
        }

        <GVLayout>{
            height: Fit,
            spacing: 8.0,
            <GDivider>{
                theme: Dark,
                height: Fit,
                <GLabel>{
                    text: "Aws Doc",
                    font_size: 12.0,
                    font_family: (BOLD_FONT2),
                }
            }
            <GLink>{
                href: "https://docs.aws.amazon.com/zh_cn/general/latest/gr/Welcome.html",
                text: "AWS General Reference",
                font_size: 10.0,
                font_family: (BOLD_FONT2),
            }
            <GLabel>{
                width: Fill,
                font_family: (BOLD_FONT2),
                text: "The AWS General Reference provides AWS service endpoint and quota information for Amazon Web Services. Additionally, you can find links to other common topics.",
                font_size: 9.0,

            }
            <GVLayout>{
                height: Fit,
                padding: {
                    left: 8.0,
                    right: 8.0,
                }
                spacing: 8.0,
                <GLink>{
                    font_family: (BOLD_FONT2),
                    text: "1. AWS security credentials",
                    font_size: 9.0,
                    href: "https://docs.aws.amazon.com/general/latest/gr/Welcome.html#aws-ip-ranges"
                }
                <GLink>{
                    font_family: (BOLD_FONT2),
                    text: "2. AWS IP address ranges",
                    font_size: 9.0,
                    href: "https://docs.aws.amazon.com/general/latest/gr/Welcome.html#aws-ip-ranges"
                }
                <GLink>{
                    font_family: (BOLD_FONT2),
                    text: "3. AWS APIs",
                    font_size: 9.0,
                    href: "https://docs.aws.amazon.com/general/latest/gr/Welcome.html#aws-apis"
                }
                <GLink>{
                    font_family: (BOLD_FONT2),
                    text: "4. AWS services endpoints and quotas",
                    font_size: 9.0,
                    href: "https://docs.aws.amazon.com/general/latest/gr/Welcome.html#endpoints-quotas"
                }
                <GLink>{
                    font_family: (BOLD_FONT2),
                    text: "4. AWS Glossary",
                    font_size: 9.0,
                    href: "https://docs.aws.amazon.com/general/latest/gr/Welcome.html#aws-glossary"
                }
            }

        }
    }
}

#[derive(Live, Widget)]
pub struct SettingsPage {
    #[deref]
    pub super_widget: GView,
}

impl LiveHook for SettingsPage {}

impl Widget for SettingsPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.super_widget.draw_walk(cx, scope, walk)
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.super_widget.handle_event(cx, event, scope));
        self.gbutton(id!(change_btn)).borrow().map(|x| {
            if x.clicked(&actions).is_some() {
                GRouter::nav_to_path(cx, self.widget_uid(), scope, id!(sigin_screen));
            }
        });
    }
}
