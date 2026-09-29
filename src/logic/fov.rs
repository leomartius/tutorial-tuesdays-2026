pub fn compute_fov(visible: &mut [bool], width: i32, height: i32, pov_x: i32, pov_y: i32) {
    visible.fill(false);

    for x in pov_x - 2..=pov_x + 2 {
        for y in pov_y - 2..=pov_y + 2 {
            if x >= 0 && x < width && y >= 0 && y < height {
                visible[y as usize * width as usize + x as usize] = true;
            }
        }
    }
}
