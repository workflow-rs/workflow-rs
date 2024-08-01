use crate::frame::app::App;
use crate::imports::*;

pub struct Core<T>
where
    T: App,
{
    app: Box<T>,

    is_shutdown_pending: bool,
    _settings_storage_requested: bool,
    _last_settings_storage_request: Instant,

    #[allow(dead_code)]
    runtime: Runtime,
    events: ApplicationEventsChannel,
}

impl<T> Core<T>
where
    T: App,
{
    /// Core initialization
    pub fn try_new(
        cc: &eframe::CreationContext<'_>,
        runtime: Runtime,
        app_creator: crate::frame::AppCreator<T>,
    ) -> Result<Self> {
        let mut app = app_creator(cc, runtime.clone())?;
        app.init(&runtime, cc);

        let events = runtime.events().clone();

        Ok(Self {
            runtime,
            app,
            is_shutdown_pending: false,
            _settings_storage_requested: false,
            _last_settings_storage_request: Instant::now(),
            events,
        })
    }
}

impl<T> eframe::App for Core<T>
where
    T: App,
{
    #[cfg(not(target_arch = "wasm32"))]
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.is_shutdown_pending = true;
        Runtime::halt();
        println!("bye!");
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        log_info!("--- update ---");

        for event in self.events.iter() {
            if let Err(err) = self.handle_events(event.clone(), ctx, frame) {
                log_error!("error processing wallet runtime event: {}", err);
            }
        }

        if self.is_shutdown_pending {
            return;
        }

        ctx.input(|input| {
            input.events.iter().for_each(|event| {
                if let egui::Event::Key {
                    key,
                    pressed,
                    modifiers,
                    repeat,
                    // TODO - propagate
                    physical_key: _,
                    // ..
                } = event
                {
                    self.handle_keyboard_events(*key, *pressed, modifiers, *repeat);
                }
            });
        });

        if let Some(device) = self.app.device() {
            device.set_screen_size(&ctx.screen_rect())
        }

        self.render(ctx, frame);

        // #[cfg(not(target_arch = "wasm32"))]
        // if let Some(screenshot) = self.screenshot.clone() {
        //     self.handle_screenshot(ctx, screenshot);
        // }
    }
}

impl<T> Core<T>
where
    T: App,
{
    fn render(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.app.render(ctx, frame);
    }

    // #[cfg(not(target_arch = "wasm32"))]
    // fn handle_screenshot(&mut self, ctx: &Context, screenshot: Arc<ColorImage>) {
    //     match rfd::FileDialog::new().save_file() {
    //         Some(mut path) => {
    //             path.set_extension("png");
    //             let screen_rect = ctx.screen_rect();
    //             let pixels_per_point = ctx.pixels_per_point();
    //             let screenshot = screenshot.clone();
    //             let sender = self.sender();
    //             std::thread::Builder::new()
    //                 .name("screenshot".to_string())
    //                 .spawn(move || {
    //                     let image = screenshot.region(&screen_rect, Some(pixels_per_point));
    //                     image::save_buffer(
    //                         &path,
    //                         image.as_raw(),
    //                         image.width() as u32,
    //                         image.height() as u32,
    //                         image::ColorType::Rgba8,
    //                     )
    //                     .unwrap();

    //                     sender
    //                         .try_send(Events::Notify {
    //                             user_notification: UserNotification::success(format!(
    //                                 "Capture saved to\n{}",
    //                                 path.to_string_lossy()
    //                             ))
    //                             .as_toast(),
    //                         })
    //                         .unwrap()
    //                 })
    //                 .expect("Unable to spawn screenshot thread");
    //             self.screenshot.take();
    //         }
    //         None => {
    //             self.screenshot.take();
    //         }
    //     }
    // }

    pub fn handle_events(
        &mut self,
        event: RuntimeEvent,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
    ) -> Result<()> {
        // log_info!("--- event: {:?}", event);
        if matches!(event, RuntimeEvent::Exit) {
            self.is_shutdown_pending = true;
        }

        self.app.handle_event(ctx, event);

        Ok(())
    }

    fn handle_keyboard_events(
        &mut self,
        key: egui::Key,
        pressed: bool,
        modifiers: &egui::Modifiers,
        _repeat: bool,
    ) {
        self.app
            .handle_keyboard_events(key, pressed, modifiers, false);
    }

    // pub fn apply_mobile_style(&self, ui: &mut Ui) {
    //     ui.style_mut().text_styles = self.mobile_style.text_styles.clone();
    // }

    // pub fn apply_default_style(&self, ui: &mut Ui) {
    //     ui.style_mut().text_styles = self.default_style.text_styles.clone();
    // }
}
