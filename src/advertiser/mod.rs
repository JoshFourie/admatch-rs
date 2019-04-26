enum Classifier {
    Technology,
    Food,
    StartUp
}

struct Candidates<I,A> { advertisements: Vec<Advertisement<I,A>> }

struct Advertisement<I,A> {

    ident: String,
    target_interests: Vec<I>,
    target_attributes: Vec<A>,
    display_information: DisplayInformation,

}

struct DisplayInformation {

    /* { FIELDS } */
    
}

