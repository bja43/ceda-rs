use std::usize;
use rand::Rng;


#[derive(Debug, PartialEq, PartialOrd)]
struct GSNode {
    grow: f64,
    branches: Vec<GSNode>,
}


impl GSNode {
    fn new(grow: f64) -> Self {
        return GSNode { grow, branches: Vec::new() };
    }

    fn grow(&mut self, available: Vec<usize>) -> () {

        let mut rng = rand::thread_rng();   
        
        for _ in available {
            let score = rng.gen();
            if score > self.grow {
                self.branches.push(GSNode::new(score))
            }
        }

        self.branches.sort_by(|a, b| b.grow.partial_cmp(&a.grow).unwrap());
    }
}


fn main() {

    let mut order = Vec::new();
    for i in 0..10 {
        order.push(i);
    }

    println!("Order: {:?}\n", order);

    let sub_order: &mut [usize] = &mut order[2..6];
    sub_order.reverse();

    println!("Order with reversed suborder: {:?}\n", order);

    let mut rng = rand::thread_rng();
    let mut root = GSNode::new(rng.gen());
    root.grow(order);

    println!("Grow Shrink Tree: {:#?}\n", root);

}
