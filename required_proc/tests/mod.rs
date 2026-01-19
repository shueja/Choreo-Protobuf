use required_proc::Required;

type Requiredu32 = u32;
#[derive(Required)]
#[required(prefix = "Required")]
struct OptionalThing {
    foo: Option<u32>,
    bar: u32,
}

#[derive(Required)]
#[required(prefix = "Required")]
struct OuterThing {
    foo: Option<OptionalThing>,
    bar: u32,
}
