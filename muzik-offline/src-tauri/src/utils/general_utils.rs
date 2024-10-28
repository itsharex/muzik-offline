use dirs::home_dir;
use image::imageops::FilterType;
use std::{path::Path, io::Cursor};
use std::path::PathBuf;
use rayon::prelude::*;
use base64::{Engine as _, engine::general_purpose};

use crate::constants::null_cover_four::NULL_COVER_FOUR;
use crate::constants::null_cover_null::NULL_COVER_NULL;
use crate::constants::null_cover_one::NULL_COVER_ONE;
use crate::constants::null_cover_three::NULL_COVER_THREE;
use crate::constants::null_cover_two::NULL_COVER_TWO;

pub fn duration_to_string(duration: &u64) -> String {
    let seconds = duration;
    let minutes = seconds / 60;
    let seconds = seconds % 60;
    let hours = minutes / 60;
    let minutes = minutes % 60;

    if hours > 0 {
        format!("{}:{:02}:{:02}", hours, minutes, seconds)
    } else {
        format!("{}:{:02}", minutes, seconds)
    }
}

pub fn extract_file_name(file_path: &str) -> String {
    let path = Path::new(file_path);

    match path.file_stem(){
        Some(file_name) => {
            file_name.to_string_lossy().to_string()
        },
        None => {
            String::from("Unknown file name")
        },
    }
}

pub fn resize_and_compress_image(original_data: &Vec<u8>, target_height: &u32) -> Option<Vec<u8>> {
    // Decode the original image
    match image::load_from_memory(&original_data){
        Ok(original_image) => {
            // Calculate the corresponding width to maintain aspect ratio
            let aspect_ratio = original_image.width() as f32 / original_image.height() as f32;
            let target_width = (*target_height as f32 * aspect_ratio) as u32;

            // Resize the image to a specific size (e.g., 250x250 pixels)
            let resized_image = original_image.resize_exact(target_width, *target_height, FilterType::Triangle);

            // Create a buffer to store the compressed image
            let mut compressed_buffer = Cursor::new(Vec::new());

            // Encode the resized image as JPEG with a certain quality
            match resized_image.write_to(&mut compressed_buffer, image::ImageOutputFormat::Jpeg(80)){
                Ok(_) => {
                    // Return the compressed image data
                    Some(compressed_buffer.into_inner())
                },
                Err(_) => {
                    None
                },
            }
        },
        Err(_) => {
            None
        },
    }
}

pub fn encode_image_in_parallel(image_as_vec: &Vec<u8>) -> String{
    let base64str = image_as_vec
        .par_chunks(51) // Chunk size must always be a multiple of 3 otherwise it will not work
        .map(|chunk| general_purpose::STANDARD.encode(chunk))
        .collect::<Vec<_>>()
        .concat();
    return base64str;
    /*
    FURTHER EXPLANATION FOR CHUNK SIZES:
    This could potentially cause issues because the base64 encoding process involves 
    converting 3 bytes(each u8 in the image_as_vec is a byte) of input data into 4 bytes of output data. If a chunk doesn’t 
    contain a multiple of 3 bytes, the base64 encoding for that chunk will be padded 
    with one or two “=” characters. When you concatenate the encoded chunks, these 
    padding characters could end up in the middle of the final base64 string, 
    which would make it invalid.

    additionally seeing “=” characters scattered in the middle of a Base64 string is not a good sign.
    In Base64 encoding, “=” is used as a padding character and should only appear at the end of the encoded string. 
    If you’re seeing “=” characters in the middle of your Base64 string, 
    it suggests that something has gone wrong with the encoding process.
     */
}

