// Reference pointer apontam para um recurso na memória

pub fn run() {
    //Primitive array
    let arr1 = [1,2,3];
    let arr2 = arr1;

    println!("Values: {:?}", (arr1, arr2));

    //No caso de no-primitives se vc definir uma variavel a um pedaço de data, estra primeira variavel não irá segurar aquele valor
    //A partir disso vc precisa utilizar reference '&' para apontar para um recurso na memoria

    //Vectors
    let vec1 = vec![1,2,3];
    let vec2 = &vec1;

    println!("Values: {:?}", (&vec1, vec2));

}