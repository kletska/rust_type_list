use crate::type_list::*;

pub trait TypeTree {
    type Constructor;
}

#[derive(Debug)]
pub struct Leaf<T>(pub T);

impl<T> TypeTree for Leaf<T> {
    type Constructor = Self;
}

#[derive(Debug)]
pub struct Node<N, T>(pub N, pub T);

impl<N, T: TypeList> TypeTree for Node<N, T> {
    type Constructor = Self;
}

impl<I, N, T: TypeList + Cons<I>> Cons<I> for Node<N, T> {
    type Res = Node<N, <T as Cons<I>>::Res>;
    fn cons(self, item: I) -> <Self as Cons<I>>::Res {
        Node(self.0, self.1.cons(item))
    }
}

pub trait Root {
    type Res;
    fn root(self) -> <Self as Root>::Res;
}

impl<T> Root for T {
    type Res = Node<T, ()>;
    fn root(self) -> <Self as Root>::Res {
        Node(self, ())
    }
}

pub struct Save<T, S>(pub T, pub S); 

/*
impl<N1, T1, N2, T2, T, S> TypeTree for Save<T, S> where 
    T1: TypeList,
    T2: TypeList,
    T: TypeTree<Constructor = Node<N1, T1>>,
    S: TypeTree<Constructor = Node<N2, T2>>
{
    type Constructor = Self;
}
*/

impl<I, T: TypeTree + Cons<I>, S> Cons<I> for Save<T, S> where {
    type Res = Save<<T as Cons<I>>::Res, S>;
    fn cons(self, item: I) -> <Self as Cons<I>>::Res {
        Save(self.0.cons(item), self.1)
    }
}

pub trait Parent<I>: Cons<Leaf<I>> {
    type Res;
    fn add_child(self, item: I) -> <Self as Parent<I>>::Res;

    fn add_leaf(self, item: I) -> <Self as Cons<Leaf<I>>>::Res;
}

impl<I, N: TypeTree + Cons<Leaf<I>>> Parent<I> for N {
    type Res = Save<Node<I, ()>, N>;

    fn add_child(self, item: I) -> <Self as Parent<I>>::Res {
        Save(Node(item, ()), self)
    }

    fn add_leaf(self, item: I) -> <Self as Cons<Leaf<I>>>::Res {
        self.cons(Leaf(item))
    }
}

impl<I, T: TypeTree + Cons<Leaf<I>>, S> Parent<I> for Save<T, S> {
    type Res = Save<Node<I, ()>, Self>;
    
    fn add_child(self, item: I) -> <Self as Parent<I>>::Res {
        Save(Node(item, ()), self)
    }

    fn add_leaf(self, item: I) -> <Self as Cons<Leaf<I>>>::Res {
        self.cons(Leaf(item))
    }
}
pub trait Back {
    type Res;
    fn back(self) -> <Self as Back>::Res;
}

impl<I, S> Back for Save<I, S> where 
    S: Cons<I>
{
    type Res = <S as Cons<I>>::Res;
    fn back(self) -> <Self as Back>::Res {
        self.1.cons(self.0)
    }
}





