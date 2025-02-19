#![allow(dead_code)]

use crate::container::*;
use crate::d3::{self, D3};
use crate::imports::*;
use atomic_float::AtomicF64;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use web_sys::{Element, HtmlCanvasElement};
use workflow_core::time::*;
use workflow_dom::inject::*;
use workflow_log::log_error;
use workflow_wasm::prelude::*;

static mut DOM_INIT: bool = false;

const ONE_DAY_MSEC: u64 = DAYS;
const ONE_DAY_SEC: u64 = DAYS / 1000;
const LOWREZ_CELL_SIZE: u64 = ONE_DAY_SEC / 4096;

#[derive(Clone)]
pub struct GraphDuration;

impl GraphDuration {
    pub fn parse<T: Into<String>>(value: T) -> std::result::Result<Duration, Error> {
        let value: String = value.into();
        let millis = if value.contains('s') {
            let seconds = value.replace('s', "").parse::<u64>()?;
            seconds * SECONDS
        } else if value.contains('m') {
            let minutes = value.replace('m', "").parse::<u64>()?;
            minutes * MINUTES
        } else if value.contains('h') {
            let hours = value.replace('h', "").parse::<u64>()?;
            hours * HOURS
        } else if value.contains('d') {
            let days = value.replace('d', "").parse::<u64>()?;
            days * DAYS
        } else {
            return Err(Error::Custom(format!("Invalid timeline str: {value:?}")));
        };

        Ok(Duration::from_millis(millis))
    }
}

#[derive(Clone)]
pub struct GraphThemeOptions {
    pub area_fill_color: String,
    pub area_stroke_color: String,
    pub x_axis_color: String,
    pub y_axis_color: String,
    pub title_color: String,
    pub x_axis_font: String,
    pub y_axis_font: String,
    pub title_font: String,
    pub y_caption_font: String,
    pub y_caption_color: String,
    // pub value_color: String,
    // pub value_font: String,
}

impl GraphThemeOptions {
    pub fn new(
        font_name: &str,
        title_color: &str,
        fill_color: &str,
        stroke_color: &str,
        axis_color: &str,
    ) -> GraphThemeOptions {
        GraphThemeOptions {
            title_font: format!("30px {font_name}"),
            x_axis_font: format!("20px {font_name}"),
            y_axis_font: format!("20px {font_name}"),
            area_fill_color: fill_color.into(),
            area_stroke_color: stroke_color.into(),
            x_axis_color: axis_color.into(),
            y_axis_color: axis_color.into(),
            title_color: title_color.into(),
            y_caption_color: axis_color.into(),
            y_caption_font: format!("15px {font_name}"),
        }
    }
}

#[derive(Clone)]
pub enum GraphTheme {
    Light,
    Dark,
    Custom(Box<GraphThemeOptions>),
}

impl GraphTheme {
    pub fn get_options(self) -> GraphThemeOptions {
        match self {
            Self::Light => Self::light_theme_options(),
            Self::Dark => Self::dark_theme_options(),
            Self::Custom(theme) => *theme,
        }
    }
    pub fn light_theme_options() -> GraphThemeOptions {
        let font = "'Consolas', 'Lucida Grande', 'Roboto Mono', 'Source Code Pro', 'Trebuchet'";
        GraphThemeOptions {
            // title_font: format!("bold 30px {font}"),
            title_font: format!("30px {font}"),
            x_axis_font: format!("20px {font}"),
            y_axis_font: format!("20px {font}"),
            //value_font: String::from("bold 23px sans-serif"),
            area_fill_color: String::from("rgb(220, 231, 240)"),
            area_stroke_color: String::from("rgb(17, 125, 187)"),
            x_axis_color: String::from("black"),
            y_axis_color: String::from("black"),
            title_color: String::from("black"),
            //value_color: String::from("black"),
            y_caption_color: String::from("#343434"),
            y_caption_font: String::from("15px {font}"),
        }
    }
    pub fn dark_theme_options() -> GraphThemeOptions {
        let font = "'Consolas', 'Lucida Grande', 'Roboto Mono', 'Source Code Pro', 'Trebuchet'";
        GraphThemeOptions {
            // title_font: format!("bold 30px {font}"),
            title_font: format!("30px {font}"),
            x_axis_font: format!("20px {font}"),
            y_axis_font: format!("20px {font}"),
            //value_font: String::from("bold 23px sans-serif"),
            area_fill_color: String::from("grey"),
            area_stroke_color: String::from("white"),
            x_axis_color: String::from("white"),
            y_axis_color: String::from("white"),
            title_color: String::from("white"),
            //value_color: String::from("white"),
            y_caption_color: String::from("white"),
            y_caption_font: format!("15px {font}"),
        }
    }
}

