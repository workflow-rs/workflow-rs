// use std::any::type_name;

use crate::imports::*;

// workflow_egui_macros::register_modules!(
//     register_generic_modules,
//     [
//         account_create,
//         account_manager,
//         donations,
//         export,
//         import,
//         overview,
//         private_key_create,
//         request,
//         scanner,
//         settings,
//         testing,
//         wallet_create,
//         wallet_open,
//         wallet_secret,
//         welcome,
//     ]
// );

// #[cfg(not(target_arch = "wasm32"))]
// kaspa_ng_macros::register_modules!(register_native_modules, [changelog, logs, node,]);

// #[cfg(not(feature = "lean"))]
// kaspa_ng_macros::register_modules!(register_advanced_modules, [block_dag, metrics,]);

pub enum ModuleStyle {
    Mobile,
    Default,
}

/// Capabilities of a module. Defines whether the module
/// should be available on the Desktop, Mobile, WebApp or
/// in a browser Extension.
pub enum ModuleCaps {
    Desktop,
    Mobile,
    WebApp,
    Extension,
}

pub trait ModuleT: Downcast {
    type Context;

    fn name(&self) -> Option<&'static str> {
        None
    }

    fn modal(&self) -> bool {
        false
    }

    fn secure(&self) -> bool {
        false
    }

    fn style(&self) -> ModuleStyle {
        // ModuleStyle::Large
        ModuleStyle::Default
    }

    // fn status_bar(&self, _core: &mut Core, _ui: &mut Ui) {}
    fn activate(&mut self, _core: &mut Self::Context) {}
    fn deactivate(&mut self, _core: &mut Self::Context) {}
    // fn reset(&mut self, _core: &mut Core) {}
    // fn connect(&mut self, _core: &mut Core, _network: Network) {}
    // fn disconnect(&mut self, _core: &mut Core) {}
    // fn network_change(&mut self, _core: &mut Core, _network: Network) {}
    // fn hide(&mut self, _core: &mut Self::Core) {}
    // fn show(&mut self, _core: &mut Self::Core) {}

    fn main(&mut self, _core: &mut Self::Context) {}

    fn render(
        &mut self,
        core: &mut Self::Context,
        ctx: &egui::Context,
        frame: &mut eframe::Frame,
        ui: &mut egui::Ui,
    );

    fn shutdown(&mut self) {}
}

impl_downcast!(ModuleT assoc Context);

pub struct Inner<T> {
    pub name: String,
    pub type_name: String,
    pub type_id: TypeId,
    pub module: Rc<RefCell<dyn ModuleT<Context = T>>>,
}

pub struct Module<T> {
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
    pub fn any(&self) -> Rc<RefCell<dyn ModuleT<Context = T>>> {
        Rc::clone(&self.inner.module)
    }

    pub fn main(&self, core: &mut T) {
        self.inner.module.borrow_mut().main(core)
    }

    pub fn activate(&self, core: &mut T) {
        self.inner.module.borrow_mut().activate(core)
    }

    pub fn deactivate(&self, core: &mut T) {
        self.inner.module.borrow_mut().deactivate(core)
    }

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

    pub fn name(&self) -> &str {
        self.inner
            .module
            .borrow_mut()
            .name()
            .unwrap_or_else(|| self.inner.name.as_str())
    }

    pub fn modal(&self) -> bool {
        self.inner.module.borrow_mut().modal()
    }

    pub fn secure(&self) -> bool {
        self.inner.module.borrow_mut().secure()
    }

    pub fn type_id(&self) -> TypeId {
        self.inner.type_id
    }

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

pub trait HashMapModuleExtension<T, M> {
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
