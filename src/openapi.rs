use std::collections::BTreeMap;

use openapi::v3_0::{
    MediaType, ObjectOrReference, Operation, PathItem, Response, Schema, Spec,
};
use openapi_schema::OpenapiSchema;

use crate::http::{Method, StatusCode};
use crate::scope::Scope;
use crate::service::{ServiceFactoryWrapper, WebServiceImpl};
use crate::types::json::Json;

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

pub trait GenerateOpenapiStatic {
    fn generate_response(_spec: &mut Spec) -> Option<(StatusCode, Response)> {
        None
    }
}

impl<T> GenerateOpenapiStatic for Json<T> {
    default fn generate_response(spec: &mut Spec) -> Option<(StatusCode, Response)> {
        let components = spec.components.get_or_insert_with(Default::default);
        let schemas = components.schemas.get_or_insert_with(Default::default);
        schemas.entry("Object".to_string()).or_insert_with(|| {
            // schema for an object with any properties
            let schema = Schema {
                schema_type: Some("object".to_string()),
                additional_properties: Some(ObjectOrReference::Object(Box::new(
                    Schema {
                        schema_type: Some("string".to_string()),
                        ..Default::default()
                    },
                ))),
                ..Default::default()
            };
            ObjectOrReference::Object(schema)
        });

        let media_type = MediaType {
            schema: Some(ObjectOrReference::Ref {
                ref_path: "#/components/schemas/Object".to_string(),
            }),
            ..Default::default()
        };

        let mut content = BTreeMap::new();
        content.insert("application/json".to_string(), media_type);

        let response = Response {
            description: Some("object".to_string()),
            content: Some(content),
            ..Default::default()
        };
        Some((StatusCode::OK, response))
    }
}

impl<T: OpenapiSchema> GenerateOpenapiStatic for Json<T> {
    fn generate_response(spec: &mut Spec) -> Option<(StatusCode, Response)> {
        let schema = T::generate_schema(spec);

        let mut content = BTreeMap::new();
        content.insert(
            "application/json".into(),
            MediaType {
                schema: Some(schema),
                ..Default::default()
            },
        );

        let response = Response {
            content: Some(content),
            ..Default::default()
        };
        Some((StatusCode::OK, response))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_json_response() {
        #[allow(dead_code)]
        struct WithoutOpenapiSchema {
            field: String,
        };

        let mut spec = Spec::default();
        if let Some((status_code, response)) =
            Json::<WithoutOpenapiSchema>::generate_response(&mut spec)
        {
            assert_eq!(status_code, StatusCode::OK);
            let content = response.content.expect("missing content");
            let content = content
                .get("application/json")
                .expect("missing application/json");
            match &content.schema {
                Some(ObjectOrReference::Ref { ref_path }) => {
                    assert!(ref_path.ends_with("Object"))
                }
                _ => panic!("expect some object reference"),
            }
        } else {
            panic!("unexpected None");
        }
    }

    #[test]
    fn test_json_with_schema_response() {
        #[allow(dead_code)]
        #[derive(OpenapiSchema)]
        struct WithOpenapiSchema {
            field: String,
        };

        let mut spec = Spec::default();
        if let Some((status_code, response)) =
            Json::<WithOpenapiSchema>::generate_response(&mut spec)
        {
            assert_eq!(status_code, StatusCode::OK);
            let content = response.content.expect("missing content");
            let content = content
                .get("application/json")
                .expect("missing application/json");
            match &content.schema {
                Some(ObjectOrReference::Ref { ref_path }) => {
                    assert!(ref_path.ends_with("WithOpenapiSchema"))
                }
                _ => panic!("expect some object reference"),
            }
        } else {
            panic!("unexpected None");
        }

        let components = spec.components.expect("missing components");
        let schemas = components.schemas.expect("missing schemas");
        schemas.contains_key("WithOpenapiSchema");
    }
}
