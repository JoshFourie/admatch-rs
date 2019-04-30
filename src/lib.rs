mod user;
mod error;
mod advertiser;
#[cfg(test)] mod test;

trait Profile 
{
    fn interests<T>(self) -> Box<dyn Iterator<Item=T>>;

    fn attributes<T>(self) -> Box<dyn Iterator<Item=T>>;
}

trait DataCube<T, U>: Sized + From<Vec<Self>>
where
    for <'a> &'a U: Profile,

{
    type Output: From<Self>;

    fn get_advertisements(self, profile: &U) -> Result<Self::Output, Box<dyn std::error::Error+Send+Sync>>
    {
        let interests_slice: Self = profile.interests::<T>()
            .into_iter()
            .map(|interest| self.slice(interest))
            .collect::<Vec<Self>>()
            .into();
        
        let attributes_slice: Self = profile.attributes::<T>()
            .into_iter()
            .map(|attribute| self.slice(attribute))
            .collect::<Vec<Self>>()
            .into();
        
        let list: Self::Output = interests_slice.find_overlap(attributes_slice)
            .expand_ontology(profile)
            .prioritise(profile)
            .into();        

        Ok(list)
    }

    fn slice<V>(&self, mapping: V) -> Self;   

    fn find_overlap(self, rhs: Self) -> Self;

    fn prioritise(self, profile: &U) -> Self;

    fn expand_ontology(self, profile: &U) -> Self;

}