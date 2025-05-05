use iced::alignment::{Alignment};
use iced::theme::{Theme};
use iced::widget::{
    button, checkbox, column, row, scrollable, text, horizontal_space,
    image, container, Column, Row, text_input, Space, Radio,
};
use iced::event::{self, Event};
use iced::Subscription;
use iced::window;
use iced::{Element};
use iced::{Center, Color, Task, Length, Size};
use chrono::{Local, Datelike};
use serde::{Deserialize, Serialize};
use std::path::{Path};
use std::time::Instant;
use std::process::Command as stdCommand;
extern crate image as create_image;
mod get_winsize;
mod dump_file;
mod fromdirpressm;
mod copypressm;
mod todirpressm;
mod get_fromdirlistm;

use get_fromdirlistm::get_fromdirlistm;
use get_winsize::get_winsize;
use fromdirpressm::fromdirpressm;
use todirpressm::todirpressm;
use copypressm::copypressm;

pub fn main() -> iced::Result {
     let mut widthxx: f32 = 1350.0;
     let mut heightxx: f32 = 750.0;
     let (errcode, _errstring, widtho, heighto) = get_winsize();
     if errcode == 0 {
         widthxx = widtho as f32 - 20.0;
         heightxx = heighto as f32 - 75.0;
     }
     iced::application(ImageList::title, ImageList::update, ImageList::view)
        .window_size((widthxx, heightxx))
        .theme(ImageList::theme)
        .subscription(ImageList::subscription)
        .run_with(ImageList::new)

}

