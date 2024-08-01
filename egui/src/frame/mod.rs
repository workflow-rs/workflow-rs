use crate::imports::*;

pub mod options;
pub use options::*;
pub mod app;
pub mod core;

pub type AppCreator<T> =
    Box<dyn FnOnce(&eframe::CreationContext<'_>, Runtime) -> std::result::Result<Box<T>, DynError>>;

cfg_if! {
    if #[cfg(not(target_arch = "wasm32"))] {

        pub fn main<T,C>(options: Options<T>, events : Option<ApplicationEventsChannel>, app_creator : C)
        where T : App,
        C : FnOnce(&eframe::CreationContext<'_>, Runtime) -> std::result::Result<Box<T>, DynError> + 'static
        {

            #[cfg(feature = "console")] {
                std::env::set_var("RUST_BACKTRACE", "full");
            }

            let body = async {
                if let Err(err) = platform_main(options, events, Box::new(app_creator)).await {
                    log_error!("Error: {err}");
                }
            };

            #[allow(clippy::expect_used, clippy::diverging_sub_expression)]
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .expect("Failed building the Runtime")
                .block_on(body);

            #[cfg(feature = "console")]
            {
                println!("Press Enter to exit...");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).expect("Failed to read line");
            }


        }

    } else {

        pub fn main<T,C>(options: Options<T>, events : Option<ApplicationEventsChannel>, app_creator : C)
        where T : App,
        C : FnOnce(&eframe::CreationContext<'_>, Runtime) -> std::result::Result<Box<T>, DynError> + 'static
        {

            wasm_bindgen_futures::spawn_local(async {
                log_info!("--- starting platform main... ---");

                // todo!();
                if let Err(err) = platform_main(options, events, Box::new(app_creator)).await {
                    log_error!("Error: {err}");
                }
            });

        }
    }
}

cfg_if! {
    if #[cfg(not(target_arch = "wasm32"))] {

        async fn platform_main<T>(options : Options<T>, events : Option<ApplicationEventsChannel>, app_creator : AppCreator<T>) -> Result<()>
        where T : App
        {
            use crate::runtime::signals::Signals;

            // ------------------------------------------------------------
            // ------------------------------------------------------------
            // ------------------------------------------------------------
            // workflow_log::set_colors_enabled(true);
            // // Log to stderr (if you run with `RUST_LOG=debug`).
            // env_logger::init();
            // set_log_level(LevelFilter::Info);
            // ------------------------------------------------------------
            // ------------------------------------------------------------
            // ------------------------------------------------------------

            let runtime: Arc<Mutex<Option<Runtime>>> = Arc::new(Mutex::new(None));
            let delegate = runtime.clone();

            eframe::run_native(
                options.caption.as_str(),
                options.native_options,
                Box::new(move |cc| {
                    let runtime = Runtime::new(&cc.egui_ctx, events);
                    delegate.lock().unwrap().replace(runtime.clone());
                    Signals::bind(&runtime);
                    runtime.start();

                    Ok(Box::new(core::Core::try_new(cc, runtime, app_creator)?))
                }),
            )?;

            let runtime = runtime.lock().unwrap().take().unwrap();
            runtime.shutdown().await;


            Ok(())
        }
    } else {

        async fn platform_main<T>(options : Options<T>, events : Option<ApplicationEventsChannel>, app_creator : AppCreator<T>) -> Result<()>
        where T : App
        {
            use workflow_dom::utils::document;

            // Redirect `log` message to `console.log` and friends:
            eframe::WebLogger::init(log::LevelFilter::Debug).ok();

            if let Some(element) = document().get_element_by_id("loading") {
                element.remove();
            }

            eframe::WebRunner::new()
                .start(
                    options.canvas_id.as_str(),
                    options.web_options,
                    Box::new(move |cc| {
                        let runtime = Runtime::new(&cc.egui_ctx, events);
                        runtime.start();
                        Ok(Box::new(core::Core::try_new(cc, runtime, app_creator)?))
                    }),
                )
                .await
                .expect("failed to start eframe");
            Ok(())
        }
    }
}
