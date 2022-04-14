use piet_common::kurbo::{RoundedRect};
use piet_common::{
    Color, Device, FontWeight, PietText, RenderContext, Text, TextAlignment, TextAttribute,
    TextLayout, TextLayoutBuilder,
};
pub(crate) trait ImageGenerator {
    type Output;

    fn generate(text: String) -> Option<Self::Output>;
}

const WIDTH: f64 = 900.;
const MARGIN: f64 = 30.;

pub struct Generator;

impl ImageGenerator for Generator {
    type Output = ();

    fn generate(text: String) -> Option<Self::Output> {
        let mut piettext = PietText::new();
        let font = piettext.font_family("Noto Sans").expect("font not found");
        let layout = piettext
            .new_text_layout(text)
            .default_attribute(TextAttribute::Weight(FontWeight::LIGHT))
            .alignment(TextAlignment::Center)
            .font(font, 36.0)
            .max_width(WIDTH)
            .text_color(Color::WHITE)
            .build()
            .ok()?;

        let size = layout.size();
        let boundrect = dbg!(layout.image_bounds());
        let mut origin = boundrect.origin();

        let rect = RoundedRect::new(
            0.,
            0.,
            WIDTH + 2. * MARGIN,
            size.height + boundrect.y0 + 2. * MARGIN ,
            0.,
        );

        let mut device = Device::new().ok()?;
        let mut bitmap = device
            .bitmap_target(rect.width() as usize, rect.height() as usize, 1.0)
            .ok()?;

        let mut rc = bitmap.render_context();

        let brush = rc.solid_brush(Color::from_hex_str("101010").unwrap());
        rc.fill(rect, &brush);

        origin.y = MARGIN;
        origin.x = origin.x + MARGIN;

        rc.draw_text(&layout, origin);
        rc.finish().ok()?;
        std::mem::drop(rc);

        bitmap.save_to_file("tmp/render.png").ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn try_generate() {
        assert!(Generator::generate("Yea its pretty\n\nPretty fucking pathetic".into()).is_some());
    }
}
