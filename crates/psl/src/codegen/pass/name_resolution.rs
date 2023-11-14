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

impl<T: NameResolutionPass> NameResolutionPass for Box<T> {
    fn resolve(&self, ctx: &mut NameResolutionContext) {
        T::resolve(&self, ctx)
    }
}

impl<T: NameResolutionPass> NameResolutionPass for Option<T> {
    fn resolve(&self, ctx: &mut NameResolutionContext) {
        let Some(value) = self else { return };
        T::resolve(value, ctx)
    }
}

type AstKey = (TypeId, u64);

pub struct NamesResolved {
    vec: Vec<Scope>,
    resolved: HashMap<AstKey, usize>,
}

impl NamesResolved {
    fn make_keys<K: Hash + 'static>(key: &K) -> AstKey {
        let ty = <K as Any>::type_id(key);

        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let hash = hasher.finish();

        (ty, hash)
    }

    pub fn insert(&mut self, scope: Scope) -> usize {
        self.vec.push(scope);
        self.vec.len() - 1
    }

    pub fn get<K: Hash + 'static>(&self, key: &K) -> Option<&Scope> {
        self.resolved
            .get(&Self::make_keys(key))
            .and_then(|idx| self.vec.get(*idx))
    }

    pub fn set<K: Hash + 'static>(&mut self, key: &K, idx: usize) {
        self.resolved.insert(Self::make_keys(key), idx);
    }
}

pub struct NameResolutionContext {
    // as we use single-threaded codegen, there must be no more than 1 mutator.
    root: Rc<RefCell<NamesResolved>>,
    scope: usize,
}

impl NameResolutionContext {
    pub fn new() -> Self {
        let root = Rc::new(RefCell::new(NamesResolved {
            vec: Vec::new(),
            resolved: HashMap::new(),
        }));
        let scope = root.borrow_mut().insert(Scope::new(Rc::clone(&root)));

        NameResolutionContext { root, scope }
    }

    pub fn visit<T: NameResolutionPass + Hash + 'static>(&mut self, node: &T) {
        self.root.borrow_mut().set(node, self.scope);

        node.resolve(self);
    }

    pub fn create_subscope(&mut self) -> NameResolutionContext {
        let new_scope = self
            .root
            .borrow_mut()
            .insert(Scope::with_parentt(Rc::clone(&self.root), self.scope));
        NameResolutionContext {
            root: Rc::clone(&self.root),
            scope: new_scope,
        }
    }

    pub fn scope_mut(&mut self) -> RefMut<Scope> {
        RefMut::map(self.root.borrow_mut(), |root| {
            root.vec.get_mut(self.scope).unwrap()
        })
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
