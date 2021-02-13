pub trait TypeList {
    type Constructor;
} 

impl TypeList for () {
    type Constructor = Self;
}

impl<T> TypeList for (T,) {
    type Constructor = Self;
}

impl<H, T: TypeList> TypeList for (H, T) {
    type Constructor = Self;
}

pub trait Cons<I> {
    type Res;
    fn cons(self, item: I) -> <Self as Cons<I>>::Res;
}

impl<I> Cons<I> for () {
    type Res = (I,);
    fn cons(self, item: I) -> <Self as Cons<I>>::Res {
        (item,)
    }
}

impl<I, T> Cons<I> for (T,) {
    type Res = (I, Self);
    fn cons(self, item: I) -> <Self as Cons<I>>::Res {
        (item, self)
    }
} 

impl<I, H, T: TypeList> Cons<I> for (H, T) {
    type Res = (I, Self);
    fn cons(self, item: I) -> <Self as Cons<I>>::Res {
        (item, self)
    }
}

/*
pub trait Append<I> {
    type Res;
    fn append(self, item: I) -> <Self as Append<I>>::Res;
}

impl<I> Append<I> for () {
    type Res = (I,);
    fn append(self, item: I) -> <Self as Append<I>>::Res {
        (item,)
    }
}

impl<I, T> Append<I> for (T,) {
    type Res = (T, (I,));
    fn append(self, item: I) -> <Self as Append<I>>::Res {
        (self.0, (item,))
    }
}

impl<I, H, T: TypeList + Append<I>> Append<I> for (H, T) {
    type Res = (H, <T as Append<I>>::Res);
    fn append(self, item: I) -> <Self as Append<I>>::Res {
        (self.0, self.1.append(item))
    }
}

*/









