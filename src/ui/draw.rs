use comfy::*;

const SQUARE_SIZE: f32 = 80.;
const SQUARE_TRIM: f32 = 4.;
const SQUARE_CENTER: f32 = (SQUARE_SIZE - SQUARE_TRIM) / 2. - 3.; // TODO: were do these 3px come from?
const CIRCLE_SIZE: f32 = SQUARE_SIZE / 2.3;
const OFFSET: f32 = 80.;
const BOARD_SIZE: usize = 8;

#[inline]
pub fn rect(color: Color, x: u8, y: u8, w: u8, h: u8) {
    let center = screen_to_world(Vec2::new(
        (x as f32 + w as f32 / 2.) * SQUARE_SIZE + OFFSET - SQUARE_CENTER - 5.,
        (y as f32 + h as f32 / 2.) * SQUARE_SIZE + OFFSET - SQUARE_CENTER - 5.,
    ));
    let sx = screen_to_world(Vec2::new(
        screen_width() / 2.0 + SQUARE_SIZE * w as f32 - 60.,
        0.,
    ))
    .x;
    let sy = screen_to_world(Vec2::new(
        screen_width() / 2.0 + SQUARE_SIZE * h as f32 - 60.,
        0.,
    ))
    .x;
    draw_rect(center, Vec2::new(sx, sy), color, 0);
}

#[inline]
pub fn circle(color: Color, x: u8, y: u8) {
    //assert!(x < 8 && y < 8);
    let size = screen_to_world(Vec2::new(screen_width() / 2.0 + CIRCLE_SIZE, 0.)).x;
    draw_circle(
        screen_to_world(Vec2::new(
            x as f32 * SQUARE_SIZE + (SQUARE_CENTER - CIRCLE_SIZE) + OFFSET,
            y as f32 * SQUARE_SIZE + (SQUARE_CENTER - CIRCLE_SIZE) + OFFSET,
        )),
        size,
        color,
        0,
    );
}

#[inline]
pub fn triangle(color: Color, x: u8, y: u8, r: f32) {
    //assert!(x < 8 && y < 8);
    let size = screen_to_world(Vec2::new(screen_width() / 2.0 + CIRCLE_SIZE, 0.)).x;
    let center = screen_to_world(Vec2::new(
        x as f32 * SQUARE_SIZE + (SQUARE_CENTER - CIRCLE_SIZE) + OFFSET,
        y as f32 * SQUARE_SIZE + (SQUARE_CENTER - CIRCLE_SIZE) + OFFSET,
    ));
    draw_poly_z(
        center,
        3,
        size,
        -90. + r,
        color,
        0,
        TextureParams {
            blend_mode: BlendMode::Alpha,
        },
    );
}

#[inline]
pub fn board() {
    let size = screen_to_world(Vec2::new(
        screen_width() / 2.0 + SQUARE_SIZE - SQUARE_TRIM,
        screen_height() / 2.0 - SQUARE_SIZE + SQUARE_TRIM,
    ));

    let gray = Color::rgb8(0xc8, 0xc9, 0xca);
    let white = Color::rgb8(0xff, 0xff, 0xff);

    clear_background(white);
    for i in 0..BOARD_SIZE {
        for j in 0..BOARD_SIZE {
            let x = i as f32 * SQUARE_SIZE + OFFSET;
            let y = j as f32 * SQUARE_SIZE + OFFSET;

            let pos = screen_to_world(Vec2::new(x, y));
            draw_rect(pos, size, gray, 0);
        }
    }
}
