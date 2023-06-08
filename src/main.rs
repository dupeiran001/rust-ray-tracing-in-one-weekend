fn main() {
    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;

    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("\rScanlines remaining: {j}");
        for i in 0..IMAGE_WIDTH {
            let r = (i as f64) / (IMAGE_WIDTH as f64 - 1 as f64);
            let g = (j as f64) / (IMAGE_HEIGHT as f64 - 1 as f64);
            let b = 0.25;

            let ir: i32 = (255.999 * r) as i32;
            let ig: i32 = (255.999 * g) as i32;
            let ib: i32 = (255.999 * b) as i32;

            println!("{ir} {ig} {ib}");
        }
    }
    eprintln!("\nDone");
}