#[derive(Debug)]
enum ImageList {
    Loaded(State),
    
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum YearChoice {
    NON,
    YR1,
    YR2,
    YR3,
    YR4,
    YR5,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MonthChoice {
    Non,
    Jan,
    Feb,
    Mar,
    Apr,
    May,
    Jun,
    Jul,
    Aug,
    Sep,
    Oct,
    Nov,
    Dec,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DayChoice {
    Non,
    D01,
    D02,
    D03,
    D04,
    D05,
    D06,
    D07,
    D08,
    D09,
    D10,
    D11,
    D12,
    D13,
    D14,
    D15,
    D16,
    D17,
    D18,
    D19,
    D20,
    D21,
    D22,
    D23,
    D24,
    D25,
    D26,
    D27,
    D28,
    D29,
    D30,
    D31,
}

impl Default for YearChoice {
    fn default() -> Self {
        YearChoice::NON
    }
}
impl Default for MonthChoice {
    fn default() -> Self {
        MonthChoice::Non
    }
}
impl Default for DayChoice {
    fn default() -> Self {
        DayChoice::Non
    }
}


#[derive(Debug, Default)]
struct State {
    filter: Filter,
    images: Vec<ImageItem>,
    yearchoice_value: YearChoice,
    monthchoice_value: MonthChoice,
    daychoice_value: DayChoice,
    fromdir_value: String,
    todir_value: String,
    msg_value: String,
    mess_color: Color,
    size_value: String,
    col_size: i32,
    fromyear_value: String,
    year01: String,
    year02: String,
    year03: String,
    year04: String,
    year05: String,
    folderyear: String,
    foldermonth: String,
    folderday: String,
    screenwidth: f32,
}

#[derive(Debug, Clone)]
enum Message {
    FilterChanged(Filter),
    ImageMessage(usize, ImageMessage),
    FromDirPressed,
    ToDirPressed,
    RefreshPressed,
    SetDatePressed,
    ReloadPressed,
    YearRadioSelected(YearChoice),
    MonthRadioSelected(MonthChoice),
    DayRadioSelected(DayChoice),
    CopyPressed,
    CopymovexFound(Result<Copymovex, Error>),
    SizeChanged(String),
    FromYear(String),
    Size(Size),

}

impl ImageList {
    fn new() -> (Self, Task<Message>) {
        let mut widthxx: u32 = 1300;
        let (errcode, errstring, widtho, _heighto) = get_winsize();
        let for_message: String;
        let datenow = Local::now();
        let yearnow = datenow.year();
        let year05s = format!("{}", yearnow);
        let year04s = format!("{}", (yearnow-1));
        let year03s = format!("{}", (yearnow-2));
        let year02s = format!("{}", (yearnow-3));
        let year01s = format!("{}", (yearnow-4));
        if errcode == 0 {
            widthxx = widtho;
            for_message = format!("{}", errstring);
        } else {
            for_message = format!("**ERROR {} get_winsize: {}", errcode, errstring);
        }

        (
            ImageList::Loaded(State
               {
                filter:Filter::All,
                images:Vec::<ImageItem>::new(),
                fromdir_value: "no directory".to_string(),
                todir_value: "no directory".to_string(),
                yearchoice_value:YearChoice::NON,
                monthchoice_value:MonthChoice::Non,
                daychoice_value:DayChoice::Non,
                mess_color: Color::from([0.5, 0.5, 1.0]),
                msg_value: for_message.to_string(),
                size_value: "140".to_string(),
                col_size: 2,
                fromyear_value: year01s.clone(),
                year01: year01s,
                year02: year02s,
                year03: year03s,
                year04: year04s,
                year05: year05s,
                folderyear: "YYYY".to_string(),
                foldermonth: "MM".to_string(),
                folderday: "DD".to_string(),
                screenwidth: widthxx as f32,
                }
            ),
            Task::none(),
        )
    }

    fn title(&self) -> String {
        format!("Copy images into a directory -- iced")
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match self {
            ImageList::Loaded(state) => {

                let command = match message {
                    Message::FilterChanged(filter) => {
                        state.filter = filter;

                        Task::none()
                    }
                    Message::ImageMessage(i, image_message) => {
                        if let Some(image) = state.images.get_mut(i) {

                            image.update(image_message);

                               Task::none()
                        } else {
                            Task::none()
                        }
                    }
                    Message::Size(size) => {
                       if state.size_value.len() == 0 { 
                           state.msg_value = "********* Icon has no value **********".to_string();
                           state.mess_color = Color::from([1.0, 0.0, 0.0]);
                           state.col_size = 2;
                       } else {
                           let icon_int: i32 = state.size_value.parse().unwrap_or(-99);
                           if icon_int > 0 {
                               if (icon_int < 50) | (icon_int > 255) {
                                   state.msg_value = "********* Icon not between 50 and 255 **********".to_string();
                                   state.mess_color = Color::from([1.0, 0.0, 0.0]);
                                   state.col_size = 2;
                               } else {
                                   let imagewidth = icon_int + 20;
                                   let numcol = (size.width / imagewidth as f32) as i32;
                                   if numcol < 1 {
                                       state.col_size = 1;
                                   } else if numcol > 7 {
                                       state.col_size = 7;
                                   } else {
                                       state.col_size = numcol;
                                   }
                                   state.msg_value = format!("Icon {} and num columns {} and screen {} and image width {}", state.size_value, state.col_size, size.width, imagewidth);
                                   state.mess_color = Color::from([1.0, 0.0, 0.0]);
                               }
                           } else if icon_int == -99 {
                               state.msg_value = "********* Icon is not an integer **********".to_string();
                               state.mess_color = Color::from([1.0, 0.0, 0.0]);
                               state.col_size = 2;
                           } else {
                               state.msg_value = "********* Icon Size not positive integer **********".to_string();
                               state.mess_color = Color::from([1.0, 0.0, 0.0]);
                               state.col_size = 2;
                           }
                       }
                         state.screenwidth = size.width;
                         Task::none()
                    }
                    Message::YearRadioSelected(xchoice) => {
                        let strx: &str;
                        match xchoice {
                              YearChoice::NON => {
                                  strx = "choice no year selected";
                                  state.monthchoice_value = MonthChoice::Non;
                                  state.daychoice_value = DayChoice::Non;
                                  state.folderyear = "YYYY".to_string();
                                  state.foldermonth = "MM".to_string();
                                  state.folderday = "DD".to_string(); 
                              },
                              YearChoice::YR1 => {
                                  strx = "choice year 1 selected";
                                  state.folderyear = state.year01.clone();
                              },
                              YearChoice::YR2 => {
                                  strx = "choice year 2 selected";
                                  state.folderyear = state.year02.clone();
                              },
                              YearChoice::YR3 => {
                                  strx = "choice year 3 selected";
                                  state.folderyear = state.year03.clone();
                              },
                              YearChoice::YR4 => {
                                  strx = "choice year 4 selected";
                                  state.folderyear = state.year04.clone();
                              },
                              YearChoice::YR5 => {
                                  strx = "choice year 5 selected";
                                  state.folderyear = state.year05.clone();
                              },
                        };
                        state.yearchoice_value = xchoice;
                        state.msg_value = strx.to_string();
                        Task::none()
                    }
                    Message::MonthRadioSelected(xchoice) => {
                        let strx: &str;
                        match xchoice {
                              MonthChoice::Non => {
                                  strx = "choice no month selected";
                                  state.foldermonth = "MM".to_string();
                                  state.daychoice_value = DayChoice::Non;
                                  state.folderday = "DD".to_string(); 
                              },
                              MonthChoice::Jan => {
                                  strx = "choice Jan month selected";
                                  state.foldermonth = "01".to_string();
                              },
                              MonthChoice::Feb => {
                                  strx = "choice Feb month selected";
                                  state.foldermonth = "02".to_string();
                              },
                              MonthChoice::Mar => {
                                  strx = "choice Mar month selected";
                                  state.foldermonth = "03".to_string();
                              },
                              MonthChoice::Apr => {
                                  strx = "choice Apr month selected";
                                  state.foldermonth = "04".to_string();
                              },
                              MonthChoice::May => {
                                  strx = "choice May month selected";
                                  state.foldermonth = "05".to_string();
                              },
                              MonthChoice::Jun => {
                                  strx = "choice Jun month selected";
                                  state.foldermonth = "06".to_string();
                              },
                              MonthChoice::Jul => {
                                  strx = "choice Jul month selected";
                                  state.foldermonth = "07".to_string();
                              },
                              MonthChoice::Aug => {
                                  strx = "choice Aug month selected";
                                  state.foldermonth = "08".to_string();
                              },
                              MonthChoice::Sep => {
                                  strx = "choice Sep month selected";
                                  state.foldermonth = "09".to_string();
                              },
                              MonthChoice::Oct => {
                                  strx = "choice Oct month selected";
                                  state.foldermonth = "10".to_string();
                              },
                              MonthChoice::Nov => {
                                  strx = "choice Nov month selected";
                                  state.foldermonth = "11".to_string();
                              },
                              MonthChoice::Dec => {
                                  strx = "choice Dec month selected";
                                  state.foldermonth = "12".to_string();
                              },
                       };
                       state.monthchoice_value = xchoice;
                       state.msg_value = strx.to_string();
                       Task::none()
                    }
                    Message::DayRadioSelected(xchoice) => {
                        let strx: &str;
                        match xchoice {
                              DayChoice::Non => {
                                  strx = "choice no day selected";
                                  state.folderday = "DD".to_string();
                              },
                              DayChoice::D01 => {
                                  strx = "choice 01 day selected";
                                  state.folderday = "01".to_string();
                              },
                              DayChoice::D02 => {
                                  strx = "choice 02 day selected";
                                  state.folderday = "02".to_string();
                              },
                              DayChoice::D03 => {
                                  strx = "choice 03 day selected";
                                  state.folderday = "03".to_string();
                              },
                              DayChoice::D04 => {
                                  strx = "choice 04 day selected";
                                  state.folderday = "04".to_string();
                              },
                              DayChoice::D05 => {
                                  strx = "choice 05 day selected";
                                  state.folderday = "05".to_string();
                              },
                              DayChoice::D06 => {
                                  strx = "choice 06 day selected";
                                  state.folderday = "06".to_string();
                              },
                              DayChoice::D07 => {
                                  strx = "choice 07 day selected";
                                  state.folderday = "07".to_string();
                              },
                              DayChoice::D08 => {
                                  strx = "choice 08 day selected";
                                  state.folderday = "08".to_string();
                              },
                              DayChoice::D09 => {
                                  strx = "choice 09 day selected";
                                  state.folderday = "09".to_string();
                              },
                              DayChoice::D10 => {
                                  strx = "choice 10 day selected";
                                  state.folderday = "10".to_string();
                              },
                              DayChoice::D11 => {
                                  strx = "choice 11 day selected";
                                  state.folderday = "11".to_string();
                              },
                              DayChoice::D12 => {
                                  strx = "choice 12 day selected";
                                  state.folderday = "12".to_string();
                              },
                              DayChoice::D13 => {
                                  strx = "choice 13 day selected";
                                  state.folderday = "13".to_string();
                              },
                              DayChoice::D14 => {
                                  strx = "choice 14 day selected";
                                  state.folderday = "14".to_string();
                              },
                              DayChoice::D15 => {
                                  strx = "choice 15 day selected";
                                  state.folderday = "15".to_string();
                              },
                              DayChoice::D16 => {
                                  strx = "choice 16 day selected";
                                  state.folderday = "16".to_string();
                              },
                              DayChoice::D17 => {
                                  strx = "choice 17 day selected";
                                  state.folderday = "17".to_string();
                              },
                              DayChoice::D18 => {
                                  strx = "choice 18 day selected";
                                  state.folderday = "18".to_string();
                              },
                              DayChoice::D19 => {
                                  strx = "choice 19 day selected";
                                  state.folderday = "19".to_string();
                              },
                              DayChoice::D20 => {
                                  strx = "choice 20 day selected";
                                  state.folderday = "20".to_string();
                              },
                              DayChoice::D21 => {
                                  strx = "choice 21 day selected";
                                  state.folderday = "21".to_string();
                              },
                              DayChoice::D22 => {
                                  strx = "choice 22 day selected";
                                  state.folderday = "22".to_string();
                              },
                              DayChoice::D23 => {
                                  strx = "choice 23 day selected";
                                  state.folderday = "23".to_string();
                              },
                              DayChoice::D24 => {
                                  strx = "choice 24 day selected";
                                  state.folderday = "24".to_string();
                              },
                              DayChoice::D25 => {
                                  strx = "choice 25 day selected";
                                  state.folderday = "25".to_string();
                              },
                              DayChoice::D26 => {
                                  strx = "choice 26 day selected";
                                  state.folderday = "26".to_string();
                              },
                              DayChoice::D27 => {
                                  strx = "choice 27 day selected";
                                  state.folderday = "27".to_string();
                              },
                              DayChoice::D28 => {
                                  strx = "choice 28 day selected";
                                  state.folderday = "28".to_string();
                              },
                              DayChoice::D29 => {
                                  strx = "choice 29 day selected";
                                  state.folderday = "29".to_string();
                              },
                              DayChoice::D30 => {
                                  strx = "choice 30 day selected";
                                  state.folderday = "30".to_string();
                              },
                              DayChoice::D31 => {
                                  strx = "choice 31 day selected";
                                  state.folderday = "31".to_string();
                              },
                       };
                       state.daychoice_value = xchoice;
                       state.msg_value = strx.to_string();
                       Task::none()
                    }

                    Message::FromDirPressed => {
                       let (errcode, errstr, newdir, listitems, newtoi, icon_int1) = fromdirpressm(state.fromdir_value.clone(), state.size_value.clone());
                       if errcode == 0 {
                           if newtoi != 0 {
                               state.images.clear();                         
                               for indexi in 0..newtoi {
                                    state.fromdir_value = newdir.to_string();
                                    let linestr = listitems[indexi as usize].clone();
                                    let lineparse: Vec<&str> = linestr[0..].split(" | ").collect();
                                    let filefromx = lineparse[0].to_string();
                                    let fullpath = state.fromdir_value.clone() + "/" + &filefromx;
                                    let newwidth: u32;
                                    let newheight: u32;
                                    if let Ok((iwidth, iheight)) = create_image::image_dimensions(fullpath.clone()) {
                                        if iwidth > iheight {
                                            newwidth = icon_int1;
                                            newheight = icon_int1 * iheight / iwidth;
                                        } else {
                                            newheight = icon_int1;
                                            newwidth = icon_int1 * iwidth / iheight;
                                        }
                                        let loadimg = create_image::open(fullpath.clone()).unwrap();
                                        let imgbuffer = create_image::imageops::thumbnail(&loadimg, newwidth, newheight);
                                        let rgbconv = imgbuffer.into_vec();
                                        state
                                           .images
                                           .push(ImageItem::new(listitems[indexi as usize].clone(), rgbconv, newwidth, newheight));
                                    } else {
                                        println!("no wxh fullpath -{}- ", fullpath);
                                    }
                               }
                           }
                       }
                       state.msg_value = errstr.to_string();
                       if errcode == 0 {
                           state.mess_color = Color::from([0.0, 1.0, 0.0]);
                       } else {
                           state.mess_color = Color::from([1.0, 0.0, 0.0]);
                       };

                       Task::none()
                    } 

                    Message::ReloadPressed => {
                       if !Path::new(&state.fromdir_value).exists() {
                           state.msg_value = "from direcory does not exist".to_string();
                           state.mess_color = Color::from([1.0, 0.0, 0.0]);
                       } else {
                           let (errcda, errstra, newlist) = get_fromdirlistm(Path::new(&state.fromdir_value).to_path_buf());
                           if errcda != 0 {
                               state.msg_value = errstra.to_string();
                               state.mess_color = Color::from([1.0, 0.0, 0.0]);
                           } else {
                               if state.size_value.len() == 0 { 
                                   state.msg_value = "********* List: Icon has no value **********".to_string();
                                   state.mess_color = Color::from([1.0, 0.0, 0.0]);
                               } else {
                                   let icon_int: i32 = state.size_value.parse().unwrap_or(-99);
                                   if icon_int > 0 {
                                       if (icon_int < 50) | (icon_int > 255) {
                                           state.msg_value = "********* List: Icon not between 50 and 255 **********".to_string();
                                           state.mess_color = Color::from([1.0, 0.0, 0.0]);
                                       } else {
                                           let newtoi = newlist.len() as i32 ;
                                           state.images.clear();                         
                                           if newtoi != 0 {
                                               state.images.clear();                         
                                               for indexi in 0..newtoi {
                                                    let linestr = newlist[indexi as usize].clone();
                                                    let lineparse: Vec<&str> = linestr[0..].split(" | ").collect();
                                                    let filefromx = lineparse[0].to_string();
                                                    let fullpath = state.fromdir_value.clone() + "/" + &filefromx;
                                                    let newwidth: u32;
                                                    let newheight: u32;
                                                    if let Ok((iwidth, iheight)) = create_image::image_dimensions(fullpath.clone()) {
                                                        if iwidth > iheight {
                                                            newwidth = icon_int as u32;
                                                            newheight = (icon_int * iheight as i32 / iwidth as i32) as u32;
                                                        } else {
                                                            newheight = icon_int as u32;
                                                            newwidth = (icon_int * iwidth as i32 / iheight as i32) as u32;
                                                        }
                                                        let loadimg = create_image::open(fullpath.clone()).unwrap();
                                                        let imgbuffer = create_image::imageops::thumbnail(&loadimg, newwidth, newheight);
                                                        let rgbconv = imgbuffer.into_vec();
                                                        state
                                                         .images
                                                         .push(ImageItem::new(newlist[indexi as usize].clone(), rgbconv, newwidth, newheight));
                                                    } else {
                                                        println!("no wxh fullpath -{}- ", fullpath);
                                                    }
                                               }
                                               state.msg_value = "Reload of images completed".to_string();
                                               state.mess_color = Color::from([0.0, 1.0, 0.0]);
                                           } else {
                                               state.msg_value = "no more images to load".to_string();
                                               state.mess_color = Color::from([1.0, 0.0, 0.0]);
                                           }
                                       }
                                   } else if icon_int == -99 {
                                       state.msg_value = "********* List: Icon is not an integer **********".to_string();
                                       state.mess_color = Color::from([1.0, 0.0, 0.0]);
                                   } else {
                                       state.msg_value = "********* List: Icon Size not positive integer **********".to_string();
                                       state.mess_color = Color::from([1.0, 0.0, 0.0]);
                                   }
                               }
                           }
                       }
                       Task::none()
                    } 
                    Message::ToDirPressed => {
                        let (errcode, errstr, newdir) = todirpressm(state.fromdir_value.clone());
                        state.msg_value = errstr.to_string();
                        if errcode == 0 {
                            state.todir_value = newdir;
                            state.mess_color = Color::from([0.0, 1.0, 0.0]);
                        } else {
                            state.mess_color = Color::from([1.0, 0.0, 0.0]);
                        }
                        Task::none()
                    } 
                    Message::CopyPressed => {
// check if selections
                       let images_selected = state.images.iter().filter(|imageitem| imageitem.completed).count();
                       if images_selected < 1 {
                           state.msg_value = "no FROM image selected".to_string();
                           state.mess_color = Color::from([1.0, 0.0, 0.0]);
                           Task::none()
                       } else {
                           state.msg_value = format!("copy ready for validation: {} images selected", images_selected);
                           state.mess_color = Color::from([0.0, 1.0, 0.0]);
                           let mut listofimages: Vec<String> = Vec::new();
                           for imagesy in state.images.iter() {
                                if imagesy.completed {
                                    let lineparse: Vec<&str> = imagesy.description.split(" | ").collect();
                                    let fromfilename = lineparse[0].to_string();
                                    listofimages.push(fromfilename);
                                }
                           }
                           let (errcode, errstr) = copypressm(listofimages.clone(), state.fromdir_value.clone(),
                                                              state.todir_value.clone(), state.folderyear.clone(), state.foldermonth.clone(), 
                                                              state.folderday.clone());
                           if errcode == 0 {
                               state.msg_value = format!("{} images selected {}", listofimages.len(), listofimages[0]);
                               state.mess_color = Color::from([0.0, 1.0, 0.0]);
                               Task::perform(Copymovex::copymoveit(listofimages.clone(), state.fromdir_value.clone(),
                                                              state.todir_value.clone(), state.folderyear.clone(), state.foldermonth.clone(), 
                                                              state.folderday.clone()), Message::CopymovexFound)
                           } else {
                               state.msg_value = errstr.to_string();
                               state.mess_color = Color::from([1.0, 0.0, 0.0]);
                               Task::none()
                           }
                       }
                    }
                    Message::CopymovexFound(Ok(copyx)) => {
                        state.msg_value = copyx.errval.clone();
                        state.mess_color = copyx.errcolor.clone();
                        Task::none()
                    }
                    Message::CopymovexFound(Err(_error)) => {
                        state.msg_value = "error in copymoveit routine".to_string();
                        state.mess_color = Color::from([1.0, 0.0, 0.0]);
                        Task::none()
                    }
                    Message::SetDatePressed => {
                       let images_selected = state.images.iter().filter(|imageitem| imageitem.completed).count();
                       if images_selected < 1 {
                           state.msg_value = "no FROM image selected".to_string();
                           state.mess_color = Color::from([1.0, 0.0, 0.0]);
                       } else {
                           let mut bolok = false;
                           for imagesy in state.images.iter() {
                                if imagesy.completed {
                                    bolok = true;
                                    let lineparse: Vec<&str> = imagesy.description.split(" | ").collect();
                                    let fromdisplaydate = lineparse[1].to_string();
                                    let dateyr = fromdisplaydate.get(3..7).unwrap().to_string();
                                    let from_int: i32 = state.year01.parse().unwrap_or(-99);
                                    if from_int > 0 {
                                        let nyr_int: i32 = dateyr.parse().unwrap_or(-99);
                                        if nyr_int > 0 {
                                            if (nyr_int >= from_int) && (nyr_int <= (from_int+4)) {
                                                let datemo = fromdisplaydate.get(8..10).unwrap().to_string();
                                                let nmo_int: i32 = datemo.parse().unwrap_or(-99);
                                                if nmo_int > 0 {
                                                    if (nmo_int >= 1) && (nmo_int <= 12) {
                                                        let dateday = fromdisplaydate.get(11..13).unwrap().to_string();
                                                        let nda_int: i32 = dateday.parse().unwrap_or(-99);
                                                        if nda_int > 0 {
                                                          if (nda_int >= 1) && (nda_int <= 31) {
                                                            let vecyear: Vec<YearChoice> = [YearChoice::NON, YearChoice::YR1, YearChoice::YR2,
                                                                                  YearChoice::YR3, YearChoice::YR4, YearChoice::YR5].to_vec();
                                                            let vecmon: Vec<MonthChoice> = [MonthChoice::Non, MonthChoice::Jan, MonthChoice::Feb,
                                                                                  MonthChoice::Mar, MonthChoice::Apr, MonthChoice::May,
                                                                                  MonthChoice::Jun, MonthChoice::Jul, MonthChoice::Aug,
                                                                                  MonthChoice::Sep, MonthChoice::Oct, MonthChoice::Nov,
                                                                                  MonthChoice::Dec].to_vec();
                                                             let vecday: Vec<DayChoice> = [DayChoice::Non, DayChoice::D01, DayChoice::D02,
                                                                                  DayChoice::D03, DayChoice::D04, DayChoice::D05, DayChoice::D06,
                                                                                  DayChoice::D07, DayChoice::D08, DayChoice::D09, DayChoice::D10,
                                                                                  DayChoice::D11, DayChoice::D12, DayChoice::D13, DayChoice::D14,
                                                                                  DayChoice::D15, DayChoice::D16, DayChoice::D17, DayChoice::D18,
                                                                                  DayChoice::D19, DayChoice::D20, DayChoice::D21, DayChoice::D22,
                                                                                  DayChoice::D23, DayChoice::D24, DayChoice::D25, DayChoice::D26,
                                                                                  DayChoice::D27, DayChoice::D28, DayChoice::D29, DayChoice::D30,
                                                                                  DayChoice::D31].to_vec();
                                                             state.yearchoice_value = vecyear[(nyr_int - from_int + 1) as usize];
                                                             state.monthchoice_value = vecmon[nmo_int as usize];
                                                             state.daychoice_value = vecday[nda_int as usize];
                                                             state.msg_value = format!("folder set to display date of {}", fromdisplaydate);
                                                             state.mess_color = Color::from([0.0, 1.0, 0.0]);
                                                             state.folderyear = dateyr;
                                                             state.foldermonth = datemo;
                                                             state.folderday = dateday;
                                                          } else {
                                                             state.msg_value = format!("display day of {} is not valid", dateday);
                                                             state.mess_color = Color::from([1.0, 0.0, 0.0]);
                                                          }
                                                         } else {
                                                          state.msg_value = format!("display day of {} is not valid", dateday);
                                                          state.mess_color = Color::from([1.0, 0.0, 0.0]);
                                                         }
                                                    } else {
                                                         state.msg_value = format!("display month of {} is not valid", datemo);
                                                         state.mess_color = Color::from([1.0, 0.0, 0.0]);
                                                    }
                                                } else {
                                                    state.msg_value = format!("display month of {} is not valid", datemo);
                                                    state.mess_color = Color::from([1.0, 0.0, 0.0]);
                                                }
                                            } else {
                                                state.msg_value = format!("display year of {} not in current years range", dateyr);
                                                state.mess_color = Color::from([1.0, 1.0, 0.0]);
                                            }
                                        } else {
                                            state.msg_value = format!("display year of {} is not valid", dateyr);
                                            state.mess_color = Color::from([1.0, 0.0, 0.0]);
                                        }
                                    } else {
                                        state.msg_value = format!("year01 is invalid year of {}", state.year01);
                                        state.mess_color = Color::from([1.0, 0.0, 0.0]);
                                    }
                                    break;
                                }
                           }
                           if !bolok {
                                state.msg_value = format!("no images selected error");
                                state.mess_color = Color::from([1.0, 0.0, 0.0]);
                           }
                       }
                       Task::none()
                    }
                    Message::RefreshPressed => {
                       if state.fromyear_value.len() == 0 { 
                           state.msg_value = "********* from year has no value **********".to_string();
                           state.mess_color = Color::from([1.0, 0.0, 0.0]);
                       } else {
                           let mut from_int: i32 = state.fromyear_value.parse().unwrap_or(-99);
                           if from_int > 0 {
                               let datenow = Local::now();
                               let yearnow = datenow.year();
                               if (from_int < 1800) | (from_int > yearnow) {
                                   state.msg_value = format!("********* from year not between 1800 and {} **********", yearnow);
                                   state.mess_color = Color::from([1.0, 0.0, 0.0]);
                               } else {
                                   if from_int > (yearnow - 4) {
                                       from_int = yearnow - 4;                                       
                                       state.msg_value = "refreshed years but from year within current 5 years".to_string();
                                       state.mess_color = Color::from([1.0, 1.0, 0.0]);
                                   } else {
                                       state.msg_value = "refreshed years".to_string();
                                       state.mess_color = Color::from([0.0, 1.0, 0.0]);
                                   }
                                   state.year01 = format!("{}", from_int);
                                   state.year02 = format!("{}", (from_int+1));
                                   state.year03 = format!("{}", (from_int+2));
                                   state.year04 = format!("{}", (from_int+3));
                                   state.year05 = format!("{}", (from_int+4));
                                   state.yearchoice_value = YearChoice::NON;
                                   state.monthchoice_value = MonthChoice::Non;
                                   state.daychoice_value = DayChoice::Non;
                                   state.folderyear = "YYYY".to_string();
                                   state.foldermonth = "MM".to_string();
                                   state.folderday = "DD".to_string();
                               }
                           } else {
                               state.msg_value = "********* from year has bad value **********".to_string();
                               state.mess_color = Color::from([1.0, 0.0, 0.0]);
                           }
                       }
                       Task::none()
                    } 
                    Message::SizeChanged(value) => { 
                       if value.len() == 0 { 
                           state.msg_value = "********* Icon has no value **********".to_string();
                           state.mess_color = Color::from([1.0, 0.0, 0.0]);
                           state.col_size = 2;
                       } else {
                           let icon_int: i32 = value.parse().unwrap_or(-99);
                           if icon_int > 0 {
                               if (icon_int < 50) | (icon_int > 255) {
                                   state.msg_value = "********* Icon not between 50 and 255 **********".to_string();
                                   state.mess_color = Color::from([1.0, 0.0, 0.0]);
                                   state.col_size = 2;
                               } else {
                                   let imagewidth = icon_int + 20;
                                   let numcol = (state.screenwidth / imagewidth as f32) as i32;
                                   if numcol < 1 {
                                       state.col_size = 1;
                                   } else if numcol > 7 {
                                       state.col_size = 7;
                                   } else {
                                       state.col_size = numcol;
                                   }
                                   state.msg_value = format!("Icon {} and num columns {} and screen {} and image width {}", value, state.col_size, state.screenwidth, imagewidth);
                                   state.mess_color = Color::from([1.0, 0.0, 0.0]);
                               }
                           } else if icon_int == -99 {
                               state.msg_value = "********* Icon is not an integer **********".to_string();
                               state.mess_color = Color::from([1.0, 0.0, 0.0]);
                               state.col_size = 2;
                           } else {
                               state.msg_value = "********* Icon Size not positive integer **********".to_string();
                               state.mess_color = Color::from([1.0, 0.0, 0.0]);
                               state.col_size = 2;
                           }
                       }
                       state.size_value = value;
                       Task::none()
                    }
                    Message::FromYear(value) => { state.fromyear_value = value; Task::none() }

                };

                Task::batch(vec![command, Task::none()])
            }
        }
    }

    fn view(&self) -> Element<Message> {
        match self {
            ImageList::Loaded(State {
                filter,
                images,
                fromdir_value,
                todir_value,
                yearchoice_value,
                monthchoice_value,
                daychoice_value,
                msg_value,
                mess_color,
                size_value,
                col_size,
                fromyear_value,
                year01,
                year02,
                year03,
                year04,
                year05,
                folderyear,
                foldermonth,
                folderday,
                screenwidth,
                ..
            }) => {
                let mut messcol = Column::new().spacing(10);
                messcol = messcol.push(container(row![text("Message:").size(20),
                 text(msg_value).size(20).color(*mess_color),
            ].align_y(Alignment::Center).spacing(10).padding(5)
                    ));

                let mut dirbutshow = Column::new().spacing(10);
                let dirspace = 5.0;
                dirbutshow = dirbutshow.push(container(row![container(row![button("From Directory Button")
                                                             .on_press(Message::FromDirPressed),
                                                            text(fromdir_value)
                                                             .size(20)].spacing(10)).width(Length::Fill),
                                                             Space::with_width(Length::Fixed(dirspace)),
                                                             container(row![button("To Directory Button")
                                                             .on_press(Message::ToDirPressed),
                                                            text(todir_value)
                                                             .size(20)].spacing(10)).width(Length::Fill),
                                                           ].align_y(Alignment::Center).spacing(10).padding(5),
                 ));
                let controls = view_controls(images, *filter);

                let filtered_images =
                    images.iter().filter(|imageitem| filter.matches(imageitem));

                let mut imagescol1 = Column::new().spacing(10);
                let mut imagescol2 = Column::new().spacing(10);
                let mut imagescol3 = Column::new().spacing(10);
                let mut imagescol4 = Column::new().spacing(10);
                let mut imagescol5 = Column::new().spacing(10);
                let mut imagescol6 = Column::new().spacing(10);
                let mut imagescol7 = Column::new().spacing(10);
                let mut colpos = 0;
                let mut n = 0;
                if filtered_images.clone().count() == 0 {
                    n = 1;
                    imagescol1 = imagescol1.push(container(row![empty_message(match filter {
                        Filter::All => "No directory selected or no files in directory",
                        Filter::Active => "All files have been selected",
                        Filter::Selected => {
                            "No files have been selected" }
                    })]));
                } else {
                    for imagesy in images.iter() {
                         if imagesy.completed {
                             if (filter == &Filter::All) || (filter == &Filter::Selected) {
                               if colpos == 0 {
                                 imagescol1 = imagescol1.push(container(row![imagesy.view(n).map(move |message| {
                                    Message::ImageMessage(n, message)
                                   })]));
                                 colpos  = 1;
                               } else if colpos == 1 {
                                 imagescol2 = imagescol2.push(container(row![imagesy.view(n).map(move |message| {
                                    Message::ImageMessage(n, message)
                                   })]));
                                 if *col_size < 3 {
                                     colpos = 0;
                                 } else {
                                     colpos = 2;
                                 }
                               } else if colpos == 2 {
                                 imagescol3 = imagescol3.push(container(row![imagesy.view(n).map(move |message| {
                                    Message::ImageMessage(n, message)
                                   })]));
                                 if *col_size < 4 {
                                     colpos = 0;
                                 } else {
                                     colpos = 3;
                                 }
                               } else if colpos == 3 {
                                 imagescol4 = imagescol4.push(container(row![imagesy.view(n).map(move |message| {
                                    Message::ImageMessage(n, message)
                                   })]));
                                 if *col_size < 5 {
                                     colpos = 0;
                                 } else {
                                     colpos = 4;
                                 }
                               } else if colpos == 4 {
                                 imagescol5 = imagescol5.push(container(row![imagesy.view(n).map(move |message| {
                                    Message::ImageMessage(n, message)
                                   })]));
                                 if *col_size < 6 {
                                     colpos = 0;
                                 } else {
                                     colpos = 5;
                                 }
                               } else if colpos == 5 {
                                 imagescol6 = imagescol6.push(container(row![imagesy.view(n).map(move |message| {
                                    Message::ImageMessage(n, message)
                                   })]));
                                 if *col_size < 7 {
                                     colpos = 0;
                                 } else {
                                     colpos = 6;
                                 }
                               } else if colpos == 6 {
                                 imagescol7 = imagescol7.push(container(row![imagesy.view(n).map(move |message| {
                                    Message::ImageMessage(n, message)
                                   })]));
                                 colpos = 0;
                               }
                             }
                        } else {
                             if (filter == &Filter::All) || (filter == &Filter::Active) {
                               if colpos == 0 {
                                 imagescol1 = imagescol1.push(container(row![imagesy.view(n).map(move |message| {
                                    Message::ImageMessage(n, message)
                                   })]));
                                 colpos  = 1;
                               } else if colpos == 1 {
                                 imagescol2 = imagescol2.push(container(row![imagesy.view(n).map(move |message| {
                                    Message::ImageMessage(n, message)
                                   })]));
                                 if *col_size < 3 {
                                     colpos = 0;
                                 } else {
                                     colpos = 2;
                                 }
                               } else if colpos == 2 {
                                 imagescol3 = imagescol3.push(container(row![imagesy.view(n).map(move |message| {
                                    Message::ImageMessage(n, message)
                                   })]));
                                 if *col_size < 4 {
                                     colpos = 0;
                                 } else {
                                     colpos = 3;
                                 }
                               } else if colpos == 3 {
                                 imagescol4 = imagescol4.push(container(row![imagesy.view(n).map(move |message| {
                                    Message::ImageMessage(n, message)
                                   })]));
                                 if *col_size < 5 {
                                     colpos = 0;
                                 } else {
                                     colpos = 4;
                                 }
                               } else if colpos == 4 {
                                 imagescol5 = imagescol5.push(container(row![imagesy.view(n).map(move |message| {
                                    Message::ImageMessage(n, message)
                                   })]));
                                 if *col_size < 6 {
                                     colpos = 0;
                                 } else {
                                     colpos = 5;
                                 }
                               } else if colpos == 5 {
                                 imagescol6 = imagescol6.push(container(row![imagesy.view(n).map(move |message| {
                                    Message::ImageMessage(n, message)
                                   })]));
                                 if *col_size < 7 {
                                     colpos = 0;
                                 } else {
                                     colpos = 6;
                                 }
                               } else if colpos == 6 {
                                 imagescol7 = imagescol7.push(container(row![imagesy.view(n).map(move |message| {
                                    Message::ImageMessage(n, message)
                                   })]));
                                 colpos = 0;
                               }
                             }
                         }
                         n = n + 1;
                    }
                }
                let imagwid = screenwidth / *col_size as f32;
                let mut imagesrow = Row::new().spacing(20);
                imagesrow = imagesrow.push(container(imagescol1).padding(10).width(Length::Fixed(imagwid)));
                if n > 1 && *col_size > 1 {
                    imagesrow = imagesrow.push(container(imagescol2).padding(10).width(Length::Fixed(imagwid)));
                    if n > 2 && *col_size > 2 {
                       imagesrow = imagesrow.push(container(imagescol3).padding(10).width(Length::Fixed(imagwid)));
                       if n > 3 && *col_size > 3 {
                           imagesrow = imagesrow.push(container(imagescol4).padding(10).width(Length::Fixed(imagwid)));
                           if n > 4 && *col_size > 4 {
                               imagesrow = imagesrow.push(container(imagescol5).padding(10).width(Length::Fixed(imagwid)));
                               if n > 5 && *col_size > 5 {
                                   imagesrow = imagesrow.push(container(imagescol6).padding(10).width(Length::Fixed(imagwid)));
                                   if n > 6 && *col_size > 6 {
                                       imagesrow = imagesrow.push(container(imagescol7).padding(10).width(Length::Fixed(imagwid)));
                                   }
                               }
                           }
                       }
                    }
                }

                let scrollable_content: Element<Message> =
                  Element::from(scrollable(
                    imagesrow
                )
                .height(Length::Fill)
                .direction({
                    let scrollbar = scrollable::Scrollbar::new()
                        .width(10)
                        .margin(10)
                        .scroller_width(10);

                    scrollable::Direction::Both {
                        horizontal: scrollbar,
                        vertical: scrollbar,
                    }
                 })
                ); 

                let targetfolder = format!("pic{}{}{}", folderyear, foldermonth, folderday);
                let contentab = row![
                                     button("Refresh").on_press(Message::RefreshPressed).padding(5),
                                     text(" Year From: ").size(20),
                                     text_input("2025", fromyear_value).on_input(Message::FromYear).padding(5).size(20).width(60),
                                     horizontal_space(), 
                                     text("target folder:").size(20),
                                     text(targetfolder).size(20),
                                     horizontal_space(), 
                                     button("Copy").on_press(Message::CopyPressed).padding(5),
                                    ].spacing(10).padding(5);

                let winwidth: f32 = screenwidth - 20.0;

                let selected_yearchoice = Some(yearchoice_value);
                let ya = Radio::new(
                         "No year",
                         YearChoice::NON,
                         selected_yearchoice.copied(),
                         Message::YearRadioSelected,
                ).size(15);
                let yb = Radio::new(
                         year01,
                         YearChoice::YR1,
                         selected_yearchoice.copied(),
                         Message::YearRadioSelected,
                ).size(15);
           
                let yc = Radio::new(
                         year02,
                         YearChoice::YR2,
                         selected_yearchoice.copied(),
                         Message::YearRadioSelected,
                ).size(15);
           
                let yd = Radio::new(
                           year03,
                           YearChoice::YR3,
                           selected_yearchoice.copied(),
                           Message::YearRadioSelected
                ).size(15);

                let ye = Radio::new(
                           year04,
                           YearChoice::YR4,
                           selected_yearchoice.copied(),
                           Message::YearRadioSelected
                ).size(15);

                let yf = Radio::new(
                           year05,
                           YearChoice::YR5,
                           selected_yearchoice.copied(),
                           Message::YearRadioSelected
                ).size(15);

                let selected_monthchoice = Some(monthchoice_value);

                let m00 = Radio::new(
                         "No month",
                         MonthChoice::Non,
                         selected_monthchoice.copied(),
                         Message::MonthRadioSelected,
                ).size(15);
                let m01 = Radio::new(
                         "Jan",
                         MonthChoice::Jan,
                         selected_monthchoice.copied(),
                         Message::MonthRadioSelected,
                ).size(15);
                let m02 = Radio::new(
                         "Feb",
                         MonthChoice::Feb,
                         selected_monthchoice.copied(),
                         Message::MonthRadioSelected,
                ).size(15);
                let m03 = Radio::new(
                         "Mar",
                         MonthChoice::Mar,
                         selected_monthchoice.copied(),
                         Message::MonthRadioSelected,
                ).size(15);
                let m04 = Radio::new(
                         "Apr",
                         MonthChoice::Apr,
                         selected_monthchoice.copied(),
                         Message::MonthRadioSelected,
                ).size(15);
                let m05 = Radio::new(
                         "May",
                         MonthChoice::May,
                         selected_monthchoice.copied(),
                         Message::MonthRadioSelected,
                ).size(15);
                let m06 = Radio::new(
                         "Jun",
                         MonthChoice::Jun,
                         selected_monthchoice.copied(),
                         Message::MonthRadioSelected,
                ).size(15);
                let m07 = Radio::new(
                         "Jul",
                         MonthChoice::Jul,
                         selected_monthchoice.copied(),
                         Message::MonthRadioSelected,
                ).size(15);
                let m08 = Radio::new(
                         "Aug",
                         MonthChoice::Aug,
                         selected_monthchoice.copied(),
                         Message::MonthRadioSelected,
                ).size(15);
                let m09 = Radio::new(
                         "Sep",
                         MonthChoice::Sep,
                         selected_monthchoice.copied(),
                         Message::MonthRadioSelected,
                ).size(15);
                let m10 = Radio::new(
                         "Oct",
                         MonthChoice::Oct,
                         selected_monthchoice.copied(),
                         Message::MonthRadioSelected,
                ).size(15);
                let m11 = Radio::new(
                         "Nov",
                         MonthChoice::Nov,
                         selected_monthchoice.copied(),
                         Message::MonthRadioSelected,
                ).size(15);
                let m12 = Radio::new(
                         "Dec",
                         MonthChoice::Dec,
                         selected_monthchoice.copied(),
                         Message::MonthRadioSelected,
                ).size(15);

                let selected_daychoice = Some(daychoice_value);

                let d00 = Radio::new(
                         "no day",
                         DayChoice::Non,
                         selected_daychoice.copied(),
                         Message::DayRadioSelected,
                ).size(15);
                let d01 = Radio::new(
                         "01",
                         DayChoice::D01,
                         selected_daychoice.copied(),
                         Message::DayRadioSelected,
                ).size(15);
                let d02 = Radio::new(
                         "02",
                         DayChoice::D02,
                         selected_daychoice.copied(),
                         Message::DayRadioSelected,
                ).size(15);
                let d03 = Radio::new(
                         "03",
                         DayChoice::D03,
                         selected_daychoice.copied(),
                         Message::DayRadioSelected,
                ).size(15);
                let d04 = Radio::new(
                         "04",
                         DayChoice::D04,
                         selected_daychoice.copied(),
                         Message::DayRadioSelected,
                ).size(15);
                let d05 = Radio::new(
                         "05",
                         DayChoice::D05,
                         selected_daychoice.copied(),
                         Message::DayRadioSelected,
                ).size(15);
                let d06 = Radio::new(
                         "06",
                         DayChoice::D06,
                         selected_daychoice.copied(),
                         Message::DayRadioSelected,
                ).size(15);
                let d07 = Radio::new(
                         "07",
                         DayChoice::D07,
                         selected_daychoice.copied(),
                         Message::DayRadioSelected,
                ).size(15);
                let d08 = Radio::new(
                         "08",
                         DayChoice::D08,
                         selected_daychoice.copied(),
                         Message::DayRadioSelected,
                ).size(15);
                let d09 = Radio::new(
                         "09",
                         DayChoice::D09,
                         selected_daychoice.copied(),
                         Message::DayRadioSelected,
                ).size(15);
                let d10 = Radio::new(
                         "10",
                         DayChoice::D10,
                         selected_daychoice.copied(),
                         Message::DayRadioSelected,
                ).size(15);
                let d11 = Radio::new(
                         "11",
                         DayChoice::D11,
                         selected_daychoice.copied(),
                         Message::DayRadioSelected,
                ).size(15);
                let d12 = Radio::new(
                         "12",
                         DayChoice::D12,
                         selected_daychoice.copied(),
                         Message::DayRadioSelected,
                ).size(15);
                let d13 = Radio::new(
                         "13",
                         DayChoice::D13,
                         selected_daychoice.copied(),
                         Message::DayRadioSelected,
                ).size(15);
                let d14 = Radio::new(
                         "14",
                         DayChoice::D14,
                         selected_daychoice.copied(),
                         Message::DayRadioSelected,
                ).size(15);
                let d15 = Radio::new(
                         "15",
                         DayChoice::D15,
                         selected_daychoice.copied(),
                         Message::DayRadioSelected,
                ).size(15);
                let d16 = Radio::new(
                         "16",
                         DayChoice::D16,
                         selected_daychoice.copied(),
                         Message::DayRadioSelected,
                ).size(15);
                let d17 = Radio::new(
                         "17",
                         DayChoice::D17,
                         selected_daychoice.copied(),
                         Message::DayRadioSelected,
                ).size(15);
                let d18 = Radio::new(
                         "18",
                         DayChoice::D18,
                         selected_daychoice.copied(),
                         Message::DayRadioSelected,
                ).size(15);
                let d19 = Radio::new(
                         "19",
                         DayChoice::D19,
                         selected_daychoice.copied(),
                         Message::DayRadioSelected,
                ).size(15);
                let d20 = Radio::new(
                         "20",
                         DayChoice::D20,
                         selected_daychoice.copied(),
                         Message::DayRadioSelected,
                ).size(15);
                let d21 = Radio::new(
                         "21",
                         DayChoice::D21,
                         selected_daychoice.copied(),
                         Message::DayRadioSelected,
                ).size(15);
                let d22 = Radio::new(
                         "22",
                         DayChoice::D22,
                         selected_daychoice.copied(),
                         Message::DayRadioSelected,
                ).size(15);
                let d23 = Radio::new(
                         "23",
                         DayChoice::D23,
                         selected_daychoice.copied(),
                         Message::DayRadioSelected,
                ).size(15);
                let d24 = Radio::new(
                         "24",
                         DayChoice::D24,
                         selected_daychoice.copied(),
                         Message::DayRadioSelected,
                ).size(15);
                let d25 = Radio::new(
                         "25",
                         DayChoice::D25,
                         selected_daychoice.copied(),
                         Message::DayRadioSelected,
                ).size(15);
                let d26 = Radio::new(
                         "26",
                         DayChoice::D26,
                         selected_daychoice.copied(),
                         Message::DayRadioSelected,
                ).size(15);
                let d27 = Radio::new(
                         "27",
                         DayChoice::D27,
                         selected_daychoice.copied(),
                         Message::DayRadioSelected,
                ).size(15);
                let d28 = Radio::new(
                         "28",
                         DayChoice::D28,
                         selected_daychoice.copied(),
                         Message::DayRadioSelected,
                ).size(15);
                let d29 = Radio::new(
                         "29",
                         DayChoice::D29,
                         selected_daychoice.copied(),
                         Message::DayRadioSelected,
                ).size(15);
                let d30 = Radio::new(
                         "30",
                         DayChoice::D30,
                         selected_daychoice.copied(),
                         Message::DayRadioSelected,
                ).size(15);
                let d31 = Radio::new(
                         "31",
                         DayChoice::D31,
                         selected_daychoice.copied(),
                         Message::DayRadioSelected,
                ).size(15);

                  let columntob = column![row![ya, yb, yc, yd, ye, yf].spacing(20),
                                          row![m00, m01, m02, m03, m04, m05, m06, m07, m08, m09, m10, m11, m12].spacing(20),
                                          row![d00, d01, d02, d03, d04, d05, d06, d07, d08, d09].spacing(20),
                                          row![d10, d11, d12, d13, d14, d15, d16, d17, d18, d19].spacing(20),
                                          row![d20, d21, d22, d23, d24, d25, d26, d27, d28, d29, d30, d31].spacing(20),
                                   ].width(Length::Fill).spacing(5);

                   column![messcol, dirbutshow, contentab, columntob, 
                           row![controls,
                                Space::with_width(Length::Fixed(20.0)),
                                text(" Icon Size: ").size(20),
                                text_input("140", size_value).on_input(Message::SizeChanged).padding(5).size(20).width(60),
                                Space::with_width(Length::Fixed(20.0)),
                                button("Reload Images").on_press(Message::ReloadPressed).padding(5),
                                horizontal_space(), 
                                button("Set to Display Date").on_press(Message::SetDatePressed).padding(5),
                          ], scrollable_content]
                         .spacing(10)
                         .max_width(winwidth)
                         .padding(5)
                         .into()
            }
        }
    }
    fn theme(&self) -> Theme {
         Theme::Dracula
    }

    fn subscription(&self) -> Subscription<Message> {
        event::listen_with(|event, _status, _window| match event {
            Event::Window(window::Event::Resized(size)) => {
                Some(Message::Size(size))
            }
            _ => None,
        })
    }

}

#[derive(Debug, Clone)]
struct ImageItem {
    description: String,
    completed: bool,
    rgbconv: Vec<u8>,
    twidth: u32,
    theight: u32,
}

#[derive(Debug, Clone)]
pub enum ImageMessage {
    Selected(bool),
}

impl ImageItem {

    fn new(description: String, rgbconv: Vec<u8>, twidth:  u32, theight: u32,) -> Self {
        ImageItem {
            description,
            completed: false,
            rgbconv,
            twidth,
            theight,
        }
    }

    fn update(&mut self, message: ImageMessage) {
        match message {
            ImageMessage::Selected(completed) => {
                self.completed = completed;
            }
        }
    }

    fn view(&self, _i: usize) -> Element<ImageMessage> {
        let checkbox = checkbox(
            &self.description,
            self.completed).on_toggle(ImageMessage::Selected).width(Length::Fill).text_size(12);
        let newimage = image::Handle::from_rgba(self.twidth.clone(), self.theight.clone(), self.rgbconv.clone()); 
        let colhigh: f32;
        if self.twidth > self.theight {
            colhigh = self.twidth as f32 + 10.0;
        } else {
            colhigh = self.theight as f32 + 10.0;
        }
        column![
           container(
        // This should go away once we unify resource loading on native
        // platforms
             image::Viewer::new(newimage)
                 .height(Length::Fill)
                 .width(Length::Fill),
           )
           .width(Length::Fill),
            checkbox,
        ]
        .align_x(Alignment::Center)
        .height(Length::Fixed(colhigh))
        .spacing(5)
        .into()

    }
}

fn view_controls(images: &[ImageItem], current_filter: Filter) -> Element<Message> {
    let images_left = images.iter().filter(|imageitem| imageitem.completed).count();

    let filter_button = |label, filter, current_filter| {
        let label = text(label).size(16);

        let button = button(label).style(if filter == current_filter {
            button::primary
        } else {
            button::text
        });

        button.on_press(Message::FilterChanged(filter)).padding(5)
    };

    row![Space::with_width(Length::Fixed(20.0)),
        text(format!(
            "{} {} selected",
            images_left,
            if images_left == 1 { "file" } else { "files" }
        ))
        .size(16),
        row![
            filter_button("All", Filter::All, current_filter),
            filter_button("Not Selected", Filter::Active, current_filter),
            filter_button("Selected", Filter::Selected, current_filter,),
        ]
        .width(Length::Shrink)
        .spacing(10)
    ]
    .spacing(20)
    .align_y(Alignment::Center)
    .into()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Filter {
    All,
    Active,
    Selected,
}

impl Default for Filter {
    fn default() -> Self {
        Filter::All
    }
}

impl Filter {
    fn matches(&self, imageitem: &ImageItem) -> bool {
        match self {
            Filter::All => true,
            Filter::Active => !imageitem.completed,
            Filter::Selected => imageitem.completed,
        }
    }
}

fn empty_message(message: &str) -> Element<'_, Message> {
    container(
        text(message)
            .width(Length::Fill)
            .size(25)
            .align_x(Center)
            .color([0.7, 0.7, 0.7]),
    )
    .width(Length::Fill)
    .height(Length::Fixed(200.0))
    .into()
}

#[derive(Debug, Clone)]
pub struct Copymovex {
    errcolor: Color,
    errval: String,
}

impl Copymovex {

    pub async fn copymoveit(listofimages: Vec<String>, fromstr: String, tostr: String, yearv: String, monthv: String, dayv: String,) -> Result<Copymovex, Error> {
        let start_time = Instant::now();
        let lenmg1 = listofimages.len();
        for indl in 0..lenmg1 {
             let str_cur_dirfrom = fromstr.clone();
             let fullfrom = str_cur_dirfrom.clone() + "/" + &listofimages[indl].clone();
             let fulldone = str_cur_dirfrom.clone() + "/done";
             if !Path::new(&fulldone).exists() {
                 let _output = stdCommand::new("mkdir")
                               .arg(&fulldone)
                               .output()
                               .expect("failed to execute process");
             }
             let fulltodir = tostr.clone() + "/pic" + &yearv.clone() + &monthv.clone() + &dayv.clone();
             if !Path::new(&fulltodir).exists() {
                 let _output = stdCommand::new("mkdir")
                               .arg(&fulltodir)
                               .output()
                               .expect("failed to execute process");
             }
             let fullto = fulltodir + "/" + &listofimages[indl].clone();
             let _output = stdCommand::new("cp")
                                         .arg("-p")
                                         .arg(&fullfrom)
                                         .arg(&fullto)
                                         .output()
                                         .expect("failed to execute process");
             let _output = stdCommand::new("mv")
                                         .arg(&fullfrom)
                                         .arg(&fulldone)
                                         .output()
                                         .expect("failed to execute process");
        }
         let diffx = start_time.elapsed();     
         let errstring = format!("copied and moved {} files in {} seconds", lenmg1, diffx.as_secs());
         let colorx = Color::from([0.0, 1.0, 0.0]);
         Ok(Copymovex {
            errcolor: colorx,
            errval: errstring,
        })
    }
}
#[derive(Debug, Clone)]
pub enum Error {
//    APIError,
}
