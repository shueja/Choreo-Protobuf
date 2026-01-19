include!("./gen.rs");
pub mod validate {
    use tonic::{Request, Status};

    pub trait Validate {
            type Valid;
        fn validate(self) -> Result<Self::Valid, String>;
    }

    pub fn validate<R: Validate>(request: Request<R>) -> Result<<R as Validate>::Valid, Status> {
            request.into_inner().validate()
                .map_err(|e|{eprintln!("{:?}", e); Status::invalid_argument(e)})
}
}