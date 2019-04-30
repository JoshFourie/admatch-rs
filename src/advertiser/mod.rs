mod data;

#[derive(Debug)]
enum Classifier {
    Technology,
    Food,
    StartUp
}

#[derive(Debug)]
struct Candidates<I,A> { advertisements: Vec<Advertisement<I,A>> }


#[derive(Debug)]
struct Advertisement<I,A> {

    ident: String,
    target_interests: Vec<I>,
    target_attributes: Vec<A>,
    display_information: DisplayInformation,

}

#[derive(Debug)]
struct DisplayInformation {

    /* { FIELDS } */
    
}

