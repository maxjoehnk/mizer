use egui::{
    CentralPanel, CollapsingHeader, ColorImage, Response, ScrollArea, TextureHandle, Ui, WidgetText,
};
use egui_dock::TabViewer;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

pub type TextureMap = HashMap<u64, TextureHandle>;

pub struct DebugUiRenderHandle<'a> {
    context: &'a egui::Context,
    textures: &'a mut TextureMap,
}

impl<'a> DebugUiRenderHandle<'a> {
    pub(crate) fn new(
        context: &'a egui::Context,
        textures: &'a mut HashMap<u64, TextureHandle>,
    ) -> Self {
        Self { context, textures }
    }

    pub fn draw(&mut self, call: impl FnOnce(&mut DebugUiDrawHandle, &mut TextureMap)) {
        CentralPanel::default().show(self.context, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                let mut draw_handle = DebugUiDrawHandle::new(ui);
                call(&mut draw_handle, self.textures);
            });
        });
    }
}

pub struct DebugUiDrawHandle<'a> {
    ui: &'a mut Ui,
}

impl<'a> DebugUiDrawHandle<'a> {
    pub(crate) fn new(ui: &'a mut Ui) -> Self {
        Self { ui }
    }

    pub fn horizontal(&mut self, cb: impl FnOnce(&mut DebugUiDrawHandle)) {
        self.ui.horizontal(|ui| {
            let mut handle = DebugUiDrawHandle::new(ui);

            cb(&mut handle);
        });
    }

    pub fn button(&mut self, text: impl Into<String>) -> DebugUiResponse {
        let text = text.into();
        self.ui.button(text).into()
    }

    pub fn heading(&mut self, text: impl Into<String>) {
        let text = text.into();
        self.ui.heading(text);
    }

    pub fn label(&mut self, text: impl Into<String>) {
        let text = text.into();
        self.ui.label(text);
    }

    pub fn collapsing_header(
        &mut self,
        title: impl Into<String>,
        add_content: impl FnOnce(&mut DebugUiDrawHandle),
    ) {
        let title = title.into();
        CollapsingHeader::new(title).show(self.ui, |ui| {
            let mut handle = DebugUiDrawHandle::new(ui);

            add_content(&mut handle);
        });
    }

    pub fn columns(&mut self, count: usize, add_contents: impl FnOnce(&mut [DebugUiDrawHandle])) {
        self.ui.columns(count, |columns| {
            let mut cols = columns
                .iter_mut()
                .map(|ui| DebugUiDrawHandle::new(ui))
                .collect::<Vec<_>>();

            add_contents(&mut cols);
        });
    }

    pub fn image<I: Hash>(&mut self, image_id: I, data: &[u8], textures: &mut TextureMap) {
        let mut hasher = DefaultHasher::new();
        image_id.hash(&mut hasher);
        let hash = hasher.finish();
        if !textures.contains_key(&hash) {
            if let Ok(image) = image::load_from_memory(data) {
                let size = [image.width() as _, image.height() as _];
                let image_buffer = image.to_rgba8();
                let pixels = image_buffer.as_flat_samples();
                let image = ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
                let handle =
                    self.ui
                        .ctx()
                        .load_texture(format!("image-{hash}"), image, Default::default());

                textures.insert(hash, handle);
            }
        }
        if let Some(texture) = textures.get(&hash) {
            self.ui.image(texture, texture.size_vec2());
        }
    }
}

pub struct DebugUiResponse(Response);

impl From<Response> for DebugUiResponse {
    fn from(response: Response) -> Self {
        Self(response)
    }
}

impl DebugUiResponse {
    #[inline]
    pub fn clicked(&self) -> bool {
        self.0.clicked()
    }
}

struct DebugUiTabs {}

impl TabViewer for DebugUiTabs {
    type Tab = String;

    fn ui(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {}

    fn title(&mut self, tab: &mut Self::Tab) -> WidgetText {
        (&*tab).into()
    }
}
