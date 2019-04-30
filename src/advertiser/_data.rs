
trait DataCube 
{ 
    type Output;

    type Ontology;

    fn drill_down(self) -> Self::Output;
    
    fn drill_up(self) -> Self::Output;

    fn slice(self) -> Self::Output;

    fn dice(self) -> Self::Output;

    fn pivot(self) -> Self::Output; 

    fn push(self) -> Self::Output; 

    fn get(self, key: Self::Ontology) -> Self::Output;
}

/*********** Placeholder Cube Structure  *****************/

// nested hashmaps are a horrible implementation but it lets us test without a relational DB.

#[cfg(test)] mod hashmap_testing_cube {
    
    use std::collections::BTreeMap;
    use super::*;

    #[derive(Debug)]
    struct DataTree<K,V> {
        inner: BTreeMap<K,V>
    }

    impl Default for DataTree<&'static str,Vec<&'static str>> 
    {
        fn default() -> Self 
        {
            let mut inner: BTreeMap<_,_> = BTreeMap::new();

            inner.insert("MisoSoup", vec!["Food", "Taste", "Cheap"]);
            inner.insert("MicroChip", vec!["Technology", "Laptop", "Expensive"]);
            inner.insert("MiddleMan", vec!["Finance"]);

            Self { inner }
        }
    }

    impl DataTree<&'static str,Vec<&'static str>>
    {
        fn get(self, key: &'static str) -> Vec<&'static str> 
        { 
            self.inner[key].clone() 
        }
    }

    #[test] fn test_default_tree() 
    {
        let tree: DataTree<_,_> = Default::default();
        assert!(tree.inner["MicroChip"].contains(&"Technology"));
    }

    #[test] fn test_tree_get() 
    {
        let tree: DataTree<_,_> = Default::default();
        let test = tree.get("MicroChip");
        let exp = vec!["Technology", "Laptop", "Expensive"];
        assert_eq!(test,exp);
    }    
}