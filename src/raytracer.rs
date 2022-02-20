pub fn render() -> Vec<u8> {
    let (width, height) = (256, 256);
    let mut img = vec![];

    for j in (0..height).rev() {
        for i in 0..width {
            let (r, g, b) = (
                i as f64 / (width as f64 - 1.0),
                j as f64 / (height as f64 - 1.0),
                0.25,
            );

            let (ir, ig, ib) = (
                (255.999 * r) as u8,
                (255.999 * g) as u8,
                (255.999 * b) as u8,
            );

            img.extend([ir, ig, ib]);
        }
    }

    img
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample_image() {
        let got = super::render();
        let want = 256 * 256 * 3;
        assert_eq!(got.len(), want)
    }
}
