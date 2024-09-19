use gen_components::components::{card::GCard, label::GLabelWidgetRefExt, table::GTableWidgetExt};
use makepad_widgets::*;

use crate::state::State;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import gen_components::components::*;
    Personal = {{Personal}}{
        width: 280.0,
        flow: Down,
        padding: 10.0,
        spacing: 16.0,
        <GHLayout>{
            height: 32.0,
            width: Fill,
            spacing: 6.0,
            align: {x: 0.5, y: 0.5},
            <GImage>{
                height: 24.0,
                width: 32.0,
                src: dep("crate://self/resources/aws.png"),
            }
            <GLabel>{
                font_size: 12.0,
                text: "Aws Personal Cloud Drive",
            }
        }
        <GVLayout>{
            height: Fit,
            spacing: 2.0,
            <GHLayout>{
                height: Fit,
                spacing: 6.0,
                <GDropDown>{
                    mode: ToolTip,
                    height: Fit,
                    width: Fit,
                    trigger_mode: Hover,
                    position: BottomLeft,
                    trigger = <GLabel>{
                        text: "Configurations",
                    }
                    popup :<GToolTip> {
                        theme: Dark,
                        height: 150.0,
                        width: 220.0,
                        padding: {
                            top: 12.0,
                            
                        },
                        container: {
                            height: 150.0,
                            width: 220.0,
                            flow: Down,
                            <GLabel>{
                                margin: 6.0,
                                height: Fit,
                                width: 220.0,
                                text:"AWS Configurations is a file that stores your settings for AWS CLI, including your security credentials, default output format, and default region.",
                            }
                        }
                    }
                }
                <GDivider>{
                    theme: Dark,
                    stroke_width: 1.4,
                    width: Fill,
                }
            }
            config_table = <GTable>{
                height: 100.0,
                width: Fill,
                header: {
                    height: Fit,
                    <GTRow>{
                        height: 32.0,
                        width: Fit,
                        <GTCell>{
                            height: Fill,
                            width: 130.0,
                            <GLabel>{
                                color: #667085,
                                text: "Prop",
                            }
                        }
                        <GTCell>{
                            height: Fill,
                            width: 130.0,
                            <GLabel>{
                                color: #667085,
                                text: "Value",
                            }
                        }
                    }
                }
                body: {
                    height: Fit,
                    <GTRow>{
                        height: 32.0,
                        width: Fill,
                        <GTCell>{
                            height: Fill,
                            width: 130.0,
                            <GLabel>{
                                color: #E36640,
                                text: "Region",
                            }
                        }
                        <GTCell>{
                            height: Fill,
                            width: 130.0,
                            region_str = <GLabel>{
                                color: #6BC45D,
                                text: "-",
                            }
                        }
                    }
                    <GTRow>{
                        height: 32.0,
                        width: Fill,
                        <GTCell>{
                            height: Fill,
                            width: 130.0,
                            <GLabel>{
                                color: #E36640,
                                text: "Output",
                            }
                        }
                        <GTCell>{
                            height: Fill,
                            width: 130.0,
                            output_str = <GLabel>{
                                color: #6BC45D,
                                text: "-",
                            }
                        }
                    }
                }
            }
        }
        <GVLayout>{
            height: Fit,
            width: Fill,
            spacing: 2.0,
            <GHLayout>{
                height: Fit,
                spacing: 6.0,
                <GLabel>{
                    text: "Credentials",
                }
                <GDivider>{
                    theme: Dark,
                    stroke_width: 1.4,
                    width: Fill,
                }
            }
            credential_table = <GTable>{
                height: 200.0,
                width: Fill,
                header: {
                    visible: false,
                    height: Fit,
                    <GTRow>{
                        height: 32.0,
                        width: Fit,
                        <GTCell>{
                            height: Fill,
                            width: 130.0,
                            <GLabel>{
                                color: #667085,
                                text: "Prop",
                            }
                        }
                        <GTCell>{
                            height: Fill,
                            width: 130.0,
                            <GLabel>{
                                color: #667085,
                                text: "Value",
                            }
                        }
                    }
                }
                body: {
                    height: Fit,
                    <GTRow>{
                        height: 32.0,
                        width: Fill,
                        <GTCell>{
                            height: Fill,
                            width: Fill,
                            <GLabel>{
                                color: #E36640,
                                text: "Accsee Key ID",
                            }
                        }
                    }
                    <GTRow>{
                        height: 32.0,
                        width: Fill,
                        <GTCell>{
                            height: Fill,
                            width: Fill,
                            accsee_key_str = <GLabel>{
                                color: #6BC45D,
                                text: "-",
                            }
                        }
                    }
                    <GTRow>{
                        height: 32.0,
                        width: Fill,
                        <GTCell>{
                            height: Fill,
                            width: Fill,
                            <GLabel>{
                                color: #E36640,
                                text: "Secret Key",
                            }
                        }
                    }
                    <GTRow>{
                        height: 32.0,
                        width: Fill,
                        <GTCell>{
                            height: Fill,
                            width: Fill,
                            secret_key_str = <GLabel>{
                                color: #6BC45D,
                                text: "-",
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct Personal {
    #[deref]
    pub super_widget: GCard,
    #[rust]
    pub state: State,
}

impl Widget for Personal {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.super_widget.draw_walk(cx, scope, walk)
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.super_widget.handle_event(cx, event, scope);
    }
}

impl Personal {
    pub fn set_state(&mut self, cx: &mut Cx, state: State) {
        self.state = state;

        if self.state.login {
            self.gtable(id!(config_table)).borrow_mut().map(|table| {
                for (_child_id, child) in table.body.children.iter() {
                    child.borrow_mut().map(|child| {
                        for (_, cell) in child.children.iter() {
                            let region_cell = cell.glabel(id!(region_str));
                            region_cell.set_text_and_redraw(cx, self.state.region.as_str());
                            let output_cell = cell.glabel(id!(output_str));
                            output_cell.set_text_and_redraw(cx, self.state.output.as_str());
                        }
                    });
                }
            });

            self.gtable(id!(credential_table))
                .borrow_mut()
                .map(|table| {
                    for (_child_id, child) in table.body.children.iter() {
                        child.borrow_mut().map(|child| {
                            for (_, cell) in child.children.iter() {
                                let accsee_key_cell = cell.glabel(id!(accsee_key_str));
                                accsee_key_cell
                                    .set_text_and_redraw(cx, self.state.accsee_key.as_str());
                                let secret_key_cell = cell.glabel(id!(secret_key_str));

                                let secret1 = self.state.secret_key.as_str()[0..5].to_string();
                                let secret2 = self.state.secret_key.as_str()
                                    [self.state.secret_key.len() - 5..]
                                    .to_string();
                                secret_key_cell.set_text_and_redraw(
                                    cx,
                                    format!("{}***{}", secret1, secret2).as_str(),
                                );
                            }
                        });
                    }
                });

            self.redraw(cx);
        }
    }
}
