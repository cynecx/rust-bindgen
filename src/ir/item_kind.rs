use super::function::Function;
use super::module::Module;
use super::ty::Type;
use super::var::Var;

/// A item we parse and translate.
#[derive(Debug)]
pub enum ItemKind {
    /// A module, created implicitly once (the root module), or via C++
    /// namespaces.
    Module(Module),

    /// A type declared in any of the multiple ways it can be declared.
    Type(Type),

    /// A function or method declaration.
    Function(Function),
    /// A variable declaration, most likely a static.
    Var(Var),
}

impl ItemKind {
    pub fn as_module(&self) -> Option<&Module> {
        match *self {
            ItemKind::Module(ref module) => Some(module),
            _ => None,
        }
    }

    pub fn is_module(&self) -> bool {
        self.as_module().is_some()
    }

    pub fn expect_module(&self) -> &Module {
        self.as_module().expect("Not a module")
    }

    pub fn as_function(&self) -> Option<&Function> {
        match *self {
            ItemKind::Function(ref func) => Some(func),
            _ => None,
        }
    }

    pub fn is_function(&self) -> bool {
        self.as_function().is_some()
    }

    pub fn expect_function(&self) -> &Function {
        self.as_function().expect("Not a function")
    }

    pub fn as_type(&self) -> Option<&Type> {
        match *self {
            ItemKind::Type(ref ty) => Some(ty),
            _ => None,
        }
    }

    pub fn as_type_mut(&mut self) -> Option<&mut Type> {
        match *self {
            ItemKind::Type(ref mut ty) => Some(ty),
            _ => None,
        }
    }

    pub fn is_type(&self) -> bool {
        self.as_type().is_some()
    }

    pub fn expect_type(&self) -> &Type {
        self.as_type().expect("Not a type")
    }

    pub fn as_var(&self) -> Option<&Var> {
        match *self {
            ItemKind::Var(ref v) => Some(v),
            _ => None,
        }
    }

    pub fn is_var(&self) -> bool {
        self.as_var().is_some()
    }

    pub fn expect_var(&self) -> &Var {
        self.as_var().expect("Not a var")
    }
}
