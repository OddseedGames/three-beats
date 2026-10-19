use macroquad::prelude::*;
pub const BG: Color = Color::new(0.043, 0.067, 0.086, 1.0);
pub const PANEL: Color = Color::new(0.075, 0.106, 0.125, 1.0);
pub const INK: Color = Color::new(0.93, 0.91, 0.82, 1.0);
pub const MUTED: Color = Color::new(0.47, 0.56, 0.58, 1.0);
pub const LINE: Color = Color::new(0.17, 0.23, 0.25, 1.0);
pub const AMBER: Color = Color::new(0.94, 0.74, 0.40, 1.0);
pub const TEAL: Color = Color::new(0.43, 0.74, 0.73, 1.0);
pub const ALERT: Color = Color::new(0.94, 0.40, 0.32, 1.0);
pub fn alpha(mut c: Color, a: f32) -> Color {
    c.a *= a;
    c
}
pub fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color::from_rgba(r, g, b, 255)
}
pub fn load_font() -> Font {
    let font =
        load_ttf_font_from_bytes(include_bytes!("../../assets/IBMPlexMono-Regular.ttf")).unwrap();
    let chars: Vec<char> = (' '..='~')
        .chain('А'..='я')
        .chain("Ёё·…×—".chars())
        .collect();
    font.populate_font_cache(&chars, (64.0 * screen_dpi_scale()).ceil() as u16);
    font
}
pub fn text(font: &Font, s: &str, x: f32, y: f32, size: f32, color: Color) {
    draw_text_ex(
        s,
        x,
        y,
        TextParams {
            font: Some(font),
            font_size: 64,
            font_scale: size / 64.0,
            color,
            ..Default::default()
        },
    );
}
pub fn centered(font: &Font, s: &str, x: f32, y: f32, size: f32, color: Color) {
    let width = measure_text(s, Some(font), 64, size / 64.0).width;
    text(font, s, x - width / 2.0, y, size, color);
}
pub fn right(font: &Font, s: &str, x: f32, y: f32, size: f32, color: Color) {
    let width = measure_text(s, Some(font), 64, size / 64.0).width;
    text(font, s, x - width, y, size, color);
}
pub fn panel(r: Rect, fill: Color, border: Color) {
    draw_rectangle(r.x, r.y, r.w, r.h, fill);
    draw_rectangle_lines(r.x, r.y, r.w, r.h, 1.0, border);
}
pub fn button(font: &Font, s: &str, r: Rect, pointer: Vec2, enabled: bool) {
    let fill = if !enabled {
        PANEL
    } else if r.contains(pointer) {
        INK
    } else {
        AMBER
    };
    panel(r, fill, if enabled { AMBER } else { LINE });
    centered(
        font,
        s,
        r.x + r.w / 2.0,
        r.y + r.h / 2.0 + 6.0,
        16.0,
        if enabled { BG } else { MUTED },
    );
}
