mod user;
mod advertiser;
mod matcher;
mod error;
#[cfg(test)] mod test;

trait Matcher<A,U> {

    type Output;

    fn get_advertisement<F>(adv: A, usr: U) -> Self::Output;

}