use openapi::v3_0::{ObjectOrReference, Operation, PathItem, Spec};

use crate::http::Method;
use crate::scope::Scope;
use crate::service::{ServiceFactoryWrapper, WebServiceImpl};

pub trait GenerateOpenapi {
    fn generate_openapi(&self, _spec: &mut Spec) {}
    fn generate_path_item(
        &self,
        _spec: &mut Spec,
    ) -> Option<ObjectOrReference<PathItem>> {
        None
    }
    fn generate_operation(&self, _spec: &mut Spec) -> Option<(Method, Operation)> {
        None
    }
}

impl<T> GenerateOpenapi for ServiceFactoryWrapper<T>
where
    T: GenerateOpenapi,
{
    fn generate_openapi(&self, spec: &mut Spec) {
        if let Some(factory) = self.factory.as_ref() {
            factory.generate_openapi(spec);
        }
    }
}

impl<T> GenerateOpenapi for Scope<T> {}

impl<T> GenerateOpenapi for WebServiceImpl<T> {}
