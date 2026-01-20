// @generated
// @@protoc_insertion_point(attribute:entity)
pub mod entity {
    include!("entity/entity.rs");
    // @@protoc_insertion_point(entity)
    // @@protoc_insertion_point(attribute:entity.parameters)
    pub mod parameters {
        include!("entity/parameters/entity.parameters.rs");
        // @@protoc_insertion_point(entity.parameters)
        // @@protoc_insertion_point(attribute:entity.parameters.constraint)
        pub mod constraint {
            include!("entity/parameters/constraint/entity.parameters.constraint.rs");
            // @@protoc_insertion_point(entity.parameters.constraint)
            // @@protoc_insertion_point(attribute:entity.parameters.constraint.max_acceleration)
            pub mod max_acceleration {
                include!("entity/parameters/constraint/max_acceleration/entity.parameters.constraint.max_acceleration.rs");
                // @@protoc_insertion_point(entity.parameters.constraint.max_acceleration)
            }
            // @@protoc_insertion_point(attribute:entity.parameters.constraint.maxvelocity)
            pub mod maxvelocity {
                include!("entity/parameters/constraint/maxvelocity/entity.parameters.constraint.maxvelocity.rs");
                // @@protoc_insertion_point(entity.parameters.constraint.maxvelocity)
            }
        }
        // @@protoc_insertion_point(attribute:entity.parameters.robotconfig)
        pub mod robotconfig {
            include!("entity/parameters/robotconfig/entity.parameters.robotconfig.rs");
            // @@protoc_insertion_point(entity.parameters.robotconfig)
        }
        // @@protoc_insertion_point(attribute:entity.parameters.waypoint)
        pub mod waypoint {
            include!("entity/parameters/waypoint/entity.parameters.waypoint.rs");
            // @@protoc_insertion_point(entity.parameters.waypoint)
        }
    }
}
// @@protoc_insertion_point(attribute:service)
pub mod service {
    include!("service/service.rs");
    // @@protoc_insertion_point(service)
    // @@protoc_insertion_point(attribute:service.commands)
    pub mod commands {
        include!("service/commands/service.commands.rs");
        // @@protoc_insertion_point(service.commands)
    }
}
