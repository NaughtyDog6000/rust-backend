use axum::{Extension, Json, Router, routing::get, routing::post, http::{StatusCode, HeaderMap}, response::{IntoResponse, Response}, extract::{Query, Multipart}};
use log::{warn, info, trace, error};
use serde::Deserialize;
use sqlx::{pool, PgPool};
use serde_json::{json, Value};
use std::{io::Write, fs};
use uuid::Uuid;
use crate::errors::{handle_error, CustomErrors};





pub fn router() -> Router {
    Router::new()
    .route("/admin/achievements/create", get(get_admin_create_achievement).post(admin_create_achievement))

}

pub async fn get_admin_create_achievement() -> axum::response::Html<&'static str> {
    include_str!("../html/create_achievement_form.html").into()
}

#[derive(Debug)]
struct Achievement {
    pub name: Option<String>,
    pub description: Option<String>,
    pub tier: Option<i32>,
    pub unlock_code: Option<String>,
}


async fn admin_create_achievement(
    Extension(pool): Extension<PgPool>,
    Extension(admin_key): Extension<String>,
    headers: HeaderMap,
    mut multipart: Multipart
) {
    let mut authorised: bool = false;
    let mut image_data: Option<Vec<u8>> = None;
    let mut achievement: Achievement = Achievement { name: None, description: None, tier: None, unlock_code: None };

    println!("daowjdpaw");
    // THE NEXT 3 LINES ARE taken from: https://stackoverflow.com/q/76297891/21365457
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        // println!("Length of `{}` is {} bytes", name, data.len());
        match name.as_str() {
            "admin_code" => {
                let temp = String::from_utf8(data.to_vec());
                match temp {
                    Ok(formadmincode) => {
                        if formadmincode != admin_key {
                            warn!("an attempt at creating a form with the wrong admin code was made");
                            // todo!() Proper auth for admins
                        } else {
                            authorised = true;
                        }
                    },
                    Err(e) => {
                        todo!()
                    }
                }
            },
            "icon" => {

                image_data = Some(data.to_vec());

                // todo!()
            },
            "name" => {
                let temp = String::from_utf8(data.to_vec());
                match temp {
                    Ok(name) => {
                        achievement.name = Some(name);
                    },
                    Err(_) => todo!(),
                }
            },
            "description" => {
                let temp = String::from_utf8(data.to_vec());
                match temp {
                    Ok(description) => {
                        achievement.description = Some(description);
                    },
                    Err(_) => todo!(),
                }

            },
            "tier" => {
                let temp = String::from_utf8(data.to_vec());
                if temp.is_err() {
                    todo!()
                }
                let tier = temp.unwrap().parse::<i32>();
                match tier {
                    Ok(tier) => {
                        achievement.tier = Some(tier);
                    },
                    Err(e) => todo!(),
                }
            },
            _ => {
                error!("an unknown field was found in the form");
                return;
            }
        }
    }

    if image_data.is_none() {
        return ;
    }
    println!("achievement generated from the form: {:?}", achievement);
    match authorised {
        true => {
            let result = write_achievement_image_file(image_data.unwrap());
            if result.is_err() {
                todo!();
            }



        },
        false => todo!(),
    }


    return;
}


fn write_achievement_image_file(data: Vec<u8>) -> Result<String, CustomErrors>  {
    let filepath = dotenv::var("IMAGESTOREPATH").expect("image path .env not set");
                
    let filename = Uuid::new_v4().to_string() + ".png";
    
    let fullpath = format!("{}{}", filepath, filename);

    let mut file = fs::File::create(fullpath);
    match file {
        Err(e) => {
            return Err(CustomErrors::FileError);
        },
        Ok(mut file) => {
            file.write_all(&data).expect("file writing failed");
            return Ok(filename);
        },
    }
}

fn upload_achievement_to_database(
    pool: PgPool,
    achievement: Achievement
) -> Result<(), CustomErrors> {
    // -- verify that the required info is present --

    // -- 




    return Ok(());
}