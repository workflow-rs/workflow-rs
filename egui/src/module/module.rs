use crate::imports::*;

/// Layout style with which a module's UI should be rendered, selecting the set
/// of text styles applied before rendering.
pub enum ModuleStyle {
    /// Render using the application's mobile text styles.
    Mobile,
    /// Render using the application's default (desktop) text styles.
    Default,
}

/// Capabilities of a module. Defines whether the module
/// should be available on the Desktop, Mobile, WebApp or
/// in a browser Extension.
pub enum ModuleCaps {
    /// The module is available on the desktop application.
    Desktop,
    /// The module is available on the mobile application.
    Mobile,
    /// The module is available in the web application.
    WebApp,
    /// The module is available in the browser extension.
    Extension,
}

/// Trait implemented by a UI module: a self-contained, downcastable unit with
/// lifecycle hooks (activate/deactivate/main/render/shutdown) operating over an
/// associated application context.
pub trait ModuleT: Downcast {
    /// The application context type passed to the module's lifecycle hooks.
    type Context;

    /// Returns an explicit display name for the module, or `None` to use the
    /// name derived from its type.
    fn name(&self) -> Option<&'static str> {
        None
    }

    /// Returns whether the module should be presented modally. Defaults to
    /// `false`.
    fn modal(&self) -> bool {
        false
    }

    /// Returns whether the module handles sensitive data and should be treated
    /// as secure. Defaults to `false`.
    fn secure(&self) -> bool {
        false
    }

    /// Returns the layout style the module should be rendered with. Defaults to
    /// [`ModuleStyle::Default`].
    fn style(&self) -> ModuleStyle {
        ModuleStyle::Default
    }

    /// Called when the module becomes the active/focused module.
    fn activate(&mut self, _core: &mut Self::Context) {}

    /// Called when the module loses focus or is switched away from.
    fn deactivate(&mut self, _core: &mut Self::Context) {}

    /// Runs the module's main update step, called each frame while active.
    fn main(&mut self, _core: &mut Self::Context) {}

    /// Renders the module's UI for the current frame.
    fn render(
        &mut self,
        core: &mut Self::Context,
        ctx: &egui::Context,
        frame: &mut eframe::Frame,
        ui: &mut egui::Ui,
    );

    /// Called when the module is being torn down, allowing it to release
    /// resources.
    fn shutdown(&mut self) {}
}

impl_downcast!(ModuleT assoc Context);

/// Shared inner state of a [`Module`], holding the module instance alongside
/// its identifying metadata.
pub struct Inner<T> {
    /// The module's display name (derived from its type name).
    pub name: String,
    /// The fully-qualified type name of the concrete module.
    pub type_name: String,
    /// The [`TypeId`] of the concrete module type, used as a registry key.
    pub type_id: TypeId,
    /// The interior-mutable, type-erased module instance.
    pub module: Rc<RefCell<dyn ModuleT<Context = T>>>,
}

/// A cheaply-clonable, reference-counted handle to a registered module.
pub struct Module<T> {
    /// Shared inner state holding the module and its metadata.
    pub inner: Rc<Inner<T>>,
}

impl<T> Clone for Module<T> {
    fn clone(&self) -> Self {
        Self {
            inner: Rc::clone(&self.inner),
        }
    }
}