pub struct Margin {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl Margin {
    pub fn new(left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Self {
            left,
            right,
            top,
            bottom,
        }
    }
}

struct Inner {
    width: f32,
    height: f32,
    full_width: f32,
    full_height: f32,
    margin_left: f32,
    margin_right: f32,
    margin_top: f32,
    margin_bottom: f32,
    // min_date: js_sys::Date,
    value: String,
    title_box_height: f64,
    x_tick_width: f64,
    title_padding_y: f64,
    duration: Duration,
    retention: Duration,
}

#[derive(Clone)]
pub struct Graph {
    #[allow(dead_code)]
    element: Element,
    canvas: HtmlCanvasElement,
    context: web_sys::CanvasRenderingContext2d,

    inner: Arc<Mutex<Inner>>,
    x: Rc<d3::ScaleTime>,
    y: Rc<d3::ScaleLinear>,
    area: Rc<d3::Area>,
    data_hirez: Array,
    data_lowrez: Array,
    lowrez_cell: Rc<AtomicU64>,
    lowrez_cell_value: Rc<AtomicF64>,
    x_tick_size: f64,
    y_tick_size: f64,
    x_tick_count: u32,
    y_tick_count: u32,
    y_tick_padding: f64,
    title: Option<String>,
    y_caption: String,
    options: Arc<Mutex<GraphThemeOptions>>,
    time: Arc<AtomicU64>,
    redraw: Arc<AtomicBool>,
    last_draw_time: Arc<AtomicU64>,

    /// holds references to [Callback](workflow_wasm::callback::Callback)
    pub callbacks: CallbackMap,
}

unsafe impl Sync for Graph {}
unsafe impl Send for Graph {}

const DEFAULT_STYLE: &str = include_str!("graph.css");

impl Graph {
    pub async fn try_init(id: Option<&str>) -> Result<()> {
        if !unsafe { DOM_INIT } {
            inject_css(id, DEFAULT_STYLE)?;
            unsafe {
                DOM_INIT = true;
            }
        }

        Ok(())
    }

    pub async fn default_style() -> Result<String> {
        Ok(DEFAULT_STYLE.to_string())
    }

