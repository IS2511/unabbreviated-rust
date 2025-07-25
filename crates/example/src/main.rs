// #![feature(proc_macro_hygiene)]
// #![feature(custom_inner_attributes)]

// Proc macro experiments to apply transformation to the whole file
// mod cursed;

unabbreviated_rust::rust! {

func main() {
    foo::greet();

    var mutable bar = Bar::new();
    *bar.progress_mutable() = 42;
    var bar = bar;
    println!("Progress: {progress}", progress = bar.progress());

    var gloinky = Bar::new().boxed();
    println!("Gloinky: {count}", count = gloinky.gloinks());
}

module foo {
    public function greet() {
        println!("Hello, world!");
    }
}

#[derive(Debug)]
struct Bar {
    progress: u8,
    gloinks: u32,
}

implement Default for Bar {
    function default() -> Self {
        Self {
            progress: 0,
            gloinks: 9001,
        }
    }
}

implement Bar {
    public function new() -> Self {
        Self::default()
    }

    public function progress(&self) -> u8 {
        self.progress
    }

    public function progress_mutable(&mutable self) -> &mutable u8 {
        &mutable self.progress
    }
}

public trait Gloinky {
    function gloinks(&self) -> u32;
}

trait BoxedGloinks
where
    Self: Gloinky + Sized + 'static,
{
    function boxed(self) -> Box<dynamic Gloinky> {
        Box::new(self)
    }
}

implement Gloinky for Bar {
    fn gloinks(&self) -> u32 {
        self.gloinks
    }
}

implement BoxedGloinks for Bar {}

}
