use egui::Ui;
use whiskers::prelude::*;

/// Very custom data structure that is not supported by default
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
struct GrayRed {
    gray: f64,
    red: f64,
}

impl From<GrayRed> for vsvg::Color {
    fn from(value: GrayRed) -> Self {
        let red = ((value.red) * 255.0) as u8;
        let gray = (value.gray * 255.0) as u8;
        vsvg::Color::new(red, gray, gray, 255)
    }
}

/// Custom UI widget for [`GreyRed`]. It must implement the [`whiskers::widgets::Widget<GrayRed>`]
/// trait.
#[derive(Default)]
struct GrayRedWidget {
    label_color: egui::Color32,
    underline: bool,
}

/// We want the ability to customise the look of our widget!
impl GrayRedWidget {
    pub fn label_color(mut self, color: egui::Color32) -> Self {
        self.label_color = color;
        self
    }

    pub fn underline(mut self, underline: bool) -> Self {
        self.underline = underline;
        self
    }
}

/// This is where the custom UI code happens.
impl whiskers_widgets::Widget<GrayRed> for GrayRedWidget {
    fn ui(&self, ui: &mut Ui, label: &str, value: &mut GrayRed) -> bool {
        let mut label = egui::RichText::new(label).color(self.label_color);
        if self.underline {
            label = label.underline();
        }
        ui.add(egui::Label::new(label));

        // The UI from this function is integrated in a two column layout, for a nice alignment of
        // the labels. It is thus important that we render only *two* top-level `ui` calls. Here, we
        // have the label and the `ui.vertical()` call, so we're good.

        // Note: for more complex UI, it's possible to override `use_grid()` and return `false`.
        // In this case, the two-column grid will not be used for this widget.

        ui.vertical(|ui| {
            let mut changed = false;
            changed = ui
                .horizontal(|ui| {
                    ui.label("gr:");
                    ui.add(egui::Slider::new(&mut value.gray, 0.0..=1.0))
                })
                .inner
                .changed()
                || changed;

            changed = ui
                .horizontal(|ui| {
                    ui.label("rd:");
                    ui.add(egui::Slider::new(&mut value.red, 0.0..=1.0))
                })
                .inner
                .changed()
                || changed;

            changed
        })
        .inner
    }
}

// Let the [`Sketch`] derive macro know that [`GrayRedWidget`] is the UI widget for [`GrayRed`].
whiskers_widgets::register_widget_ui!(GrayRed, GrayRedWidget);

// =================================================================================
// from here on, we're back to super standard  sketch code...

#[sketch_app]
struct CustomUISketch {
    // these param key/value will call into the [`GrayRedWidget`]'s builder methods.
    #[param(underline, label_color = egui::Color32::BLUE)]
    color: GrayRed,
}

impl Default for CustomUISketch {
    fn default() -> Self {
        CustomUISketch {
            color: GrayRed {
                red: 0.5,
                gray: 0.5,
            },
        }
    }
}

impl App for CustomUISketch {
    fn update(&mut self, sketch: &mut Sketch, _ctx: &mut Context) -> anyhow::Result<()> {
        sketch.color(self.color);
        for i in 0..5 {
            sketch.circle(100.0, 100.0, 30.0 + 40.0 + i as f64 * 3.0);
        }

        Ok(())
    }
}

fn main() -> Result {
    CustomUISketch::runner()
        .with_page_size_options(PageSize::new(200.0, 200.0))
        .run()
}
