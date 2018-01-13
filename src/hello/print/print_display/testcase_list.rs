// 1.2.2.1  测试实例 list

use std::fmt;

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let List(ref vec) = *self;

        try!(write!(f, "["));

        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                try!(write!(f, ", "));
            }
            try!(write!(f, "{}: {}", count, v));
        }
        
        write!(f, "]")
    }
}

pub fn run(){
    let v = List(vec![1,2,3]);
    println!("{}", v);
}