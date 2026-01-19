include!("./gen.rs");
pub mod validate {
    use std::convert::Infallible;

    use tonic::{Request, Response, Status};

    pub trait Validate {
            type Valid : Required;
        fn validate(self) -> Result<Self::Valid, String>;
    }

    pub trait Required {
        type Optional : Validate;
        fn optionize(self) -> Self::Optional;
    }
    pub fn validate<R: Validate + TryInto<<R as Validate>::Valid>>(request: Request<R>) -> Result<<R as Validate>::Valid, Status> {
            request.into_inner().validate()
                .map_err(|e|{eprintln!("{:?}", e); Status::invalid_argument(e)})
}   
    /**
     * This doesn't fail but it needs the right error return signature for use in server implementations.
     */
    pub fn response<R: Required + Into<<R as Required>::Optional>>(body: R) -> Result<Response<<R as Required>::Optional>, Status> {
        Ok(Response::new(body.into()))
    }
}