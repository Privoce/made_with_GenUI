use gen_components::components::{
    button::GButtonWidgetExt,
    card::{GCard, GCardWidgetExt},
    icon::GIconWidgetExt,
    label::{GLabelWidgetExt, GLabelWidgetRefExt},
    table::{
        row::{GTableRowRef, GTableRowWidgetRefExt},
        GTableWidgetExt,
    },
};
use makepad_widgets::*;

use crate::utils::{ls, ls_dir, LsResult, S3Data};

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import gen_components::components::*;

    BOLD_FONT = dep("crate://self/resources/JuliaMono-BlackItalic.ttf");
    BOLD_FONT2 = dep("crate://self/resources/FiraCode-Bold.ttf");

    BtnLink = <GButton>{
        padding: 0.0,
        theme: Dark,
        background_visible: false,
        spread_radius: 0.0,
        width: Fill,
        slot: {
            font_size: 8.0

        }
    }


    MainPage = {{MainPage}}{
        height: Fill,
        width: Fill,
        flow: Down,
        border_radius: 0.0,
        background_color: #161616,
        align: {
            x: 0.5,
            y: 0.0
        },
        spacing: 0.0,
        ls_item1: <GTRow>{
            height: 32.0,
            width: Fill,
            <GTCell>{
                height: Fill,
                width: 140.0,
                align: {x: 0.1, y: 0.5},

                name_str = <GLabel>{
                    color: #E36640,
                    text: "-",
                    font_size: 8.0
                }
            }
            <GTCell>{
                height: Fill,
                width: 110.0,
                align: {x: 0.1, y: 0.5},
                date_str = <GLabel>{
                    color: #1C2128,
                    text: "-",
                    font_size: 8.0
                }
            }
            <GTCell>{
                height: Fill,
                width: 90.0,
                align: {x: 0.1, y: 0.5},
                size_str = <GLabel>{
                    color: #1C2128,
                    text: "-",
                    font_size: 8.0
                }
            }
            opt_cell = <GTCell>{
                height: Fill,
                width: Fill,
                align: {x: 0.5, y: 0.5},
                <GHLayout>{
                    height: Fit,
                    spacing: 6.0,
                    align: {
                        x: 0.5,
                        y: 0.5
                    },
                    choose_btn = <BtnLink>{
                        width: 20.0,
                        slot: {
                            color: #161616,
                            text: "ls"
                        }
                    }
                    delete_btn = <BtnLink>{
                        slot: {
                            color: #F44336,
                            text: "delete"
                        }
                    }
                }
            }
        }
        <GVLayout>{
            height: 360.0,
            spacing: 8.0,
            align: {
                x: 0.5,
                y: 0.0,
            },
            padding: {
                bottom: 8.0,
            },
            clip_y: true,
            clip_x: true,
            scroll_bars: <GScrollBars>{}
            <GCard>{
                padding: 12.0,
                width: Fill,
                border_radius: 0.0,
                theme: Dark,
                height: Fit,

                align: {
                    x: 0.0,
                    y: 0.5
                }
                <GLabel>{
                    width: Fill,
                    text: "AWS S3 Bucket",
                    font_family: (BOLD_FONT)
                }
                <GIcon>{
                    theme: Dark,
                    cursor: Hand,
                    height: 16.0,
                    width: 16.0,
                    icon_type: Share,
                    stroke_width: 1.4,
                }
                fresh_s3 = <GIcon>{
                    theme: Dark,
                    cursor: Hand,
                    height: 16.0,
                    width: 16.0,
                    icon_type: Fresh,
                    stroke_width: 1.4,
                }
            }
            tip_view = <GVLayout>{
                height: Fill,
                align: {
                    x: 0.5,
                    y: 0.5
                },
                white_load1 = <GStateNoData>{
                    height: 120.0,
                    width: Fill,
                }
                <GHLayout>{
                    height: Fit,
                    spacing: 12.0,
                    padding: {left: 8.0, right: 8.0},
                    view_s3 = <GButton>{
                        width: Fill,
                        theme: Dark,
                        slot: {
                            font_family: (BOLD_FONT),
                            text: "View files on S3"
                        }
                    }
                    help_s3 = <GButton>{
                        width: Fill,
                        theme: Warning,
                        slot: {
                            font_family: (BOLD_FONT),
                            text: "No S3? Click Here!"
                        }
                    }
                }
            }

            ls_table1 = <GTable>{
                visible: false,
                margin: 8.0,
                mode: Virtual,
                height: Fit,
                width: Fill,
                header: {
                    height: Fit,
                    width: Fill,
                    <GTRow>{
                        height: 32.0,
                        width: Fill,
                        <GTCell>{
                            height: Fill,
                            width: 140.0,
                            align: {x: 0.1, y: 0.5},
                            <GLabel>{
                                color: #E36640,
                                text: "Name",
                                font_family: (BOLD_FONT2)
                            }
                        }
                        <GTCell>{
                            height: Fill,
                            width: 110.0,
                            align: {x: 0.1, y: 0.5},
                            <GLabel>{
                                color: #1C2128,
                                text: "Date",
                                font_family: (BOLD_FONT2)
                            }
                        }
                        <GTCell>{
                            height: Fill,
                            width: 90.0,
                            align: {x: 0.1, y: 0.5},
                            <GLabel>{
                                color: #1C2128,
                                text: "Size",
                                font_family: (BOLD_FONT2)
                            }
                        }
                        <GTCell>{
                            height: Fill,
                            width: Fill,
                            align: {x: 0.1, y: 0.5},

                        }
                    }
                }

            }
        }
        <GDivider>{
            height: 2.0,
            stroke_width: 1.2,
            margin: {
                left: 2.0,
                right: 2.0,
            },
            theme: Dark,
        }
        <GVLayout>{
            <GCard>{
                padding: 12.0,
                width: Fill,
                border_radius: 0.0,
                theme: Dark,
                height: Fit,
                spacing: 8.0,
                align: {
                    x: 0.0,
                    y: 0.5
                }
                <GLabel>{
                    width: Fill,
                    text: "Upload Bucket",
                    font_family: (BOLD_FONT)
                }
                <GHLayout>{
                    height: Fit,
                    width: Fit,
                    align: {
                        x: 0.5,
                        y: 0.5
                    },
                    <GUpload>{
                        height: 16.0,
                        width: 16.0,
                        mode: Folder,
                        icon: {
                            theme: Warning,
                            height: 16.0,
                            width: 16.0,
                        }
                    }
                }
                <GIcon>{
                    theme: Dark,
                    cursor: Hand,
                    height: 16.0,
                    width: 16.0,
                    icon_type: Fresh,
                    stroke_width: 1.4,
                }
            }
            <GLabel>{
                margin: 16.0,
                align: {
                    x: 0.5
                },
                width: Fill,
                font_size: 10.0,
                color: #FF7043,
                font_family: (BOLD_FONT),
                text: "You should click the upload icon to choose target bucket!"
            }
            ls_table2 = <GTable>{
                visible: false,
                margin: 8.0,
                mode: Virtual,
                height: Fit,
                width: Fill,
                header: {
                    height: Fit,
                    width: Fill,
                    <GTRow>{
                        height: 32.0,
                        width: Fill,
                        <GTCell>{
                            height: Fill,
                            width: 200.0,
                            align: {x: 0.1, y: 0.5},
                            <GLabel>{
                                color: #E36640,
                                text: "Name",
                                font_family: (BOLD_FONT2)
                            }
                        }
                        <GTCell>{
                            height: Fill,
                            width: 100.0,
                            align: {x: 0.1, y: 0.5},
                            <GLabel>{
                                color: #1C2128,
                                text: "Size",
                                font_family: (BOLD_FONT2)
                            }
                        }
                        <GTCell>{
                            height: Fill,
                            width: Fill,
                            align: {x: 0.1, y: 0.5},
                            <GLabel>{
                                color: #1C2128,
                                text: "Suffix",
                                font_family: (BOLD_FONT2)
                            }
                        }
                    }
                }

            }
        }
    }
}

