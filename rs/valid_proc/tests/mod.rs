use valid_proc::Valid;

type Validu32 = u32;
#[derive(Valid)]
#[valid(prefix = "Valid")]
struct OptionalThing {
    foo: Option<u32>,
    bar: u32,
}

#[derive(Valid)]
#[valid(prefix = "Valid")]
struct OuterThing {
    foo: Option<OptionalThing>,
    bar: u32,
}
