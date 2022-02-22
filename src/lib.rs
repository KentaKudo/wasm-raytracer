mod camera;
mod hittable;
mod material;
mod ray;
mod sphere;
mod universe;
mod utils;
mod vec3;

use js_sys::Uint8ClampedArray;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use wasm_bindgen::prelude::*;

use crate::camera::Camera;
use crate::hittable::HittableList;
use crate::vec3::{Color, Point3, Vec3};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn render(width: u16, height: u16) -> Result<Uint8ClampedArray, JsValue> {
    let mut rng =
        SmallRng::from_rng(rand::thread_rng()).map_err(|e| JsValue::from(format!("{e}")))?;

    // World
    let world = HittableList::random_scene().map_err(|e| JsValue::from(format!("{e}")))?;

    let samples_per_pixel = 100;
    let max_depth = 50;

    // Camera
    let cam = Camera::new(
        Point3::new(13.0, 2.0, 3.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        width as f64 / height as f64,
        0.1,
        10.0,
    );

    let mut img = vec![];

    for j in (0..height).rev() {
        crate::utils::log!("Scanlines remaining: {j}");

        for i in 0..width {
            let color = (0..samples_per_pixel)
                .map(|_| {
                    let (u, v) = (
                        (i as f64 + rng.gen_range(0.0..1.0)) / (width as f64 - 1.0),
                        (j as f64 + rng.gen_range(0.0..1.0)) / (height as f64 - 1.0),
                    );
                    let r = cam.ray(u, v);
                    r.color(&world, max_depth)
                })
                .fold(Color::default(), |acc, c| acc + c);

            img.extend::<Vec<u8>>((color / samples_per_pixel as f64).into());
        }
    }

    crate::utils::log!("Done.");

    Ok(img[..].into())
}
