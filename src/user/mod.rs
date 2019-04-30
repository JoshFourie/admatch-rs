/*********************
 * The purpose of Raw is to build the Profile schema from a json input.
 * the only exposed api is 'get_profile' .
**********************/

mod profile;

struct User<T,U,V> {
    data: T,
    profile: U,
    schema: V,
}

impl<T,U,V> User<T,U,V> {

    fn new(data: T, profile: U, schema: V) -> Self { Self { data, profile, schema } }

}

#[cfg(test)] mod test_user_matchable {

    

}