fn main() {
    // a variable's lifetime begins when it's created and ends when it's dropped.
    // syntax = 'name, general use -> 'a, 'b, 'c 
}

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        return s1;
    } else {
        s2
    }
    // in this function, both paramaters and the return type share same lifetime 'a
    // means that lifetime of the returned reference will be the same as the shortest lifetime of input references
}


struct Sentence<'a> {
    content: &'a str,
}

impl<'a> Sentence<'a> {
    fn yell(&self) -> &str {
        return "Do not sleep till 11pm";
    }
}

fn no_no_function<'a>(x: &'a str, y: &'a str) -> &'a str {
    let some_string = String::from(x);
    &some_string // just lives in this function scope
}

// Lifetime Elision Rules
// 1. Each parameter that's a reference gets it's own lifetime parameter
// -- fn eli(x: &str, y: &str) -> assumes x: &'a str, y: &'b str
// 2. If there's exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.
// 3.  If there are multiple input lifetime parameters, but one of them is &self or &mut self,
// the lifetime of self is assigned to all output lifetime parameters. 

