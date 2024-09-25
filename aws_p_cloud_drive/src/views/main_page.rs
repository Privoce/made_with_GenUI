use std::{fs::read_dir, os::windows::fs::MetadataExt, path::PathBuf};

use gen_components::components::{
    button::GButtonWidgetExt,
    card::{GCard, GCardWidgetExt},
    file_upload::GUploadWidgetExt,
    icon::GIconWidgetExt,
    label::{GLabelWidgetExt, GLabelWidgetRefExt},
    table::{
        row::{GTableRowRef, GTableRowWidgetRefExt},
        GTableWidgetExt,
    },
};
use makepad_widgets::*;

use crate::utils::{ls, ls_dir, rm, FileTableItem, Handles, LsResult, S3Data};

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import gen_components::components::*;

    BOLD_FONT = dep("crate://self/resources/JuliaMono-BlackItalic.ttf");
    BOLD_FONT2 = dep("crate://self/resources/FiraCode-Bold.ttf");

    BtnLink = <GButton>{
        padding: 0.0,
        theme: Dark,
        background_visible: true,
        spread_radius: 0.0,
        width: 15.0,
        height: 16.0,
        border_radius: 3.8,
        slot: {
            text: ""
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
                        theme: Success
                    }
                    delete_btn = <BtnLink>{
                        theme: Error
                    }
                    download_btn = <BtnLink>{
                        theme: Primary
                    }
                }
            }
        }
        ls_item2: <GTRow>{
            height: 32.0,
            width: Fill,
            <GTCell>{
                height: Fill,
                width: 180.0,
                align: {x: 0.1, y: 0.5},
                name_str2 = <GLabel>{
                    color: #E36640,
                    text: "-",
                    font_size: 8.0
                }
            }

            <GTCell>{
                height: Fill,
                width: 120.0,
                align: {x: 0.1, y: 0.5},
                size_str2 = <GLabel>{
                    color: #1C2128,
                    text: "-",
                    font_size: 8.0
                }
            }
            opt_cell2 = <GTCell>{
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
                    upload_btn = <BtnLink>{
                        theme: Primary
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
                spacing: 6.0,
                align: {
                    x: 0.0,
                    y: 0.5
                }
                <GLabel>{
                    width: 348.0,
                    text: "AWS S3 Bucket",
                    font_family: (BOLD_FONT)
                }
                <GDropDown>{
                    mode: Dialog,
                    height: Fit,
                    width: Fit,
                    trigger = <GIcon>{
                        theme: Dark,
                        cursor: Hand,
                        height: 16.0,
                        width: 16.0,
                        icon_type: Share,
                        stroke_width: 1.4,
                    }
                    popup :<GDialog> {
                        container: {
                            height: 280.0,
                            width: 360.0,
                            flow: Down,
                            spacing: 10.0,
                            padding: 10.0,
                            <GCard>{
                                theme: Dark,
                                height: Fill,
                                width: Fill,
                                spread_radius: 4.6,
                                blur_radius: 4.6,
                                spacing: 12.0,
                                flow: Down,
                                clip_x: false,
                                clip_y: false,
                                padding: 2.0,
                                shadow_offset: vec2(0.0, 2.0),
                                align: {
                                    x: 0.5,
                                    y: 0.5
                                }
                                <GImage>{
                                    height: 86.0,
                                    width: 86.0,
                                    src: dep("crate://self/resources/share.png")
                                }
                                <GLabel>{
                                    font_size: 9.0,
                                    font_family:(BOLD_FONT2),
                                    text:"Scan the QR code!",
                                }
                                <GLabel>{
                                    font_size: 9.0,
                                    font_family: (BOLD_FONT2),
                                    text: "Copy the link to share!"
                                }
                                <GInput>{
                                    theme: Warning,
                                    width: Fill,
                                    height: 64.0,
                                    margin: 8.0,
                                    text: "https://github.com/Privoce/made_with_GenUI/tree/main/aws_p_cloud_drive"
                                }
                            }
                        }
                    }
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
            table_view = <GVLayout>{
                visible: false,
                height: Fit,
                spacing: 6.0,
                padding: 8.0,
                <GHLayout>{
                    height: Fit,
                    width: Fill,
                    spacing: 4.0
                    <BtnLink>{
                        theme: Success
                    }
                    <GLabel>{
                        text: "ls"
                    }
                    <BtnLink>{
                        theme: Error
                    }
                    <GLabel>{
                        text: "delete"
                    }
                    <BtnLink>{
                        theme: Primary
                    }
                    <GLabel>{
                        text: "download/upload"
                    }
                }
                ls_table1 = <GTable>{
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
        upload_view = <GVLayout>{
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
                    upload_icon = <GUpload>{
                        height: 16.0,
                        width: 16.0,
                        mode: Folder,
                        clear: true,
                        icon: {
                            theme: Warning,
                            height: 16.0,
                            width: 16.0,
                        }
                    }
                }
                fresh_upload = <GIcon>{
                    theme: Dark,
                    cursor: Hand,
                    height: 16.0,
                    width: 16.0,
                    icon_type: Fresh,
                    stroke_width: 1.4,
                }
            }
            tip2 = <GLabel>{
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
                            width: 180.0,
                            align: {x: 0.1, y: 0.5},
                            <GLabel>{
                                color: #E36640,
                                text: "Name",
                                font_family: (BOLD_FONT2)
                            }
                        }
                        <GTCell>{
                            height: Fill,
                            width: 120.0,
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
                                text: "Operations",
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
    #[live]
    pub ls_item2: Option<LivePtr>,
    #[rust]
    pub s3_datas: S3Data,
    #[rust]
    pub current_choose: Option<String>,
    #[rust]
    step: Step,
    #[rust]
    upload_dir: PathBuf,
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
        let mut upload_flag = false;
        self.gicon(id!(fresh_upload)).borrow().map(|icon| {
            if icon.clicked(&actions).is_some() {
                self.handle_upload(cx);
                upload_flag = true;
            }
        });

        self.gupload(id!(upload_icon)).borrow().map(|upload| {
            if let Some(uploads_dirs) = upload.after_select(&actions) {
                self.upload_dir = uploads_dirs[0].clone();
                self.handle_upload(cx);
                upload_flag = true;
            }
        });
        if upload_flag {
            self.gcard(id!(upload_view)).borrow_mut().map(|mut card| {
                card.redraw(cx);
            });
            return;
        }

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
        } else {
            match self.s3_datas {
                S3Data::Bucket(_) => {
                    self.step = Step::Bucket;
                }
                S3Data::Dir(_) => {
                    self.step = Step::Dir;
                }
            }
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
                                            cell.gbutton(id!(download_btn)).borrow_mut().map(
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
                                            cell.gbutton(id!(choose_btn)).borrow_mut().map(
                                                |mut x| {
                                                    x.visible = false;
                                                },
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
                            cell.gbutton(id!(download_btn)).borrow_mut().map(|mut x| {
                                x.visible = false;
                            });
                        });
                    }
                });
            }
            table.body_virtual.children = ls_children.clone();
            table.body_virtual.walk.height = Size::Fit;
            flag = true;
        });
        if flag {
            self.gcard(id!(table_view)).borrow_mut().map(|mut x| {
                x.visible = true;
            });
        }
        flag
    }
    fn ls_dir(&mut self, actions: &Actions) -> bool {
        let mut ls_flag = false;
        let mut handle = Handles::default();
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
                                handle = Handles::Ls;
                            }
                        });
                        cell.gbutton(id!(delete_btn)).borrow().map(|btn| {
                            if btn.clicked(&actions).is_some() {
                                flag = true;
                                handle = Handles::Delete;
                            }
                        });
                        cell.gbutton(id!(download_btn)).borrow().map(|btn| {
                            if btn.clicked(&actions).is_some() {
                                flag = true;
                                handle = Handles::Downlaod;
                            }
                        });
                    });
                }
                if flag {
                    match handle {
                        Handles::Ls => {
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
                            break;
                        }
                        Handles::Delete => {
                            let mut dir = None;
                            self.current_choose.as_ref().map(|x| {
                                dir.replace(x.to_string());
                            });

                            match rm(&dir.unwrap(), &target_dir_name) {
                                Ok(res) => {
                                    self.s3_datas = S3Data::Dir(res);
                                    ls_flag = true;
                                    self.step = Step::Dir;
                                }
                                Err(e) => {
                                    dbg!(e);
                                }
                            }
                        }
                        Handles::Downlaod => {
                            dbg!(&target_dir_name);
                        }
                        Handles::None => (),
                    }
                }
            }
        });
        return ls_flag;
    }
    fn handle_upload(&mut self, cx: &mut Cx) {
        // find all files in upload dir
        let mut files = vec![];
        for entry in read_dir(self.upload_dir.as_path()).unwrap() {
            let entry = entry.unwrap();
            if entry.path().is_file() {
                let f_meta = entry.metadata().unwrap();
                files.push(FileTableItem {
                    name: entry.file_name().to_str().unwrap().to_string(),
                    size: f_meta.file_size() as usize,
                });
            }
        }

        self.glabel(id!(tip2)).borrow_mut().map(|mut x| {
            x.visible = false;
        });
        let mut rows: ComponentMap<LiveId, GTableRowRef> = ComponentMap::default();
        self.gtable(id!(ls_table2)).borrow_mut().map(|mut table| {
            for (index, file) in files.iter().enumerate() {
                let row = rows.get_or_insert(cx, LiveId(index as u64), |cx| {
                    WidgetRef::new_from_ptr(cx, self.ls_item2).as_gtable_row()
                });
                row.borrow_mut().map(|row| {
                    for (_, cell) in row.children.iter() {
                        cell.borrow_mut().map(|cell| {
                            cell.glabel(id!(name_str2))
                                .set_text_and_redraw(cx, &file.name);
                            cell.glabel(id!(size_str2))
                                .set_text_and_redraw(cx, &format!("{}B", file.size));
                        });
                    }
                });
            }
            table.body_virtual.children = rows;
            table.body_virtual.walk.height = Size::Fit;
            table.visible = true;
        });
    }
}
