use std::path::Path;

pub fn copypressm (listofimages: Vec<String>, fromstr: String, tostr: String, yearv: String, monthv: String, dayv: String) -> (u32, String) {
        let mut errcode: u32 = 0;
        let mut errstring: String = " ".to_string();
        if !Path::new(&fromstr).exists() {
            errstring = "from directory does not exist".to_string();
            errcode = 1;
        } else {
            if !Path::new(&tostr).exists() {
                errstring = "to directory does not exist".to_string();
                errcode = 2;
            } else {
                let year_int: i32 = yearv.parse().unwrap_or(-99);
                if year_int < 1850 {
                    errstring = "year value is invalid: must be numeric and greater than 1849".to_string();
                    errcode = 3;
                } else {
                    let lenmg1 = listofimages.len();
                    if lenmg1 < 1 {
                        errstring = "no images were selected".to_string();
                        errcode = 4;
                    } else {
                        for indl in 0..lenmg1 {
                             let str_cur_dirfrom = fromstr.clone();
                             let fullfrom = str_cur_dirfrom.clone() + "/" + &listofimages[indl].clone();
                             if !Path::new(&fullfrom).exists() {
                                 errstring = format!("********* ERROR {} does not exist **********",fullfrom);
                                 errcode = 5;
                                 break;
                             }
                             let fulldone = str_cur_dirfrom.clone() + "/done";
                             if Path::new(&fulldone).exists() {
                                 let fullfromd = fulldone + "/" + &listofimages[indl].clone();
                                 if Path::new(&fullfromd).exists() {
                                     errstring = format!("********* ERROR {} exist in done folder **********",fullfromd);
                                     errcode = 6;
                                     break;
                                 }
                             }
                             let fullto = tostr.clone() + "/pic" + &yearv.clone() + &monthv.clone() + &dayv.clone() + "/" 
                                                        + &listofimages[indl].clone();
                             if Path::new(&fullto).exists() {
                                 errstring = format!("********* ERROR {} exist in to folder **********",fullto);
                                 errcode = 7;
                                 break;
                             }
                        }
                    }
                }
            }
        }
        (errcode, errstring)
}
