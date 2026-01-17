// @generated
#[cfg(feature = "entity")]
// @@protoc_insertion_point(attribute:entity)
pub mod entity {
    include!("entity/entity.rs");
    // @@protoc_insertion_point(entity)
}
#[cfg(feature = "service")]
// @@protoc_insertion_point(attribute:service)
pub mod service {
    include!("service/service.rs");
    // @@protoc_insertion_point(service)
    #[cfg(feature = "service-commands")]
    // @@protoc_insertion_point(attribute:service.commands)
    pub mod commands {
        include!("service/commands/service.commands.rs");
        // @@protoc_insertion_point(service.commands)
    }
}