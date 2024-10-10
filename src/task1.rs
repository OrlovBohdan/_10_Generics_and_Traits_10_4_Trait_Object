#[test]

/*

trait Bird {
    fn quack(&self) -> String;
}

struct Duck;
impl Duck {
    fn swim(&self) {
        println!("Look, the duck is swimming")
    }
}
struct Swan;
impl Swan {
    fn fly(&self) {
        println!("Look, the duck.. oh sorry, the swan is flying")
    }
}

impl Bird for Duck {
    fn quack(&self) -> String{
        "duck duck".to_string()
    }
}

impl Bird for Swan {
    fn quack(&self) -> String{
        "swan swan".to_string()
    }
}

fn main() {
    // FILL in the blank.
    let duck = __;
    duck.swim();

    let bird = hatch_a_bird(2);
    // This bird has forgotten how to swim, so below line will cause an error.
    // bird.swim();
    // But it can quak.
    assert_eq!(bird.quack(), "duck duck");

    let bird = hatch_a_bird(1);
    // This bird has forgotten how to fly, so below line will cause an error.
    // bird.fly();
    // But it can quak too.
    assert_eq!(bird.quack(), "swan swan");

    println!("Success!");
}

// IMPLEMENT this function.
fn hatch_a_bird...

*/

fn main() {
    // Инициализация утки.
    let duck = Duck;
    duck.swim();

    let bird = hatch_a_bird(2);
    // Эта птица умеет крякать.
    assert_eq!(bird.quack(), "duck duck");

    let bird = hatch_a_bird(1);
    // Эта птица тоже умеет крякать.
    assert_eq!(bird.quack(), "swan swan");

    println!("Success!");
}


trait Bird {
    fn quack(&self) -> String;
}

struct Duck;
impl Duck {
    fn swim(&self) {
        println!("Look, the duck is swimming");
    }
}
struct Swan;
#[allow(dead_code)]
impl Swan {
    fn fly(&self) {
        println!("Look, the duck.. oh sorry, the swan is flying");
    }
}

impl Bird for Duck {
    fn quack(&self) -> String {
        "duck duck".to_string()
    }
}

impl Bird for Swan {
    fn quack(&self) -> String {
        "swan swan".to_string()
    }
}



// Реализация функции hatch_a_bird
fn hatch_a_bird(n: u32) -> Box<dyn Bird> {
    if n == 2 {
        Box::new(Duck) // Возвращаем объект Duck в Box<dyn Bird>
    } else {
        Box::new(Swan) // Возвращаем объект Swan в Box<dyn Bird>
    }
}

/*
Вставить правильную инициализацию объекта типа Duck для переменной duck в функции main.
Реализовать функцию hatch_a_bird, которая возвращает объект, реализующий трейт Bird,
используя динамическое распределение через Box<dyn Bird>, чтобы можно было возвращать либо Duck, либо Swan.

Переменная duck: Создаем экземпляр Duck напрямую. Метод swim доступен только для объектов типа Duck,
а не для объектов, реализующих трейт Bird.
Функция hatch_a_bird: В зависимости от переданного числа создается объект Duck или Swan,
который помещается в Box<dyn Bird>, чтобы воспользоваться динамическим диспетчеризацией трейта Bird.
*/

