use std::{
    any::{Any, TypeId},
    cell::{RefCell, RefMut},
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
    rc::Rc,
};

use crate::codegen::construct::Scope;

pub trait NameResolutionPass {
    fn resolve(&self, ctx: &mut NameResolutionContext);
}

type AstKey = (TypeId, u64);

pub struct NamesResolved {
    resolved: HashMap<AstKey, Rc<RefCell<Scope>>>,
}

impl NamesResolved {
    fn make_keys<K: Hash + 'static>(key: &K) -> AstKey {
        let ty = <K as Any>::type_id(&key);

        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let hash = hasher.finish();

        (ty, hash)
    }

    pub fn get<K: Hash + 'static>(&self, key: &K) -> Option<&Rc<RefCell<Scope>>> {
        self.resolved.get(&Self::make_keys(key))
    }

    pub fn set<K: Hash + 'static>(&mut self, key: &K, scope: Rc<RefCell<Scope>>) {
        self.resolved.insert(Self::make_keys(key), scope);
    }
}

pub struct NameResolutionContext {
    // as we use single-threaded codegen, there must be no more than 1 mutator.
    root: Rc<RefCell<NamesResolved>>,
    scope: Rc<RefCell<Scope>>,
}

impl NameResolutionContext {
    pub fn new() -> Self {
        NameResolutionContext {
            root: Rc::new(RefCell::new(NamesResolved {
                resolved: HashMap::new(),
            })),
            scope: Rc::new(RefCell::new(Scope::new())),
        }
    }

    pub fn visit<T: NameResolutionPass + Hash + 'static>(&mut self, node: &T) {
        self.root.borrow_mut().set(node, Rc::clone(&self.scope));

        node.resolve(self);
    }

    pub fn create_subscope(&mut self) -> NameResolutionContext {
        NameResolutionContext {
            root: Rc::clone(&self.root),
            scope: Rc::new(RefCell::new(Scope::create_subscope(Rc::clone(&self.scope)))),
        }
    }

    pub fn scope_mut(&mut self) -> RefMut<Scope> {
        self.scope.borrow_mut()
    }

    pub fn finish(self) -> NamesResolved {
        Rc::into_inner(self.root).unwrap().into_inner()
    }
}

impl Default for NameResolutionContext {
    fn default() -> Self {
        Self::new()
    }
}
