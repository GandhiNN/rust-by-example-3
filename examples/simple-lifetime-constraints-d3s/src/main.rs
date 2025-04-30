struct House {
    garage: Garage,
}

#[derive(Debug)]
struct Garage {}

impl House {
    fn garage(&self) -> &Garage {
        &self.garage
    }
}

fn main() {
    let g = Garage {};
    let h = House { garage: g };
    let gh = h.garage();
    println!("{:#?}", gh);
}