    pub async fn replace_graph_style(id: &str, css: &str) -> Result<()> {
        inject_css(Some(id), css)?;
        window().dispatch_event(&web_sys::Event::new("resize")?)?;
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn try_new<T: Into<String>>(
        window: &web_sys::Window,
        container: &Arc<Container>,
        title: Option<T>,
        y_caption: T,
        duration: Duration,
        retention: Duration,
        theme: GraphTheme,
        margin: Margin,
    ) -> Result<Graph> {
        let document = window.document().unwrap();
        let element = document.create_element("div").unwrap();
        container.element().append_child(&element).unwrap();

        element.set_class_name("graph");
        let canvas: Element = document.create_element("canvas").unwrap();
        element.append_child(&canvas).unwrap();
        let canvas = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
        let context: web_sys::CanvasRenderingContext2d = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        let options = Arc::new(Mutex::new(theme.get_options()));

        let mut graph: Graph = Graph {
            element,
            inner: Arc::new(Mutex::new(Inner {
                width: 0.0,
                height: 0.0,
                full_width: 0.0,
                full_height: 0.0,
                margin_left: margin.left,
                margin_right: margin.right,
                margin_top: margin.top,
                margin_bottom: margin.bottom,
                // min_date: js_sys::Date::new_0(),
                value: "".into(),
                title_box_height: 20.0,
                title_padding_y: 20.0,
                x_tick_width: 20.0,
                duration,
                retention,
            })),
            x: Rc::new(D3::scale_time()),
            y: Rc::new(D3::scale_linear()),
            area: Rc::new(D3::area()),
            data_hirez: Array::new(),
            data_lowrez: Array::new(),
            lowrez_cell: Rc::new(AtomicU64::new(0)),
            lowrez_cell_value: Rc::new(AtomicF64::new(0.0)),
            canvas,
            context,
            x_tick_size: 6.0,
            y_tick_size: 6.0,
            x_tick_count: 10,
            y_tick_count: 10,
            y_tick_padding: 3.0,
            title: title.map(|title| title.into()),
            y_caption: y_caption.into(),
            options,
            callbacks: CallbackMap::new(),
            time: Arc::new(AtomicU64::new(0)),
            redraw: Arc::new(AtomicBool::new(true)),
            last_draw_time: Arc::new(AtomicU64::new(0)),
        };
        graph.init().await?;
        Ok(graph)
    }

    pub fn set_title<T: Into<String>>(mut self, title: T) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn set_x_tick_size(mut self, tick_size: f64) -> Self {
        self.x_tick_size = tick_size;
        self
    }

    pub fn set_y_tick_size(mut self, tick_size: f64) -> Self {
        self.y_tick_size = tick_size;
        self
    }

    pub fn set_x_tick_count(mut self, tick_count: u32) -> Self {
        self.x_tick_count = tick_count;
        self
    }

    pub fn set_y_tick_count(mut self, tick_count: u32) -> Self {
        self.y_tick_count = tick_count;
        self
    }

    pub fn set_y_tick_padding(mut self, tick_padding: f64) -> Self {
        self.y_tick_padding = tick_padding;
        self
    }

    pub fn options(&self) -> MutexGuard<GraphThemeOptions> {
        self.options.lock().unwrap()
    }

    fn inner(&self) -> MutexGuard<Inner> {
        self.inner.lock().unwrap()
    }

    pub fn set_title_font<T: Into<String>>(&self, font: T) -> &Self {
        self.options().title_font = font.into();
        self
    }

    pub fn set_x_axis_font<T: Into<String>>(&self, font: T) -> &Self {
        self.options().x_axis_font = font.into();
        self
    }

    pub fn set_y_axis_font<T: Into<String>>(&self, font: T) -> &Self {
        self.options().y_axis_font = font.into();
        self
    }

    pub fn set_area_fill_color<T: Into<String>>(&self, color: T) -> &Self {
        self.options().area_fill_color = color.into();
        self
    }

    pub fn set_area_stroke_color<T: Into<String>>(&self, color: T) -> &Self {
        self.options().area_stroke_color = color.into();
        self
    }

    pub fn set_x_axis_color<T: Into<String>>(&self, color: T) -> &Self {
        self.options().x_axis_color = color.into();
        self
    }

    pub fn set_y_axis_color<T: Into<String>>(&self, color: T) -> &Self {
        self.options().y_axis_color = color.into();
        self
    }

    pub fn set_title_color<T: Into<String>>(&self, color: T) -> &Self {
        self.options().title_color = color.into();
        self
    }

    pub fn set_y_caption_color<T: Into<String>>(&self, color: T) -> &Self {
        self.options().y_caption_color = color.into();
        self
    }

    pub fn set_y_caption_font<T: Into<String>>(&self, font: T) -> &Self {
        self.options().y_caption_font = font.into();
        self
    }

    pub fn set_theme(&self, theme: GraphTheme) -> Result<()> {
        {
            *self.options() = theme.get_options();
        }
        self.calculate_title_box()?;
        self.draw()?;
        Ok(())
    }

    pub fn set_duration(&self, duration: Duration) -> Result<()> {
        self.inner().duration = duration;
        self.draw()?;
        Ok(())
    }

    pub fn duration(&self) -> Duration {
        self.inner().duration
    }

    // fn set_cell_value(&self, value: f64) -> Result<()> {
    //     self.lowrez_cell_value.store(value, Ordering::Relaxed);
    //     self.draw()?;
    //     Ok(())
    // }

    pub fn redraw(&self) {
        self.redraw.store(true, Ordering::Relaxed);
    }

    pub fn needs_redraw(&self) -> bool {
        let flag = self.redraw.load(Ordering::Relaxed);
        if flag {
            self.redraw.store(false, Ordering::Relaxed);
        }
        flag
    }

    pub async fn init(&mut self) -> Result<()> {
        self.calculate_title_box()?;
        self.update_size()?;
        self.update_x_domain()?;
        self.x.set_clamp(true);
        // line = d3.line()
        //     .x(function(d) { return x(d.date); })
        //     .y(function(d) { return y(d.value); })
        //     .curve(d3.curveStep)
        //     .context(context);

        let height = self.height();
        let that = self.clone();
        let x_cb = callback!(move |d: js_sys::Object| {
            that.x.call1(&JsValue::NULL, &d.get_value("date").unwrap())
        });
        let that = self.clone();
        let y_cb = callback!(move |d: js_sys::Object| {
            that.y.call1(&JsValue::NULL, &d.get_value("value").unwrap())
        });
        self.area
            .x(x_cb.get_fn())
            .y0(height)
            .y1(y_cb.get_fn())
            .context(&self.context);

        let that = self.clone();
        let on_resize = callback!(move || { that.update_size() });

        window().add_event_listener_with_callback("resize", on_resize.get_fn())?;

        self.callbacks.retain(x_cb)?;
        self.callbacks.retain(y_cb)?;
        self.callbacks.retain(on_resize)?;

        Ok(())
    }

    fn update_size(&self) -> Result<()> {
        let rect = self.canvas.get_bounding_client_rect();
        let pixel_ratio = workflow_dom::utils::window().device_pixel_ratio() as f32;
        //workflow_log::log_info!("rectrectrect: {:?}, pixel_ratio:{pixel_ratio}", rect);
        let width = (pixel_ratio * rect.right() as f32).round()
            - (pixel_ratio * rect.left() as f32).round();
        let height = (pixel_ratio * rect.bottom() as f32).round()
            - (pixel_ratio * rect.top() as f32).round();
        self.canvas.set_width(width as u32);
        self.canvas.set_height(height as u32);
        let (height, margin_left, margin_top) = {
            let mut inner = self.inner();
            inner.width = width - inner.margin_left - inner.margin_right;
            inner.height = height
                - inner.margin_top
                - inner.margin_bottom
                - inner.title_box_height as f32
                - inner.title_padding_y as f32;
            inner.full_width = width;
            inner.full_height = height;

            self.x.range([0.0, inner.width]);
            self.y.range([inner.height, 0.0]);
            (
                inner.height,
                inner.margin_left,
                inner.margin_top as f64 + inner.title_box_height + inner.title_padding_y,
            )
        };
        let context = &self.context;
        context.translate(margin_left as f64, margin_top)?;
        self.x_axis()?;
        self.y_axis()?;
        self.area.y0(height);
        self.redraw();
        Ok(())
    }

    pub fn height(&self) -> f32 {
        self.inner().height
    }
    pub fn width(&self) -> f32 {
        self.inner().width
    }
    // pub fn min_date(&self) -> js_sys::Date {
    //     self.inner().min_date.clone()
    // }

    pub fn set_value<T: Into<String>>(&self, value: T) {
        self.inner().value = value.into();
    }

    pub fn value(&self) -> String {
        self.inner().value.clone()
    }

    pub fn title_box_height(&self) -> f64 {
        self.inner().title_box_height
    }

    pub fn x_tick_width(&self) -> f64 {
        self.inner().x_tick_width
    }

    // pub fn value_color(&self) -> String {
    //     self.options().value_color.clone()
    // }

    // pub fn value_font(&self) -> String {
    //     self.options().value_font.clone()
    // }

    pub fn area_fill_color(&self) -> String {
        self.options().area_fill_color.clone()
    }
    pub fn area_stroke_color(&self) -> String {
        self.options().area_stroke_color.clone()
    }
    pub fn area_color(&self) -> (String, String) {
        let options = self.options();
        (
            options.area_fill_color.clone(),
            options.area_stroke_color.clone(),
        )
    }
    pub fn title_font(&self) -> String {
        self.options().title_font.clone()
    }
    pub fn title_color(&self) -> String {
        self.options().title_color.clone()
    }
    pub fn x_axis_font(&self) -> String {
        self.options().x_axis_font.clone()
    }
    pub fn x_axis_color(&self) -> String {
        self.options().x_axis_color.clone()
    }
    pub fn y_caption_font(&self) -> String {
        self.options().y_caption_font.clone()
    }
    pub fn y_caption_color(&self) -> String {
        self.options().y_caption_color.clone()
    }

    fn x_axis(&self) -> Result<()> {
        let width = self.width();
        let tick_count = self.x_tick_count;
        let tick_size = self.x_tick_size;
        // let tick_width = self.x_tick_width() as f32;
        // let count = (width / tick_width) as u32;
        //let ticks = self.x.ticks(count);
        let ticks = self.x.ticks(tick_count);
        // let count2 = ticks.length();
        let tick_format = self.x.tick_format();
        let context = &self.context;
        //workflow_log::log_info!("tick_format:::: {:?}", tick_format);
        let options = self.options();
        let height = self.height();

        context.begin_path();
        context.move_to(0.0, height as f64);
        context.line_to(width as f64, height as f64);
        context.set_stroke_style_str(&options.x_axis_color);
        context.stroke();

        context.begin_path();
        for tick in ticks.clone() {
            //workflow_log::log_info!("tick:::: {:?}", tick);
            let x = self
                .x
                .call1(&JsValue::NULL, &tick)
                .unwrap()
                .as_f64()
                .unwrap();
            //workflow_log::log_info!("tick::::x: {:?}", x);
            context.move_to(x, height as f64);
            context.line_to(x, height as f64 + tick_size);
        }
        context.set_stroke_style_str(&options.x_axis_color);
        context.stroke();

        // used for debugging

        context.set_text_align("center");
        context.set_text_baseline("top");
        context.set_fill_style_str(&options.x_axis_color);
        context.set_font(&options.x_axis_font);
        // context.fill_text(
        //     &format!("{tick_width}/{width}/{count}/{count2}"),
        //     150.0,
        //     40.0,
        // )?;

        let mut last_end = 0.0;
        for tick in ticks {
            let x = self
                .x
                .call1(&JsValue::NULL, &tick)
                .unwrap()
                .as_f64()
                .unwrap();
            if x < last_end {
                continue;
            }

            let text = tick_format
                .call1(&JsValue::NULL, &tick)
                .unwrap()
                .as_string()
                .unwrap();
            context.fill_text(&text, x, height as f64 + tick_size)?;
            let m = context.measure_text(&text).unwrap();
            last_end = x + m.width() + 2.0;
        }

        Ok(())
    }

    fn y_axis(&self) -> Result<()> {
        let tick_count = self.y_tick_count;
        let tick_size = self.y_tick_size;
        let tick_padding = self.y_tick_padding;
        let ticks = self.y.ticks(tick_count);
        let tick_format = self.y.tick_format();
        let context = &self.context;
        context.begin_path();
        let options = self.options();
        for tick in ticks.clone() {
            let y = self
                .y
                .call1(&JsValue::NULL, &tick)
                .unwrap()
                .as_f64()
                .unwrap();
            context.move_to(0.0, y);
            context.line_to(-tick_size, y);
        }
        context.set_stroke_style_str(&options.y_axis_color);
        context.stroke();
        let height = self.height();
        context.begin_path();
        context.move_to(-tick_size, 0.0);
        context.line_to(0.0, 0.0);
        context.line_to(0.0, height as f64);
        context.line_to(-tick_size, height as f64);
        context.set_stroke_style_str(&options.y_axis_color);
        context.stroke();

        context.set_text_align("right");
        context.set_text_baseline("middle");
        context.set_fill_style_str(&options.y_axis_color);
        context.set_font(&options.y_axis_font);
        for tick in ticks {
            let y = self
                .y
                .call1(&JsValue::NULL, &tick)
                .unwrap()
                .as_f64()
                .unwrap();
            let text = tick_format
                .call1(&JsValue::NULL, &tick)
                .unwrap()
                .as_string()
                .unwrap();
            context.fill_text(&text, -tick_size - tick_padding, y)?;
        }
        Ok(())
    }

    fn calculate_title_box(&self) -> Result<()> {
        let context = &self.context;
        let title_font = self.title_font();
        let title_color = self.title_color();
        let x_axis_font = self.x_axis_font();

        context.save();
        context.set_text_baseline("top");
        context.set_font(&title_font);
        context.set_fill_style_str(&title_color);
        let metrics = if let Some(title) = self.title.as_ref() {
            context.measure_text(&format!("{} {}", title, self.value()))?
        } else {
            context.measure_text(&self.value())?
        };

        context.set_font(&x_axis_font);
        let x_metrics = context.measure_text("_00:00PM_")?;

        {
            let mut inner = self.inner();
            inner.title_box_height = metrics.actual_bounding_box_ascent().abs()
                + metrics.actual_bounding_box_descent().abs();
            inner.x_tick_width = x_metrics.width();
        }

        context.restore();

        Ok(())
    }

    fn draw_all_captions(&self) -> Result<()> {
        self.draw_axis_captions()?;
        self.draw_title(false)?;
        Ok(())
    }

    fn draw_axis_captions(&self) -> Result<()> {
        let context = &self.context;
        let y_caption_color = self.y_caption_color();
        let y_caption_font = self.y_caption_font();
        // let value_color = self.value_color();
        // let value_font = self.value_font();
        context.save();
        context.rotate(-std::f64::consts::PI / 2.0)?;
        context.set_text_align("right");
        context.set_text_baseline("top");
        context.set_font(&y_caption_font);
        context.set_fill_style_str(&y_caption_color);
        context.fill_text(&self.y_caption, -10.0, 10.0)?;
        context.restore();

        Ok(())
    }

    fn draw_title(&self, clear: bool) -> Result<()> {
        let context = &self.context;
        let title_font = self.title_font();
        let title_color = self.title_color();

        context.save();

        context.set_text_align("left");
        context.set_text_baseline("top");
        context.set_font(&title_font);
        context.set_fill_style_str(&title_color);

        {
            let (y, height, width) = {
                let inner = self.inner();
                (
                    -(inner.margin_top as f64
                        + inner.title_box_height
                        + inner.title_padding_y / 2.0),
                    inner.title_box_height + inner.title_padding_y / 2.0,
                    inner.width as f64,
                )
            };

            if clear {
                context.clear_rect(0.0, y, width, height);
            }

            if let Some(title) = self.title.as_ref() {
                context.fill_text(&format!("{} {}", title, self.value()), 0.0, y)?;
            } else {
                context.fill_text(self.value().as_str(), 0.0, y)?;
            }
        }
        context.restore();

        Ok(())
    }

    pub fn _element(&self) -> &Element {
        &self.element
    }

    pub fn clear(&self) -> Result<()> {
        let inner = self.inner();
        let context = &self.context;
        context.clear_rect(
            -inner.margin_left as f64,
            -(inner.margin_top as f64 + inner.title_box_height + inner.title_padding_y),
            inner.full_width as f64,
            inner.full_height as f64,
        );
        Ok(())
    }

    fn update_x_domain(&self) -> Result<()> {
        let date1 = js_sys::Date::new_0();
        let time = date1.get_time();
        let date2 = js_sys::Date::new(&time.into());
        let inner = self.inner();
        date2.set_time(time - inner.duration.as_millis() as f64);
        let x_domain = js_sys::Array::new();
        x_domain.push(&date2);
        x_domain.push(&date1);

        self.x.set_domain_array(x_domain);
        Ok(())
    }

    fn update_axis_and_title(&self, data: &Array) -> Result<()> {
        self.update_x_domain()?;
        let cb = js_sys::Function::new_with_args("d", "return d.value");
        // self.y.set_domain_array(D3::extent(&self.data, cb));
        self.y.set_domain_array(D3::extent(data, cb));
        self.clear()?;
        self.x_axis()?;
        self.y_axis()?;
        self.draw_all_captions()?;

        Ok(())
    }

    fn handle_retention(&self) -> Result<()> {
        let limit = js_sys::Date::new_0();
        limit.set_time(limit.get_time() - self.inner().retention.as_millis() as f64);

        loop {
            let first_item_date = self
                .data_hirez
                .at(0)
                .dyn_into::<js_sys::Object>()?
                .get_value("date")?
                .dyn_into::<js_sys::Date>()?;
            if first_item_date.lt(&limit) {
                self.data_hirez.shift();
            } else {
                break;
            }
        }

        loop {
            let first_item_date = self
                .data_lowrez
                .at(0)
                .dyn_into::<js_sys::Object>()?
                .get_value("date")?
                .dyn_into::<js_sys::Date>()?;
            if first_item_date.lt(&limit) {
                self.data_lowrez.shift();
            } else {
                break;
            }
        }

        Ok(())
    }

    fn store(&self, time: f64, value_f64: f64) -> Result<()> {
        let value = JsValue::from(value_f64);
        // store ingested data point
        let item = js_sys::Object::new();
        let date = js_sys::Date::new(&JsValue::from(time));
        item.set("date", &date)?;
        item.set("value", &value)?;
        self.data_hirez.push(&item.into());

        let lowrez_cell = self.lowrez_cell.fetch_add(1, Ordering::SeqCst);
        if lowrez_cell % LOWREZ_CELL_SIZE == 0 {
            let lowrez_cell_value = self.lowrez_cell_value.load(Ordering::SeqCst);
            let lowrez_value = JsValue::from(lowrez_cell_value);
            let item = js_sys::Object::new();
            item.set("date", &date)?;
            item.set("value", &lowrez_value)?;
            self.data_lowrez.push(&item.into());
        } else {
            self.lowrez_cell_value
                .fetch_max(value_f64, Ordering::SeqCst);
        }

        Ok(())
    }

    pub async fn ingest(&self, time: f64, value_f64: f64, text: &str) -> Result<()> {
        // store text as value
        self.set_value(text);

        self.store(time, value_f64)?;

        // cleanup data past retention period
        self.handle_retention().unwrap_or_else(|err| {
            log_error!("Error handling retention: {err:?}");
        });

        // store current time for redraw suppression
        let time_u64 = time as u64;
        self.time.store(time_u64, Ordering::Relaxed);

        // calculate redraw suppression resolution
        let msec = self.duration().as_millis() as f32;
        let width = self.width();
        let resolution = (msec / width) as u64;
        let elapsed = time_u64 - self.last_draw_time.load(Ordering::SeqCst);
        let needs_redraw = elapsed + 1000 > resolution;

        if self.needs_redraw() || needs_redraw {
            self.draw()?;
        } else {
            self.draw_title(true)?;
        }

        Ok(())
    }

    fn draw(&self) -> Result<()> {
        let time_u64 = self.time.load(Ordering::SeqCst);
        self.last_draw_time.store(time_u64, Ordering::SeqCst);

        let secs = self.duration().as_secs() as u32;

        let data = if secs > ONE_DAY_SEC as u32 {
            let len = self.data_lowrez.length();
            let cells = secs / LOWREZ_CELL_SIZE as u32;
            if let Some(start) = len.checked_sub(cells) {
                self.data_lowrez.slice(start, len)
            } else {
                self.data_lowrez.clone()
            }
        } else {
            let len = self.data_hirez.length();
            if let Some(start) = len.checked_sub(secs) {
                self.data_hirez.slice(start, len)
            } else {
                self.data_hirez.clone()
            }
        };

        self.update_axis_and_title(&data)?;

        let (area_fill_color, area_stroke_color) = self.area_color();

        let context = &self.context;
        context.begin_path();
        self.area.call1(&JsValue::NULL, &data)?;
        context.set_fill_style_str(&area_fill_color);
        context.set_stroke_style_str(&area_stroke_color);
        context.fill();
        context.stroke();

        Ok(())
    }
}
