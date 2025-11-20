use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

#[proc_macro_derive(MongoRepository, attributes(entity, schema))]
pub fn mongo_repository_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    // Extract entity and schema types from attributes or struct fields
    let (entity_type, _schema_type) = match extract_types(&input) {
        Ok(types) => types,
        Err(err) => return err.to_compile_error().into(),
    };

    let expanded = quote! {
        #[async_trait::async_trait]
        impl domain::repositories::base_repository::BaseRepository<#entity_type> for #struct_name {
            async fn create(&self, entity: &#entity_type) -> shared::types::DomainResponse<#entity_type> {
                self.base.create(entity).await
            }

            async fn update(&self, id: &str, entity: &#entity_type) -> shared::types::DomainResponse<#entity_type> {
                self.base.update(id, entity).await
            }

            async fn delete(&self, id: &str) -> shared::types::DomainResponse<usize> {
                self.base.delete(id).await
            }

            async fn remove(&self, id: &str) -> shared::types::DomainResponse<#entity_type> {
                self.base.remove(id).await
            }

            async fn find(&self, id: &str) -> shared::types::DomainResponse<Option<#entity_type>> {
                self.base.find(id).await
            }

            async fn find_and_delete(&self, id: &str) -> shared::types::DomainResponse<#entity_type> {
                self.base.find_and_delete(id).await
            }

            async fn find_and_remove(&self, id: &str) -> shared::types::DomainResponse<#entity_type> {
                self.base.find_and_remove(id).await
            }

            async fn finds(&self) -> shared::types::DomainResponse<Vec<#entity_type>> {
                self.base.finds().await
            }

            async fn finds_paginated(
                &self,
                page: u32,
                page_size: u32,
            ) -> shared::types::DomainResponse<(shared::models::paginate::Paginate, Vec<#entity_type>)> {
                self.base.finds_paginated(page, page_size).await
            }
        }
    };

    TokenStream::from(expanded)
}

fn extract_types(input: &DeriveInput) -> syn::Result<(syn::Type, syn::Type)> {
    // Try to extract from struct fields
    if let Data::Struct(data_struct) = &input.data
        && let Fields::Named(fields) = &data_struct.fields
    {
        for field in &fields.named {
            if field.ident.as_ref().is_some_and(|i| i == "base") {
                // Extract generic parameters from MongoBaseRepository<E, S>
                if let syn::Type::Path(type_path) = &field.ty
                    && let Some(segment) = type_path.path.segments.last()
                    && segment.ident == "MongoBaseRepository"
                    && let syn::PathArguments::AngleBracketed(args) = &segment.arguments
                    && args.args.len() == 2
                {
                    let entity = &args.args[0];
                    let schema = &args.args[1];

                    if let (syn::GenericArgument::Type(entity_type), syn::GenericArgument::Type(schema_type)) =
                        (entity, schema)
                    {
                        return Ok((entity_type.clone(), schema_type.clone()));
                    }
                }
            }
        }
    }

    Err(syn::Error::new_spanned(input, "Could not find 'base: MongoBaseRepository<Entity, Schema>' field"))
}
