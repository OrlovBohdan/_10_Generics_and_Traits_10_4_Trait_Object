#[test]

/*
trait Bird {
    fn quack(&self);
}

struct Duck;
impl Duck {
    fn fly(&self) {
        println!("Look, the duck is flying")
    }
}
struct Swan;
impl Swan {
    fn fly(&self) {
        println!("Look, the duck.. oh sorry, the swan is flying")
    }
}

impl Bird for Duck {
    fn quack(&self) {
        println!("{}", "duck duck");
    }
}

impl Bird for Swan {
    fn quack(&self) {
        println!("{}", "swan swan");
    }
}

fn main() {
    // FILL in the blank to make the code work.
    let birds __;

    for bird in birds {
        bird.quack();
        // When duck and swan turn into Birds, they all forgot how to fly, only remember how to quack.
        // So, the code below will cause an error.
        // bird.fly();
    }
}
*/


fn main() {
    // Создаем вектор объектов, реализующих трейт Bird, используя Box<dyn Bird>.
    let birds: Vec<Box<dyn Bird>> = vec![Box::new(Duck), Box::new(Swan)];

    for bird in birds {
        bird.quack();
        // Птицы забыли, как летать, потому что они теперь типажные объекты Bird.
    }
}
trait Bird {
    fn quack(&self);
}

struct Duck;
#[allow(dead_code)]
impl Duck {
    fn fly(&self) {
        println!("Look, the duck is flying")
    }
}
struct Swan;
#[allow(dead_code)]
impl Swan {
    fn fly(&self) {
        println!("Look, the duck.. oh sorry, the swan is flying")
    }
}

impl Bird for Duck {
    fn quack(&self) {
        println!("{}", "duck duck");
    }
}

impl Bird for Swan {
    fn quack(&self) {
        println!("{}", "swan swan");
    }
}


