#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

extern crate home;

extern crate image;

use std::path::Path;

use std::thread;

use std::fs;

use std::sync::Arc;

use image::GenericImageView;

use image::imageops::FilterType::Gaussian;

use rocket_contrib::json::Json;

use rocket::Data;

use rocket::http::ContentType;

use rand::{distributions::Alphanumeric, Rng};

use serde::{Deserialize, Serialize};

use rocket_multipart_form_data::{
    MultipartFormData, MultipartFormDataField, MultipartFormDataOptions,
};

use rocket::http::Method;

use rocket_cors::{AllowedOrigins, CorsOptions};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[derive(Serialize, Deserialize)]
struct ResponseData {
    filename: String,
}

#[derive(Serialize, Deserialize)]
struct ResponseMeta {
    message: String,
}

#[derive(Serialize, Deserialize)]
struct ResponseUpload {
    success: bool,
    data: ResponseData,
    meta: ResponseMeta,
}

fn compress_image(
    path: String,
    width: u32,
    height: u32,
    _suffix: &str,
    custom_path_file: String,
    file_random: String,
) {
    let img_clone = image::open(path).unwrap();
    let img_main = img_clone.resize(width, height, Gaussian);

    let mut main_file_name: String = file_random.clone();
    main_file_name.push_str("-");
    main_file_name.push_str(_suffix);
    main_file_name.push_str(".");
    main_file_name.push_str("jpg");
    let custom_file = custom_path_file.clone();
    let _absolute_path: String = format!("{}/{}", custom_file, main_file_name);

    img_main.save(_absolute_path).unwrap();
}

