use proc_macro::TokenStream;
mod enums;
mod seal;

///
/// Attribute macro for automatic conversion of enums to their string representation
///
/// This macro works only with pure enums (it does not support enums that have
/// values represented as structs)
///
/// This macro implements the following methods:
///
///
/// <div class="example-wrap"><pre class="rust rust-example-rendered"><code><span class="comment">// returns a Vec of all enum permutations
/// </span><span class="kw">fn </span>list() -&gt; `Vec&lt;MyEnum&gt;`;
/// <span class="comment">// returns the `rustdoc` description of the enum
/// </span><span class="kw">fn </span>descr(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="kw-2">&amp;</span><span class="lifetime">'static </span>str;
/// <span class="comment">// return the name of the value i.e. `Value`
/// </span><span class="kw">fn </span>as_str(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="kw-2">&amp;</span><span class="lifetime">'static </span>str;
/// <span class="comment">// return the the namespaced enum value i.e. `MyEnum::Value`
/// </span><span class="kw">fn </span>as_str_ns(<span class="kw-2">&amp;</span><span class="self">self</span>)-&gt;<span class="kw-2">&amp;</span><span class="lifetime">'static </span>str;
/// <span class="comment">// get enum value from the name i.e. `Value`
/// </span><span class="kw">fn </span>from_str(str:<span class="kw-2">&amp;</span>str)-&gt;<span class="prelude-ty">Option</span>&lt;<span class="kw">MyEnum</span>&gt;;
/// <span class="comment">// get enum value from the namespaced value name i.e. `MyEnum::Value`
/// </span><span class="kw">fn </span>from_str_ns(str:<span class="kw-2">&amp;</span>str)-&gt;<span class="prelude-ty">Option</span>&lt;#enum_name&gt;;
/// </code></pre></div>
/// 
///
// #[proc_macro_attribute]
#[proc_macro_derive(Describe, attributes(descr, describe))]
// pub fn describe_enum(attr: TokenStream, item: TokenStream) -> TokenStream {
pub fn describe_enum(item: TokenStream) -> TokenStream {
    // enums::macro_handler(attr, item)
    enums::macro_handler(item)
}

#[proc_macro]
pub fn seal(input: TokenStream) -> TokenStream {
    seal::seal(input)
}
