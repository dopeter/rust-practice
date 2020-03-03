use std::fs;
use serde::Deserialize;
use serde::Serialize;
use std::borrow::{BorrowMut, Borrow};
use std::ptr::null;
use std::rc::Rc;
use std::cell::RefCell;
use failure::_core::cell::Cell;

/**
todo
1 add polyline field
2 add output result test func
3 vec iter_mut func description
4 improve to multi thread or concurrent
*/

type SingleResult<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub async fn walk_dir(dir: &str) {
    let mut pathList = Vec::new();

    for entry in (fs::read_dir(dir)).unwrap() {
        match entry {
            Err(err) => println!("err : {}", err),
            Ok(dir) => {
                println!("file name: {} , {}", dir.path().to_str().unwrap(), dir.file_name().into_string().unwrap());
                pathList.push(dir.path().to_str().unwrap().to_string());
            }
        }
    }

    pathList.sort();

    let mut root: DistrictDto = DistrictDto {
        adcode: "".to_string(),
        name: "".to_string(),
        // polyline: "".to_string(),
        center: "".to_string(),
        level: "".to_string(),
        districts: Vec::new(),
    };

    for path in pathList {
        let content = read_file_content(&path).await;
        let entity = process_entity(content.unwrap()).await;
        ;

        println!("current path : {}", path);

        match entity {
            Err(err) => println!("parse entity error : {}", err),
            Ok(entity) => {
                if (path.contains("100000") && root.adcode.is_empty()) {
                    root = entity;
                } else {
                    let found=hangNormalProvince(&mut root, &entity.districts[0]);
                    println!("normal found result: {}", found);
                    if  found== false {
                        if hangMunicipality(&mut root, &entity.districts[0]) == false {
                            println!("normal or municipality are failure. {} ",path)
                        }
                    }
                }
            }
        }

        // let json = serde_json::to_string(&root);
        //
        // println!("json result : {}", json.unwrap());
        println!("------------")
    }

    fs::write("district_data/1_all.json", serde_json::to_string(&root).unwrap());
}

fn hangNormalProvince(root: &mut DistrictDto, node: &DistrictDto) -> bool {
    let mut found=false;

    for district in root.districts.iter_mut() {

        //println!("root name: {}",district.name);
        if district.name == node.name && district.level == node.level {
            println!("normal found : {} , {}", district.name, &node.name);

            district.districts = node.districts.to_vec();

            // if (node.polyline.is_empty() == false) {
            //     district.polyline = node.polyline.to_string();
            // }

            found=true;
            break;

        } else {
            found= hangNormalProvince(district.borrow_mut(), node);

            if found {
                break;
            }

        }
    }

    return found;
}

fn hangMunicipality(root: &mut DistrictDto, node: &DistrictDto) -> bool {
    let mut found = false;

    for mut district in root.districts.iter_mut() {
        if (&district.adcode[0..4] == &node.adcode[0..4]) {
            println!("municipality found : {} , {}", district.name, &node.name);

            district.districts.push(node.to_owned());

            found = true;
            break;
        } else {
            found=hangMunicipality(district.borrow_mut(), node);

            if found {
                break;
            }
        }
    }

    return found;
}


pub async fn read_file_content(path: &String) -> SingleResult<Box<String>> {
    let contents = fs::read_to_string(path)?;

    Ok(Box::new(contents))
}

pub async fn process_entity(content: Box<String>) -> SingleResult<DistrictDto> {
    let district_info: DistrictDto = serde_json::from_str(&content).unwrap();

    // println!("file content: {}", serde_json::to_string(&district_info).unwrap());

    Ok(district_info)
}

#[derive(Serialize, Deserialize)]
pub struct DistrictDto {
    #[serde(default)]
    adcode: String,
    #[serde(default)]
    name: String,
    // #[serde(default)]
    // polyline: String,
    #[serde(default)]
    center: String,
    #[serde(default)]
    level: String,
    #[serde(default)]
    districts: Vec<DistrictDto>,
}

impl Clone for DistrictDto {
    fn clone(&self) -> Self {
        return DistrictDto {
            adcode: self.adcode.clone(),
            name: self.name.clone(),
            // polyline: self.polyline.clone(),
            center: self.center.clone(),
            level: self.level.clone(),
            districts: self.districts.clone(),
        };
    }
}