fn upload_image(
    _id: String,
    compress: Option<String>,
    folder: Option<String>,
    content_type: &ContentType,
    form_data: Data,
    type_file: String,
) -> ResponseUpload {
    let mut respon = ResponseUpload {
        success: false,
        data: ResponseData {
            filename: "".to_string(),
        },
        meta: ResponseMeta {
            message: "Failed".to_string(),
        },
    };

    let mut options = MultipartFormDataOptions::new();

    options.allowed_fields = vec![MultipartFormDataField::file("file")];

    let multipart_form_data = MultipartFormData::parse(content_type, form_data, options);

    match multipart_form_data {
        Ok(form) => {
            match form.files.get("file") {
                Some(img) => {
                    let now = chrono::Utc::now();
                    let file_field = &img[0];
                    let _content_type = &file_field.content_type;
                    let _file_name = &file_field.file_name;
                    let _path = &file_field.path;
                    let file_name_format: Vec<&str> =
                        _file_name.as_ref().unwrap().split('.').collect(); /* Reparsing the fileformat */

                    let _extension = file_name_format[1];

                    let mut list_image = ["fpt", "fgt", "fcp"];

                    if type_file == "image".to_string() {
                        list_image = ["jpg", "jpeg", "png"];
                    } else if type_file == "fingerprint".to_string() {
                        list_image = ["fpt", "fgt", "fcp"];
                    }

                    if !list_image.contains(&_extension) {
                        respon = ResponseUpload {
                            success: false,
                            data: ResponseData {
                                filename: "Failed to read file, please check format".to_string(),
                            },
                            meta: ResponseMeta {
                                message: "Ok".to_string(),
                            },
                        };
                        return respon;
                    }

                    let mut custom_path_home =
                        format!("{}/uploads", home::home_dir().unwrap().display(),);

                    let folder_string = folder
                        .map(|folder| format!("{}", folder))
                        .unwrap_or_else(|| "0".into());

                    if folder_string == "buatbaju" {
                        custom_path_home = format!(
                            "{}/{}/uploads",
                            home::home_dir().unwrap().display(),
                            folder_string
                        );
                    }

                    let custom_path_relative = format!("{}/{}", _id, now.format("%b%Y"));

                    let custom_path = format!("{}/{}", custom_path_home, custom_path_relative);

                    let custom_path_file = custom_path.clone();

                    if !Path::new("path").exists() {
                        let _file_created = fs::create_dir_all(custom_path);
                    }

                    let file_random: String = rand::thread_rng()
                        .sample_iter(&Alphanumeric)
                        .take(36)
                        .map(char::from)
                        .collect();

                    let mut new_file_name: String = file_random.clone();

                    new_file_name.push_str(".");
                    new_file_name.push_str(&_extension);

                    let path_relative: String =
                        format!("uploads/{}/{}", custom_path_relative, new_file_name);

                    let absolute_path = format!("{}/{}", custom_path_file, new_file_name);
                    let thread_path = absolute_path.clone();

                    fs::copy(_path, &absolute_path).unwrap();

                    let compress_string = compress
                        .map(|compress| format!("{}", compress))
                        .unwrap_or_else(|| "0".into());

                    if compress_string == "1" {
                        let abs_path = Arc::new(absolute_path);
                        let custom_path_file_arc = Arc::new(custom_path_file);
                        let file_random_arc = Arc::new(file_random);

                        thread::spawn(move || {
                            let im = image::open(thread_path).unwrap();
                            let pat = Arc::clone(&abs_path);
                            let ptfi = Arc::clone(&custom_path_file_arc);
                            let fra = Arc::clone(&file_random_arc);

                            // MAIN IMAGE
                            compress_image(
                                pat.to_string(),
                                im.dimensions().1 * 70 / 100,
                                im.dimensions().1 * 70 / 100,
                                "main",
                                ptfi.to_string(),
                                fra.to_string(),
                            );

                            // SMALL IMAGE
                            compress_image(
                                pat.to_string(),
                                im.dimensions().1 * 25 / 100,
                                im.dimensions().1 * 25 / 100,
                                "small",
                                ptfi.to_string(),
                                fra.to_string(),
                            );

                            // MEDIUM IMAGE
                            compress_image(
                                pat.to_string(),
                                im.dimensions().1 / 2,
                                im.dimensions().1 / 2,
                                "medium",
                                ptfi.to_string(),
                                fra.to_string(),
                            );

                            // CROPPED IMAGE
                            compress_image(
                                pat.to_string(),
                                250,
                                300,
                                "cropped",
                                ptfi.to_string(),
                                fra.to_string(),
                            );

                            // COMMERCE IMAGE
                            compress_image(
                                pat.to_string(),
                                500,
                                500,
                                "commerce",
                                ptfi.to_string(),
                                fra.to_string(),
                            );
                        });
                    }

                    respon = ResponseUpload {
                        success: true,
                        data: ResponseData {
                            filename: path_relative.clone(),
                        },
                        meta: ResponseMeta {
                            message: "Ok".to_string(),
                        },
                    };

                    Some("respon")
                }
                None => None,
            };
        }
        Err(_) => todo!(),
    };

    respon
}

#[post("/image/upload/<_id>?<compress>&<folder>", data = "<form_data>")]
fn upload_custom_image(
    _id: String,
    compress: Option<String>,
    folder: Option<String>,
    content_type: &ContentType,
    form_data: Data,
) -> Json<ResponseUpload> {
    Json(upload_image(
        _id,
        compress,
        folder,
        content_type,
        form_data,
        "image".to_string(),
    ))
}

#[post("/fingerprint/upload", data = "<form_data>")]
fn upload_fingerprint_image(content_type: &ContentType, form_data: Data) -> Json<ResponseUpload> {
    Json(upload_image(
        "fingerprint".to_string(),
        Some("0".to_string()),
        Some("".to_string()),
        content_type,
        form_data,
        "fingerprint".to_string(),
    ))
}

fn main() {
    let cors = CorsOptions::default()
    .allowed_origins(AllowedOrigins::all())
    .allowed_methods(
        vec![Method::Get, Method::Post, Method::Patch]
            .into_iter()
            .map(From::from)
            .collect(),
    )
    .allow_credentials(true);

    rocket::ignite()
        .attach(cors.to_cors().unwrap())
        .mount(
            "/",
            routes![index, upload_custom_image, upload_fingerprint_image],
        )
        .launch();
}
