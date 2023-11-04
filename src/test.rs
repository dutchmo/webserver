trait Vehicle {
fn drive(&self);
}

struct Truck;

impl Vehicle for Truck {
    fn drive(&self) {
        todo!()
    }
}

fn test() {
    let t: Box<dyn Vehicle>;
    //t = Truck;
    //t.drive();


}