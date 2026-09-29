/// line-of-sight FOV algorithm
pub fn compute_fov(
    bounds: (i32, i32),
    is_transparent: impl Fn(i32, i32) -> bool,
    mut set_visible: impl FnMut(i32, i32),
    pov: (i32, i32),
    radius: i32,
) {
    let (width, height) = bounds;
    let (pov_x, pov_y) = pov;
    let mut points = Vec::with_capacity(radius as usize + 1);
    for dy in -radius..=radius {
        for dx in -radius..=radius {
            if dx * dx + dy * dy > radius * radius {
                continue;
            }
            let i = pov_x + dx;
            let j = pov_y + dy;
            if i < 0 || i >= width || j < 0 || j >= height {
                continue;
            }
            points.clear();
            line(pov_x, pov_y, i, j, &mut points);
            for &(x, y) in &points {
                set_visible(x, y);
                if !is_transparent(x, y) {
                    break;
                }
            }
        }
    }
}

/// Bresenham's line algorithm
fn line(x0: i32, y0: i32, x1: i32, y1: i32, points: &mut Vec<(i32, i32)>) {
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;
    let (mut x, mut y) = (x0, y0);
    loop {
        points.push((x, y));
        if x == x1 && y == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }
}
