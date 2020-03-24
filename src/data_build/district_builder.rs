use std::fs;
use serde::Deserialize;
use serde::Serialize;
use std::borrow::{BorrowMut, Borrow};
use std::ptr::null;
use std::rc::Rc;
use std::cell::RefCell;
use failure::_core::cell::Cell;
use std::io::Error;
use std::env::consts::OS;

/**
todo
1 add polyline field
2 add output result test func
3 vec iter_mut func description
4 improve to multi thread or concurrent

2020-3-18
difference in &str and string
*/

type SingleResult<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub async fn build_district_str(dir: &str) -> SingleResult<DistrictDto> {
    let pathList = walk_dir(dir).await;

    let mut root: DistrictDto = DistrictDto {
        adcode: "".to_string(),
        name: "".to_string(),
        // polyline: "".to_string(),
        center: "".to_string(),
        level: "".to_string(),
        districts: Vec::new(),
    };

    match pathList {
        Err(err) => { Err(err) }
        Ok(pathList) => {
            for path in pathList {
                let content = read_file_content(&path).await;
                let entity = process_entity(content.unwrap()).await;

                println!("current path : {}", path);

                match entity {
                    Err(err) => println!("parse entity error : {}", err),
                    Ok(entity) => {
                        if path.contains("100000") && root.adcode.is_empty() {
                            root = entity;
                        } else {
                            let found = hangNormalProvince(&mut root, &entity.districts[0]);
                            println!("normal found result: {}", found);
                            if found == false {
                                if hangMunicipality(&mut root, &entity.districts[0]) == false {
                                    println!("normal or municipality are failure. {} ", path)
                                }
                            }
                        }
                    }
                }

                println!("------------")
            }

            Ok(root)
            // let outPutFile = dir.to_string() + "1_all.json";
            //
            // fs::write(outPutFile, serde_json::to_string(&root).unwrap());
        }
    }
}

pub async fn save_json_file(path: &str, content: &DistrictDto) {
    fs::write(path, serde_json::to_string(&content).unwrap());
}

pub async fn walk_dir(dir: &str) -> SingleResult<Vec<String>> {
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

    Ok(pathList)
}

fn hangNormalProvince(root: &mut DistrictDto, node: &DistrictDto) -> bool {
    let mut found = false;

    for district in root.districts.iter_mut() {

        //println!("root name: {}",district.name);
        if district.name == node.name && district.level == node.level {
            println!("normal found : {} , {}", district.name, &node.name);

            district.districts = node.districts.to_vec();

            // if (node.polyline.is_empty() == false) {
            //     district.polyline = node.polyline.to_string();
            // }

            found = true;
            break;
        } else {
            found = hangNormalProvince(district.borrow_mut(), node);

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
            found = hangMunicipality(district.borrow_mut(), node);

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
    pub name: String,
    // #[serde(default)]
    // polyline: String,
    #[serde(default)]
    center: String,
    #[serde(default)]
    level: String,
    #[serde(default)]
    pub districts: Vec<DistrictDto>,
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

#[cfg(test)]
mod tests {
    use crate::data_build::district_builder::{build_district_str, DistrictDto, save_json_file};
    use std::borrow::Borrow;
    use failure::_core::cmp::max;
    use std::fs;

    #[tokio::test]
    async fn test1() {
        test2().await;

        let dto = build_district_str("district_data/").await;

        match dto {
            Err(err) => panic!("{}", err),
            Ok(dto) => {
                let level = checkDistrictLevel(&dto.districts[0], 1);
                println!("Level : {}", level);

                // assert!(2==level, "level must be 2");

                println!("frist level : {}", dto.name);
                println!("second level : {}", dto.districts.len());
                println!("third level : {}", dto.districts[0].districts[0].name);

                save_json_file("district_data/1_all.json", &dto).await;

                // test3(&dto);

                println!("test ad code {}",dto.adcode)
            }
        }
    }

    fn test3(content: &DistrictDto){
        println!("test ad code {}",content.adcode)
    }

    async fn test2() {
        println!("test2");
    }

    fn checkDistrictLevel(dto: &DistrictDto, mut level: i32) -> i32 {
        let mut childLevel = level;
        let mut maxChildLevel = level;

        if dto.districts.len() > 0 {
            level += 1;

            for district in dto.districts.iter() {
                childLevel = checkDistrictLevel(&district, level);

                if (maxChildLevel < childLevel) {
                    maxChildLevel = childLevel;
                }

                if level == 2 && childLevel < maxChildLevel {
                    maxChildLevel = childLevel
                }

                // if(level==2 && district.name.contains("北京")){
                //     println!(" 2 {}",childLevel);
                // }
                if level == 2 {
                    println!("Level: {} , child : {} ,name: {} , code :{}", level, maxChildLevel, district.name, district.adcode);
                }
            }
        }

        maxChildLevel
    }
}