#[derive(Live, Widget)]
pub struct MainPage {
    #[deref]
    pub super_widget: GCard,
    #[live]
    pub ls_item1: Option<LivePtr>,
    #[rust]
    pub s3_datas: S3Data,
    #[rust]
    pub current_choose: Option<String>,
    #[rust]
    step: Step,
}

#[derive(Debug, Clone, Copy)]
enum Step {
    Pre,
    Bucket,
    Dir,
    Fresh,
}
impl Default for Step {
    fn default() -> Self {
        Self::Pre
    }
}

impl Step {
    fn is_fresh(&self) -> bool {
        match self {
            Step::Fresh => true,
            _ => false,
        }
    }
}

impl LiveHook for MainPage {}

impl Widget for MainPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.super_widget.draw_walk(cx, scope, walk)
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.super_widget.handle_event(cx, event, scope));
        let mut ls_flag = false;
        self.gicon(id!(fresh_s3)).borrow().map(|x| {
            if x.clicked(&actions).is_some() {
                dbg!("clicked");
                match self.step {
                    Step::Pre => {
                        // do nothing
                    }
                    Step::Bucket => {
                        if let Ok(ls_tables) = ls() {
                            let mut ls_children: ComponentMap<LiveId, GTableRowRef> =
                                ComponentMap::default();
                            let flag = self.ls(cx, &mut ls_children, &ls_tables);
                            if flag {
                                self.gcard(id!(tip_view)).borrow_mut().map(|mut x| {
                                    x.visible = false;
                                });
                            }
                            self.s3_datas = S3Data::Bucket(ls_tables);
                            self.step = Step::Bucket;
                        }
                    }
                    Step::Dir => {
                        let target_dir_name = self.current_choose.as_ref().unwrap().to_string();
                        dbg!(&target_dir_name);
                        match self.s3_datas.clone() {
                            S3Data::Bucket(_) => {
                                if let Ok(res) = ls_dir(&target_dir_name) {
                                    self.s3_datas = S3Data::Dir(res);
                                    ls_flag = true;
                                    self.step = Step::Fresh;
                                }
                            }
                            S3Data::Dir(_) => {
                                if let Ok(res) = ls_dir(&target_dir_name) {
                                    dbg!(res.is_some());
                                    self.s3_datas = S3Data::Dir(res);
                                    ls_flag = true;
                                    self.step = Step::Fresh;
                                }
                            }
                        }
                    }
                    Step::Fresh => {}
                }
            }
        });

        self.gbutton(id!(view_s3)).borrow().map(|x| {
            if x.clicked(&actions).is_some() {
                // ls s3
                if let Ok(ls_tables) = ls() {
                    let mut ls_children: ComponentMap<LiveId, GTableRowRef> =
                        ComponentMap::default();
                    let flag = self.ls(cx, &mut ls_children, &ls_tables);
                    if flag {
                        self.gcard(id!(tip_view)).borrow_mut().map(|mut x| {
                            x.visible = false;
                        });
                    }
                    self.s3_datas = S3Data::Bucket(ls_tables);
                    self.step = Step::Bucket;
                }
            }
        });

        if !self.step.is_fresh() {
            ls_flag = self.ls_dir(&actions);
        }
        // means check inner
        if ls_flag {
            // update ls table1
            match &self.s3_datas {
                S3Data::Bucket(_) => {
                    println!("may be error!")
                }
                S3Data::Dir(ls_tables) => {
                    ls_tables.as_ref().map(|ls_tables| {
                        let mut ls_children: ComponentMap<LiveId, GTableRowRef> =
                            ComponentMap::default();
                        self.gtable(id!(ls_table1)).borrow_mut().map(|mut table| {
                            for (index, ls_table) in ls_tables.pre.iter().enumerate() {
                                let row =
                                    ls_children.get_or_insert(cx, LiveId(index as u64), |cx| {
                                        WidgetRef::new_from_ptr(cx, self.ls_item1).as_gtable_row()
                                    });
                                row.borrow_mut().map(|row| {
                                    for (_, cell) in row.children.iter() {
                                        cell.borrow_mut().map(|cell| {
                                            cell.glabel(id!(name_str))
                                                .set_text_and_redraw(cx, &ls_table);
                                            cell.gbutton(id!(delete_btn)).borrow_mut().map(
                                                |mut x| {
                                                    x.visible = false;
                                                },
                                            );
                                        });
                                    }
                                });
                            }
                            let len = ls_tables.pre.len();
                            for (index, ls_table) in ls_tables.files.iter().enumerate() {
                                let index = index + len;
                                let row =
                                    ls_children.get_or_insert(cx, LiveId(index as u64), |cx| {
                                        WidgetRef::new_from_ptr(cx, self.ls_item1).as_gtable_row()
                                    });
                                row.borrow_mut().map(|row| {
                                    for (_, cell) in row.children.iter() {
                                        cell.borrow_mut().map(|cell| {
                                            cell.glabel(id!(name_str))
                                                .set_text_and_redraw(cx, &ls_table.dir);
                                            cell.glabel(id!(date_str))
                                                .set_text_and_redraw(cx, &ls_table.date);
                                            cell.glabel(id!(size_str)).set_text_and_redraw(
                                                cx,
                                                &format!("{}B", ls_table.size),
                                            );
                                            cell.gbutton(id!(delete_btn)).borrow_mut().map(
                                                |mut x| {
                                                    x.visible = true;
                                                },
                                            );
                                        });
                                    }
                                });
                            }

                            table.body_virtual.children = ls_children.clone();
                            table.body_virtual.walk.height = Size::Fit;
                            table.visible = true;
                        });
                    });
                }
            }
        }
    }
}

