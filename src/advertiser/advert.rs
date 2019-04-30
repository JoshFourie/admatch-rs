struct DataCube<T>
{
    inner: T
}

pub struct Advertisement<T>
{
    identity: &'static str,
    interests: T,
    attributes: T,
    display: DisplayInformation
}

struct DisplayInformation { /* FIELDS */ }
