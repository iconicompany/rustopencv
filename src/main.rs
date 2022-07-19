use std::env;
use anyhow::anyhow;
use anyhow::Result;
use image::RgbImage;
use ndarray::{ArrayView3};
use opencv::{self as cv, prelude::*};
use tesseract::{Tesseract, TesseractError};
fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    // Read image
    let img = cv::imgcodecs::imread(filename, cv::imgcodecs::IMREAD_COLOR)?;
    // Use Orb
    let mut orb = <dyn cv::features2d::ORB>::create(
        500,
        1.2,
        8,
        31,
        0,
        2,
        cv::features2d::ORB_ScoreType::HARRIS_SCORE,
        31,
        20,
    )?;
    let mut orb_keypoints = cv::core::Vector::default();
    let mut orb_desc = cv::core::Mat::default();
    let mut dst_img = cv::core::Mat::default();
    let mask = cv::core::Mat::default();
    orb.detect_and_compute(&img, &mask, &mut orb_keypoints, &mut orb_desc, false)?;
    cv::features2d::draw_keypoints(
        &img,
        &orb_keypoints,
        &mut dst_img,
        cv::core::VecN([0., 255., 0., 255.]),
        cv::features2d::DrawMatchesFlags::DEFAULT,
    )?;
    cv::imgproc::rectangle(
        &mut dst_img,
        cv::core::Rect::from_points(cv::core::Point::new(0, 0), cv::core::Point::new(50, 50)),
        cv::core::VecN([255., 0., 0., 0.]),
        -1,
        cv::imgproc::LINE_8,
        0,
    )?;
    // Use SIFT
    let mut sift = cv::features2d::SIFT::create(0, 3, 0.04, 10., 1.6)?;
    let mut sift_keypoints = cv::core::Vector::default();
    let mut sift_desc = cv::core::Mat::default();
    sift.detect_and_compute(&img, &mask, &mut sift_keypoints, &mut sift_desc, false)?;
    cv::features2d::draw_keypoints(
        &dst_img.clone(),
        &sift_keypoints,
        &mut dst_img,
        cv::core::VecN([0., 0., 255., 255.]),
        cv::features2d::DrawMatchesFlags::DEFAULT,
    )?;
    // Write image using OpenCV
    cv::imgcodecs::imwrite("./tmp.png", &dst_img, &cv::core::Vector::default())?;
    // Convert :: cv::core::Mat -> ndarray::ArrayView3
    let a = dst_img.try_as_array()?;
    // Convert :: ndarray::ArrayView3 -> RgbImage
    // Note, this require copy as RgbImage will own the data
    let test_image = array_to_image(a);
    // Note, the colors will be swapped (BGR <-> RGB)
  	// Will need to swap the channels before
    // converting to RGBImage
    // But since this is only a demo that
    // it indeed works to convert cv::core::Mat -> ndarray::ArrayView3
    // I'll let it be
    test_image.save("out.png")?;
    let text = process_image("test/01.PNG").unwrap();
    println!("result: {}", text);
Ok(())
}
trait AsArray {
    fn try_as_array(&self) -> Result<ArrayView3<u8>>;
}
impl AsArray for cv::core::Mat {
    fn try_as_array(&self) -> Result<ArrayView3<u8>> {
        if !self.is_continuous() {
            return Err(anyhow!("Mat is not continuous"));
        }
        let bytes = self.data_bytes()?;
        let size = self.size()?;
        let a = ArrayView3::from_shape((size.height as usize, size.width as usize, 3), bytes)?;
        Ok(a)
    }
}
// From Stack Overflow: https://stackoverflow.com/questions/56762026/how-to-save-ndarray-in-rust-as-image
fn array_to_image(arr: ArrayView3<u8>) -> RgbImage {
    assert!(arr.is_standard_layout());
let (height, width, _) = arr.dim();
    let raw = arr.to_slice().expect("Failed to extract slice from array");
RgbImage::from_raw(width as u32, height as u32, raw.to_vec())
        .expect("container should have the right size for the image dimensions")
}


fn process_image(
    file_name: &str,
) -> Result<String, TesseractError> {
    let lang = "rus";

    let tesseract = Tesseract::new(None, Some(&lang))?;

    let text = tesseract
        .set_image(file_name)
        // .set_image_from_mem(&buf.as_slice())
        .unwrap()
        .recognize()
        .unwrap()
        .get_text()?;
        Ok(text)

}