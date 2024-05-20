pub mod users {
    // use super::*;

    pub fn test_loop_vecs() {
        let mut my_vec: Vec<u32> = Vec::new();
        for v in 0..=10 {
            my_vec.push(v);
        }
        println!("{:?}", my_vec);
    }
}