impl MainPage {
    fn ls(
        &mut self,
        cx: &mut Cx,
        ls_children: &mut ComponentMap<LiveId, GTableRowRef>,
        ls_tables: &Vec<LsResult>,
    ) -> bool {
        let mut flag = false;
        self.gtable(id!(ls_table1)).borrow_mut().map(|mut table| {
            for (index, ls_table) in ls_tables.iter().enumerate() {
                let row = ls_children.get_or_insert(cx, LiveId(index as u64), |cx| {
                    WidgetRef::new_from_ptr(cx, self.ls_item1).as_gtable_row()
                });
                row.borrow_mut().map(|row| {
                    for (_, cell) in row.children.iter() {
                        cell.borrow_mut().map(|cell| {
                            cell.glabel(id!(name_str))
                                .set_text_and_redraw(cx, &ls_table.dir);
                            cell.glabel(id!(date_str))
                                .set_text_and_redraw(cx, &ls_table.date);
                            cell.gbutton(id!(delete_btn)).borrow_mut().map(|mut x| {
                                x.visible = false;
                            });
                        });
                    }
                });
            }
            table.body_virtual.children = ls_children.clone();
            table.body_virtual.walk.height = Size::Fit;
            table.visible = true;
            flag = true;
        });
        flag
    }
    fn ls_dir(&mut self, actions: &Actions) -> bool {
        let mut ls_flag = false;
        self.gtable(id!(ls_table1)).borrow().map(|table| {
            for (_row_index, (_, row)) in table.body_virtual.children.iter().enumerate() {
                let mut flag = false;
                let mut target_dir_name = String::new();
                for (_cell_index, (_cell_id, cell)) in
                    row.borrow().unwrap().children.iter().enumerate()
                {
                    let name = cell.glabel(id!(name_str)).text();
                    if !name.is_empty() {
                        target_dir_name = name;
                    }
                    cell.borrow().map(|cell| {
                        cell.gbutton(id!(choose_btn)).borrow().map(|btn| {
                            if btn.clicked(&actions).is_some() {
                                flag = true;
                            }
                        });
                    });
                }
                if flag {
                    // do ls inner
                    match self.s3_datas.clone() {
                        S3Data::Bucket(_) => {
                            if let Ok(res) = ls_dir(&target_dir_name) {
                                self.current_choose.replace(target_dir_name);
                                self.s3_datas = S3Data::Dir(res);
                                ls_flag = true;
                                self.step = Step::Dir;
                            }
                        }
                        S3Data::Dir(_) => {
                            self.current_choose.as_ref().map(|x| {
                                target_dir_name = format!("{}/{}", x, target_dir_name);
                            });
                            if let Ok(res) = ls_dir(&target_dir_name) {
                                self.current_choose.replace(target_dir_name);
                                self.s3_datas = S3Data::Dir(res);
                                ls_flag = true;
                                self.step = Step::Dir;
                            }
                        }
                    }
                }
            }
        });
        return ls_flag;
    }
}
