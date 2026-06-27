use js_sys::{Array, Function, Object};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {

    /// Handle to the global `d3` JavaScript object exposing D3's factory methods.
    #[wasm_bindgen(extends = Object, js_name = d3)]
    pub type D3;

    /// (Internal) Get the filtered command line arguments when starting the app.
    /// In NW.js, some command line arguments are used by NW.js,
    /// which should not be interested of your app. App.argv will filter out
    /// those arguments and return the ones left. You can get filtered patterns
    /// from [app::filtered_argv](self::filtered_argv) and the full arguments from [app::full_argv](self::full_argv).
    ///
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/App/#appargv)
    ///
    #[wasm_bindgen(static_method_of=D3, js_class=d3, js_name = scaleTime)]
    pub fn scale_time() -> ScaleTime;

    /// Creates a new continuous linear scale (`d3.scaleLinear`).
    #[wasm_bindgen(static_method_of=D3, js_class=d3, js_name = scaleLinear)]
    pub fn scale_linear() -> ScaleLinear;

    /// Creates a new area shape generator (`d3.area`).
    #[wasm_bindgen(static_method_of=D3, js_class=d3, js_name = area)]
    pub fn area() -> Area;

    /// Returns the `[min, max]` extent of `data` as computed by the accessor
    /// callback `cb` (`d3.extent`).
    #[wasm_bindgen(static_method_of=D3, js_class=d3, js_name = extent)]
    pub fn extent(data: &Array, cb: Function) -> Array;
}

#[wasm_bindgen]
extern "C" {
    /// A D3 time scale mapping a temporal domain onto an output range.
    #[wasm_bindgen(extends = Function)]
    pub type ScaleTime;

    /// Sets the scale's output range from a JS array; prefer [`ScaleTime::range`].
    #[wasm_bindgen(method, js_name=range)]
    pub fn range_impl(this: &ScaleTime, range: Array) -> ScaleTime;

    /// Sets the scale's input domain from a JS array of dates.
    #[wasm_bindgen(method, js_name=domain)]
    pub fn set_domain_array(this: &ScaleTime, domain: Array) -> ScaleTime;

    /// Returns approximately `count` representative tick values for the scale.
    #[wasm_bindgen(method)]
    pub fn ticks(this: &ScaleTime, count: u32) -> Array;

    /// Returns a function that formats tick values for display.
    #[wasm_bindgen(method, js_name=tickFormat)]
    pub fn tick_format(this: &ScaleTime) -> Function;

    /// Enables or disables clamping of values to the scale's range.
    #[wasm_bindgen(method, js_name=clamp)]
    pub fn set_clamp(this: &ScaleTime, clamp: bool);

    // #[wasm_bindgen(method, js_name=tickFormat)]
    // pub fn call1(this: &ScaleTime, value: JsValue) -> f64;

}

impl ScaleTime {
    /// Sets the scale's output range to the given `[start, end]` pixel bounds,
    /// returning `self` for chaining.
    pub fn range(&self, range: [f32; 2]) -> &Self {
        let range_value = Array::new();
        range_value.push(&range[0].into());
        range_value.push(&range[1].into());
        self.range_impl(range_value);
        self
    }
}

#[wasm_bindgen]
extern "C" {
    /// A D3 continuous linear scale mapping a numeric domain onto an output range.
    #[wasm_bindgen(extends = Function)]
    pub type ScaleLinear;

    /// Sets the scale's output range from a JS array; prefer [`ScaleLinear::range`].
    #[wasm_bindgen(method, js_name=range)]
    pub fn range_impl(this: &ScaleLinear, range: Array) -> ScaleLinear;

    /// Sets the scale's input domain from a JS array; prefer [`ScaleLinear::set_domain`].
    #[wasm_bindgen(method, js_name=domain)]
    pub fn set_domain_array(this: &ScaleLinear, domain: Array) -> ScaleLinear;

    /// Returns approximately `count` representative tick values for the scale.
    #[wasm_bindgen(method)]
    pub fn ticks(this: &ScaleLinear, count: u32) -> Array;

    /// Returns a function that formats tick values for display.
    #[wasm_bindgen(method, js_name=tickFormat)]
    pub fn tick_format(this: &ScaleLinear) -> Function;
}

impl ScaleLinear {
    /// Sets the scale's output range to the given `[start, end]` pixel bounds,
    /// returning `self` for chaining.
    pub fn range(&self, range: [f32; 2]) -> &Self {
        let range_value = Array::new();
        range_value.push(&range[0].into());
        range_value.push(&range[1].into());
        self.range_impl(range_value);
        self
    }

    /// Sets the scale's input domain to the `[min, max]` range, returning
    /// `self` for chaining.
    pub fn set_domain(&self, min: u32, max: u32) -> &Self {
        let domain = Array::new();
        domain.push(&min.into());
        domain.push(&max.into());
        self.set_domain_array(domain);
        self
    }
}

#[wasm_bindgen]
extern "C" {
    /// A D3 area shape generator that produces filled regions between two
    /// y-values across an x-domain.
    #[wasm_bindgen(extends = Function)]
    pub type Area;

    /// Sets the accessor callback used to compute the x-coordinate of each point.
    #[wasm_bindgen(method)]
    pub fn x(this: &Area, cb: &Function) -> Area;

    /// Sets the constant baseline (lower) y-coordinate of the area.
    #[wasm_bindgen(method)]
    pub fn y0(this: &Area, value: f32) -> Area;

    /// Sets the accessor callback used to compute the topline (upper) y-coordinate.
    #[wasm_bindgen(method)]
    pub fn y1(this: &Area, cb: &Function) -> Area;

    /// Sets the canvas 2D rendering context that the generator draws to.
    #[wasm_bindgen(method)]
    pub fn context(this: &Area, ctx: &web_sys::CanvasRenderingContext2d) -> Area;
}
