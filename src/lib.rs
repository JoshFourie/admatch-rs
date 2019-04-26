mod user;
mod advertiser;
mod matcher;
mod error;
#[cfg(test)] mod test;


trait UserEngine {

    type Profile;

    fn get_profile(self) -> Self::Profile;

}

trait AdvertisementEngine {

    type Profile;
    
    fn get_advertisements<U>(src: U) -> Self::Profile;

}

trait MatchEngine {

    type Output;

    fn match_advertisement<A,U>(adv: A, usr: U) -> Self::Output;

    // fn prioritise_advertisements(self) -> Self;

}