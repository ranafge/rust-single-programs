/*
    compiller is capable of providing basic implementations of some trait vai the #[derive] attribute.
    These traits can still be manually implemented if a more complex behaviour is required.

*/

// Compreson trait Eq, PartialEq, Ord,PartialOrd
/*
    Clone, to create T from &T via a copy.
    Copy ,  copy semantics instead of move semantics
    Hash, to compute a hash form &T
    Default,  to create empty instace of a data type
    Debug, to fromat a value using the {:?} formatter.
*/


// `Centimeters`, tuple struct that can be compared

#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centementers(&self) -> Centimeters {
        let &Inches(inches) = self;
        Centimeters(inches as f64 * 2.54 )
    }}

struct Seconds(i32);



