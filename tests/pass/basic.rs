use dynosaur;

#[dynosaur::make_dyn(dyn)]
trait MyTrait {
    type Item;
    async fn foo(&self) -> Self::Item;
}

fn main() {}
