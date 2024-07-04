// use std::fs::File;
// use std::io::Write;
// use std::path::Path;
// use std::{env, fs};

// use chrono::Utc;
// use server_fn::codec::{MultipartData, MultipartFormData};
// use uuid::Uuid;

// use crate::error::{Error, Result};

// pub async fn extract_field_data()

// // pub async fn upload_file(
// //     data: MultipartData,
// //     upload_dir: &str,
// //     form_field: &str,
// //     file_field_name: &str,
// // ) -> Result<String> {
// //     let mut data = data.into_inner().unwrap();

// //     let current_dir = env::current_dir().map_err(|e| Error::Other(e.to_string()))?;
// //     let upload_dir = current_dir.join(upload_dir);
// //     if !upload_dir.exists() {
// //         fs::create_dir_all(upload_dir.clone()).map_err(|e| Error::Other(e.to_string()))?;
// //     }

// //     let mut file_name = String::new();
// //     let mut file_extension = String::new();
// //     let mut file_chunks = Vec::new();

// //     while let Ok(Some(mut field)) = data.next_field().await {
// //         let field_name = field.name().unwrap_or_default().to_string();

// //         if field_name == file_field_name {
// //             slug = field.text().await.unwrap_or_default();
// //             dbg!(slug.clone());

// //         } else if field_name == file_field_name {
// //             file_name = field.file_name().unwrap_or_default().to_string();
// //             file_extension = Path::new(&file_name)
// //                 .extension()
// //                 .unwrap_or_default()
// //                 .to_str()
// //                 .unwrap_or_default()
// //                 .to_string();

// //             while let Ok(Some(chunk)) = field.chunk().await {
// //                 file_chunks.push(chunk);
// //             }
// //         }
// //     }

// //     Err(Error::Other("No file uploaded".to_string()))
// // }
