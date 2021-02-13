trait TTree {}

impl TTree for () {}

impl<H, T: TTree> TTree for (H, T) {}

impl<T: TTree> TTree for (T,) {}

trait Cons<T>: Sized {
    fn cons(self, list: T) -> (T, Self);
}

impl<D> Cons<D> for () {
    fn cons(self, item: D) -> (D, Self) {
        (item, self)
    }
}

impl<D, H, T: TTree> Cons<D> for (H, T) {
    fn cons(self, item: D) -> (D, Self) {
        (item, self)
    }
}

impl<D, S: TTree> Cons<D> for (S,) {
    fn cons(self, item: D) -> (D, Self) {
        (item, self)
    }
}


trait Child: Sized {
    fn add_child(self) -> (Self,) {
        (self,)
    }
}

impl Child for () {}

impl<H, T: TTree> Child for (H, T) {}

impl<S: TTree> Child for (S,) {}

trait Back: Sized {
    type Res;
    fn back(self) -> Self::Res;
}

impl Back for () {
    type Res = ();
    fn back(self) -> <Self as Back>::Res {
        self
    }
}

impl <H, A: TTree, B: TTree, T: TTree + Back<Res = (A, B)>> Back for (H, T) {
    type Res = ((H, A), B);
    fn back(self) -> <Self as Back>::Res {
        let res = self.1.back();
        ((self.0, res.0), res.1)
    }
}

impl<S: TTree> Back for (S,) {
    type Res = ((), S);
    fn back(self) -> Self::Res {
        ((), self.0)
    }
}
//====================



//====================

fn main() {
    let val = ()
        .cons(1)
        .cons(().cons(2))
        .cons(().cons(3))
        ; // non linear build

    let val2 = ()
        .cons(1)
        .add_child()
        .cons(2)
        .add_child()
        .back()
        .add_child()
        .cons(3)
        .back()
        ; // linear build

    println!("{:?}", val); //((3, (2, ())), (1, ()))

    println!("{:?}", val2);//((3, (2, ())), (1, ()))

    let val3 = ()
        .cons("I am root")
        .add_child()
        .cons("I am first root child")
        .back()
        .add_child()
        .cons("I am second root child")
        .add_child()
        .cons("I am child of second root child")
        .back()
        .add_child()
        .cons("I am second child of second root child")
        .back()
        .back()
        ;

    println!("{:#?}", val3)
}
