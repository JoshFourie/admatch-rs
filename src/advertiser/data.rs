use std::collections::HashMap;
use std::hash::Hash;
use std::cmp::Eq;

use crate::error::{Error, ErrorKind, Repr};
use std::fmt::Debug;

trait DataCube { 

    type Output;

    type Ontology;

    /* fn drill_down(self) -> Self::Output;
    
    fn drill_up(self) -> Self::Output;

    fn slice(self) -> Self::Output;

    fn dice(self) -> Self::Output;

    fn pivot(self) -> Self::Output; 

    fn get(self) -> Self::Output;

    fn push(self) -> Self::Output; */

    fn get(self, key: Self::Ontology) -> Self::Output;

}

/*********** Placeholder Cube Structure  *****************/

// nested hashmaps are a horrible implementation but it lets us test without a relational DB.

#[cfg(test)] mod hashmap_testing_cube {

    use super::*;

    #[derive(Debug, Clone)]
    pub struct Cube<S, T> { 
        
        map: S,
        _mark: std::marker::PhantomData<T>

    }

    pub struct Ontology<T> {

        inner: Vec<T>

    }

    impl<T, U, V> Cube<HashMap<T, V>, U> {

        fn new(categories: U, candidates: V) -> Result<Self, Error> 
        where
            U: IntoIterator<Item = T>,
            V: Clone,
            T: Hash + Eq + Copy + Debug, 
        {
            let mut map: HashMap<T, V> = HashMap::new();

            for key in categories.into_iter() {
                map.insert(key, candidates.clone());
            }

            let _mark = std::marker::PhantomData;

            let cube: Self = Self { map, _mark };

            Ok(cube)
        }

    }
    
    impl<T> DataCube for Cube<HashMap<T, HashMap<Vec<T>, Vec<T>>, Vec<T>>, Vec<T>> 
    where
        T: Hash + Eq + Copy + Debug,
        Vec<T>: std::hash::BuildHasher
    {
        type Output = HashMap<Vec<T>, Vec<T>>;

        type Ontology = Ontology<T>;

        fn get(self, key: Self::Ontology) -> Self::Output {

            let len = key.inner.len();
            
            
            
        }
    }

    impl<T> Default for Cube<HashMap<T, HashMap<T, Vec<T>>>, Vec<T>>
    where
        T: Hash + Eq + Copy + Debug + From<&'static str>,
    {

        fn default() -> Self {
            let Companies: Vec<T> = vec![
                T::from("MisoSoup"),
                T::from("Banana"),
                T::from("Salmon")
            ];

            let AdvertisementsHigh: Vec<T> = vec![
                T::from("Technology"),
                T::from("Expensive")  
            ];

            let AdvertisementsLow: Vec<T> = vec![
                T::from("Laptop"),
                T::from("Desktop"),
                T::from("Mobile")
            ];

            let LowAdCube = Cube::new(AdvertisementsLow, Companies).unwrap();

            let AdCube = Cube::new(AdvertisementsHigh, LowAdCube.map).unwrap();

            AdCube
        }

    }

    #[test] fn test_cube_initialisation() {

        let AdCube = Cube::default();

        // check that the MisoSoup key is indexed.
        assert!(AdCube.map.contains_key(&"Technology"));

        // check that the Salmon company is listed in Expensive.
        assert!(AdCube.map.get(&"Expensive").unwrap().get(&"Laptop").unwrap().contains(&"Salmon"));

    }

}