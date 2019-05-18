use openapi::v3_0::Spec;

use crate::resource::Resource;
use crate::scope::Scope;
use crate::service::{ServiceFactoryWrapper, WebServiceImpl};

pub trait GenerateOpenapi {
    fn generate_openapi(&self, spec: &mut Spec);
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

impl<T> GenerateOpenapi for Resource<T> {
    fn generate_openapi(&self, _spec: &mut Spec) {
        // TODO
    }
}

impl<T> GenerateOpenapi for Scope<T> {
    fn generate_openapi(&self, _spec: &mut Spec) {
        // TODO
    }
}

impl<T> GenerateOpenapi for WebServiceImpl<T> {
    fn generate_openapi(&self, _spec: &mut Spec) {
        // TODO
    }
}
