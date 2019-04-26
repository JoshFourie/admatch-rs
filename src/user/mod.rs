/*********************
 * The purpose of Raw is to build the Profile schema from a json input.
 * the only exposed api is 'get_profile' .
**********************/

use serde_derive::{Serialize, Deserialize};
use serde_json::Error as SerdeError;
use super::{UserEngine, error::Error};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Raw<T> { inner: T }

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Profile {

    interests: Vec<Interests>,
    attributes: Vec<Attributes>

}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
enum Interests {

    Technology,
    Startups,
    Food

}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
enum Attributes {

    Name(String),
    Location(Location),
    Expenditure(usize)    

}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Location {

    long: (f32, String),
    lat: (f32, String)

}

impl<T> From<Raw<T>> for Result<Profile,SerdeError> 
where
    String: From<T>
{
    fn from(raw: Raw<T>) -> Self {
        serde_json::from_str(&String::from(raw.inner))
    }
}
 
impl<T> UserEngine for Raw<T> 
where
    String: From<T>
{
    type Profile = Result<Profile, Error>;

    fn get_profile(self) -> Self::Profile {
        let _profile: Result<Profile, SerdeError> = serde_json::from_str(&String::from(self.inner));
        match _profile {
            Ok(profile) => Ok(profile),
            Err(e) => Err(Error::from(e))
        }
    }

}

#[cfg(test)] mod user_tests {

    use super::*;

    fn make_dummy() -> Profile {
        let interests: Vec<Interests> = vec![
            Interests::Food, Interests::Startups, Interests::Technology
        ];
        let location: Location = Location {
            lat: (35.2777, "S".to_string()), 
            long : (149.1185, "E".to_string())
        };
        let attributes: Vec<Attributes> = vec![
            Attributes::Name("dummy".to_string()),
            Attributes::Expenditure(92000),
            Attributes::Location(location)
        ];
        Profile {
            interests,
            attributes,
        }
    }

    #[test] fn test_deserialize_for_dummy() {
        let ser_dum: String = serde_json::to_string(&make_dummy()).unwrap();
        let de_dum: Profile = serde_json::from_str(&ser_dum).unwrap();
        let dummy: Profile = make_dummy();
        assert_eq!(de_dum, dummy);
    }
}