pub fn decode_image_in_parallel(image_as_string: &String) -> Result<Vec<u8>, String>{
    let base64str = image_as_string.as_bytes();
    let decoded_image = base64str
        .par_chunks(68) // Chunk size must always be a multiple of 4 otherwise it will not work
        .map(|chunk| general_purpose::STANDARD.decode(chunk))
        .collect::<Vec<_>>();

    let mut returnable = Vec::new();
    for chunk in decoded_image.iter(){
        match chunk{
            Ok(chunk) => {
                returnable.extend(chunk);
            },
            Err(e) => {
                return Err(e.to_string());
            }
        }
    }

    return Ok(returnable);
    /*
    FURTHER EXPLANATION FOR CHUNK SIZES:
    This could potentially cause issues because the base64 encoding process involves 
    converting 3 bytes(each u8 in the image_as_vec is a byte) of input data into 4 bytes of output data. If a chunk doesn’t 
    contain a multiple of 3 bytes, the base64 encoding for that chunk will be padded 
    with one or two “=” characters. When you concatenate the encoded chunks, these 
    padding characters could end up in the middle of the final base64 string, 
    which would make it invalid.

    additionally seeing “=” characters scattered in the middle of a Base64 string is not a good sign.
    In Base64 encoding, “=” is used as a padding character and should only appear at the end of the encoded string. 
    If you’re seeing “=” characters in the middle of your Base64 string, 
    it suggests that something has gone wrong with the encoding process.
     */
}

pub fn get_cover_url(cover_data: &Option<String>, key: &i32) -> Option<String> {
    match cover_data{
        Some(cover_data_as_base64) => {
            return save_or_overwrite_image_file(
                String::from("cover.jpg"), 
                &format!("data:image/jpeg;base64,{}", cover_data_as_base64)
            );
        },
        None => {
            match key{
                key if key % 4 == 0 => {
                    return save_if_not_exists_image_file(
                        String::from("nullcoverone.jpg"), 
                        &NULL_COVER_ONE.to_owned()
                    );
                },
                key if key % 4 == 1 => {
                    return save_if_not_exists_image_file(
                        String::from("nullcovertwo.jpg"), 
                        &NULL_COVER_TWO.to_owned()
                    );
                },
                key if key % 4 == 2 => {
                    return save_if_not_exists_image_file(
                        String::from("nullcoverthree.jpg"), 
                        &NULL_COVER_THREE.to_owned()
                    );
                },
                key if key % 4 == 3 => {
                    return save_if_not_exists_image_file(
                        String::from("nullcoverfour.jpg"), 
                        &NULL_COVER_FOUR.to_owned()
                    );
                },
                &_ => {
                    return save_if_not_exists_image_file(
                        String::from("nullcovernull.jpg"), 
                        &NULL_COVER_NULL.to_owned()
                    );
                }
            }
        },
    }
}

pub fn save_or_overwrite_image_file(name: String, image_data: &String) -> Option<String> {
    let mut image_path = PathBuf::new();
    match home_dir() {
        Some(path) => image_path.push(path),
        None => return None,
    }
    image_path.push("muzik-offline-local-data");
    image_path.push("images");
    image_path.push(name);

    match std::fs::write(&image_path, &image_data){
        Ok(_) => {
            match image_path.to_str(){
                Some(path) => {
                    Some(String::from(path))
                },
                None => {
                    None
                },
            }
        },
        Err(_) => {
            None
        },
    }
}

pub fn save_if_not_exists_image_file(name: String, image_data: &String) -> Option<String> {
    let mut image_path = PathBuf::new();
    match home_dir() {
        Some(path) => image_path.push(path),
        None => return None,
    }
    image_path.push("muzik-offline-local-data");
    image_path.push("images");
    image_path.push(name);

    if !image_path.exists(){
        match std::fs::write(&image_path, &image_data){
            Ok(_) => {
                match image_path.to_str(){
                    Some(path) => {
                        Some(String::from(path))
                    },
                    None => {
                        None
                    },
                }
            },
            Err(_) => {
                None
            },
        }
    } else {
        match image_path.to_str(){
            Some(path) => {
                Some(String::from(path))
            },
            None => {
                None
            },
        }
    }
}