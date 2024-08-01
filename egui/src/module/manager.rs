use crate::imports::*;

struct Inner<T> {
    deactivation: Option<Module<T>>,
    module: Module<T>,
    modules: AHashMap<TypeId, Module<T>>,
    stack: VecDeque<Module<T>>,
}

impl<T> Inner<T>
where
    T: App,
{
    pub fn new(module: Module<T>, modules: AHashMap<TypeId, Module<T>>) -> Self {
        Self {
            deactivation: None,
            module,
            modules,
            stack: VecDeque::new(),
        }
    }
}

pub struct ModuleManager<T> {
    inner: Rc<RefCell<Inner<T>>>,
}

impl<T> Clone for ModuleManager<T> {
    fn clone(&self) -> Self {
        Self {
            inner: Rc::clone(&self.inner),
        }
    }
}

impl<T> ModuleManager<T>
where
    T: App,
{
    pub fn new(default_module: TypeId, modules: AHashMap<TypeId, Module<T>>) -> Self {
        let default_module = modules
            .get(&default_module)
            .expect("Unknown module")
            .clone();
        Self {
            inner: Rc::new(RefCell::new(Inner::new(default_module, modules))),
        }
    }

    fn inner(&self) -> Ref<Inner<T>> {
        self.inner.borrow()
    }

    fn inner_mut(&self) -> RefMut<Inner<T>> {
        self.inner.borrow_mut()
    }

    pub fn module(&self) -> Module<T> {
        self.inner().module.clone()
    }

    pub fn select<M>(&mut self, core: &mut T)
    where
        M: 'static,
    {
        self.select_with_type_id(TypeId::of::<M>(), core);
    }

    pub fn select_with_type_id(&self, type_id: TypeId, core: &mut T) {
        let (current, next) = {
            let inner = self.inner();
            (
                inner.module.clone(),
                inner.modules.get(&type_id).expect("Unknown module").clone(),
            )
        };

        if let Some(next) = (current.type_id() != next.type_id()).then(|| {
            let mut inner = self.inner_mut();
            inner.stack.push_back(current.clone());
            inner.deactivation = Some(current);
            inner.module = next.clone();
            next
        }) {
            next.activate(core);
        }
    }

    pub fn has_stack(&self) -> bool {
        !self.inner().stack.is_empty()
    }

    pub fn render(
        &self,
        app: &mut T,
        ctx: &egui::Context,
        frame: &mut eframe::Frame,
        ui: &mut egui::Ui,
    ) {
        self.module().render(app, ctx, frame, ui);
        if let Some(previous) = self.inner_mut().deactivation.take() {
            previous.deactivate(app);
        }
    }

    pub fn back(&mut self) {
        let mut inner = self.inner_mut();
        while let Some(module) = inner.stack.pop_back() {
            if !module.secure() {
                inner.module = module;
                return;
            }
        }
    }

    pub fn purge_secure_stack(&mut self) {
        self.inner_mut().stack.retain(|module| !module.secure());
    }

    pub fn get<M>(&self) -> ModuleReference<T, M>
    where
        M: ModuleT<Context = T> + 'static,
    {
        let inner = self.inner();
        let cell = inner.modules.get(&TypeId::of::<M>()).unwrap();
        ModuleReference::new(&cell.inner.module)
    }
}

pub struct ModuleReference<T, M>
where
    T: App,
{
    module: Rc<RefCell<dyn ModuleT<Context = T>>>,
    _phantom: PhantomData<M>,
}

impl<T, M> ModuleReference<T, M>
where
    T: App,
{
    fn new(module: &Rc<RefCell<dyn ModuleT<Context = T>>>) -> Self {
        Self {
            module: module.clone(),
            _phantom: PhantomData,
        }
    }

    pub fn as_ref(&self) -> Ref<'_, M>
    where
        M: ModuleT<Context = T> + 'static,
    {
        Ref::map(self.module.borrow(), |r| {
            (r).as_any()
                .downcast_ref::<M>()
                .expect("unable to downcast section")
        })
    }

    pub fn as_mut(&self) -> RefMut<'_, M>
    where
        M: ModuleT<Context = T> + 'static,
    {
        RefMut::map(self.module.borrow_mut(), |r| {
            (r).as_any_mut()
                .downcast_mut::<M>()
                .expect("unable to downcast section")
        })
    }
}