impl<T> Module<T>
where
    T: App + 'static,
{
    /// Returns a cloned reference-counted handle to the inner type-erased
    /// module.
    pub fn any(&self) -> Rc<RefCell<dyn ModuleT<Context = T>>> {
        Rc::clone(&self.inner.module)
    }

    /// Runs the module's main update step, invoking its [`ModuleT::main`] hook.
    pub fn main(&self, core: &mut T) {
        self.inner.module.borrow_mut().main(core)
    }

    /// Activates the module, invoking its [`ModuleT::activate`] hook.
    pub fn activate(&self, core: &mut T) {
        self.inner.module.borrow_mut().activate(core)
    }

    /// Deactivates the module, invoking its [`ModuleT::deactivate`] hook.
    pub fn deactivate(&self, core: &mut T) {
        self.inner.module.borrow_mut().deactivate(core)
    }

    /// Applies the module's style (mobile or default text styles) and then
    /// renders the module's UI.
    pub fn render(
        &self,
        core: &mut T,
        ctx: &egui::Context,
        frame: &mut eframe::Frame,
        ui: &mut egui::Ui,
    ) {
        let mut module = self.inner.module.borrow_mut();

        match module.style() {
            ModuleStyle::Mobile => {
                if let Some(text_styles) = core.mobile_text_styles() {
                    ui.style_mut().text_styles = text_styles;
                }
            }
            ModuleStyle::Default => {
                if let Some(text_styles) = core.default_text_styles() {
                    ui.style_mut().text_styles = text_styles;
                }
            }
        }

        module.render(core, ctx, frame, ui)
    }

    /// Returns the module's display name, falling back to the derived type name
    /// when the module does not provide its own.
    pub fn name(&self) -> &str {
        self.inner
            .module
            .borrow_mut()
            .name()
            .unwrap_or_else(|| self.inner.name.as_str())
    }

    /// Returns whether the module should be presented modally.
    pub fn modal(&self) -> bool {
        self.inner.module.borrow_mut().modal()
    }

    /// Returns whether the module is marked as secure.
    pub fn secure(&self) -> bool {
        self.inner.module.borrow_mut().secure()
    }

    /// Returns the [`TypeId`] of the concrete module type.
    pub fn type_id(&self) -> TypeId {
        self.inner.type_id
    }

    /// Borrows the inner module downcast to the concrete type `M`, panicking if
    /// the stored module is not of that type.
    pub fn as_ref<M>(&self) -> Ref<'_, M>
    where
        M: ModuleT + 'static,
    {
        Ref::map(self.inner.module.borrow(), |r| {
            (r).as_any()
                .downcast_ref::<M>()
                .expect("unable to downcast section")
        })
    }

    /// Mutably borrows the inner module downcast to the concrete type `M`,
    /// panicking if the stored module is not of that type.
    pub fn as_mut<M>(&mut self) -> RefMut<'_, M>
    where
        M: ModuleT + 'static,
    {
        RefMut::map(self.inner.module.borrow_mut(), |r| {
            (r).as_any_mut()
                .downcast_mut::<M>()
                .expect("unable to downcast_mut module")
        })
    }
}

impl<T> std::fmt::Debug for Module<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner.name)
    }
}

impl<T> Eq for Module<T> {}

impl<T> PartialEq for Module<T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner.type_id == other.inner.type_id
    }
}

impl<T, M> From<M> for Module<T>
where
    M: ModuleT<Context = T> + 'static,
    T: App,
{
    fn from(section: M) -> Self {
        let type_name = type_name::<M>().to_string();
        let name = type_name.split("::").last().unwrap().to_string();
        let type_id = TypeId::of::<M>();
        Self {
            inner: Rc::new(Inner {
                name,
                type_name,
                type_id,
                module: Rc::new(RefCell::new(section)),
            }),
        }
    }
}

/// Extension trait for inserting a module into a map keyed by its [`TypeId`].
pub trait HashMapModuleExtension<T, M> {
    /// Inserts the module into the map, using the module type's [`TypeId`] as
    /// the key.
    fn insert_typeid(&mut self, value: M)
    where
        M: ModuleT<Context = T> + 'static,
        T: App;
}

impl<T, M> HashMapModuleExtension<T, M> for AHashMap<TypeId, Module<T>>
where
    M: ModuleT<Context = T> + 'static,
    T: App,
{
    fn insert_typeid(&mut self, section: M) {
        self.insert(TypeId::of::<M>(), Module::<T>::from(section));
    }
}
