fn main() {
    let two = zero_even.Next();
    let three = two.EvenPlusOne();
    let one = one_odd;
    let four = three.OddPlusOdd(one);
    let five: <Odd<S<S<S<Z>>>> as Next>::Output = three.Next();
    let eight = four.EvenPlusEven(four);
}

#[derive(Copy, Clone, Debug)]
struct Z;

#[derive(Copy, Clone, Debug)]
struct S<T>(T);

use std::ops::Add;

impl<M> Add<M> for Z {
    type Output = M;

    fn add(self, rhs: M) -> M {
        rhs
    }
}

impl<M, N, O> Add<M> for S<N>
where
    N: Add<M, Output = O>,
{
    type Output = S<O>;

    fn add(self, rhs: M) -> S<O> {
        S(self.0 + rhs)
    }
}

#[derive(Copy, Clone, Debug)]
struct Even<T>(T);
#[derive(Copy, Clone, Debug)]
struct Odd<T>(T);

type ZeroEven = Even<Z>;
const zero_even: ZeroEven = Even(Z);
type OneOdd = Odd<S<Z>>;
const one_odd: OneOdd = Odd(S(Z));

trait Next {
    type Output;

    fn Next(&self) -> Self::Output;
}

impl<T: Copy> Next for Even<T> {
    type Output = Even<S<S<T>>>;

    fn Next(&self) -> Even<S<S<T>>> {
        Even(S(S(self.0)))
    }
}

impl<T: Copy> Next for Odd<T> {
    type Output = Odd<S<S<T>>>;

    fn Next(&self) -> Odd<S<S<T>>> {
        Odd(S(S(self.0)))
    }
}

trait Prev<T> {
    type Output;

    fn Prev(&self) -> Self::Output;
}

impl<T: Copy> Prev<T> for <Even<T> as Next>::Output {
    type Output = Even<T>;

    fn Prev(&self) -> Self::Output {
        Even(((self.0).0).0)
    }
}

impl<T: Copy> Prev<T> for <Odd<T> as Next>::Output {
    type Output = Odd<T>;

    fn Prev(&self) -> Self::Output {
        Odd(((self.0).0).0)
    }
}

trait OddPlusOne<T> {
    type Output;

    fn OddPlusOne(&self) -> Self::Output;
}

impl<T> OddPlusOne<T> for OneOdd {
    type Output = <ZeroEven as Next>::Output;

    fn OddPlusOne(&self) -> Self::Output {
        Even(Z).Next()
    }
}

impl<T: Copy> OddPlusOne<T> for <Odd<T> as Next>::Output {
    type Output = <Even<S<T>> as Next>::Output;

    fn OddPlusOne(&self) -> Self::Output {
        Even((self.0).0).Next()
    }
}

trait EvenPlusOne<T> {
    type Output;

    fn EvenPlusOne(&self) -> Self::Output;
}

impl<T> EvenPlusOne<T> for ZeroEven {
    type Output = OneOdd;

    fn EvenPlusOne(&self) -> Self::Output {
        Odd(S(Z))
    }
}

impl<T: Copy> EvenPlusOne<T> for <Even<T> as Next>::Output {
    type Output = <Odd<S<T>> as Next>::Output;

    fn EvenPlusOne(&self) -> Self::Output {
        Odd((self.0).0).Next()
    }
}

trait EvenPlusEven<M, N> {
    type Output;

    fn EvenPlusEven(&self, rhs: M) -> Self::Output;
}

impl<T: Copy> EvenPlusEven<Even<T>, T> for ZeroEven {
    type Output = Even<T>;

    fn EvenPlusEven(&self, rhs: Even<T>) -> Self::Output {
        rhs
    }
}

impl<T: Copy, U: Copy, X> EvenPlusEven<Even<T>, X> for Even<S<S<U>>>
where
    Even<U>: EvenPlusEven<Even<T>, X>,
    <Even<U> as EvenPlusEven<Even<T>, U>>::Output: Next,
{
    type Output = <<Even<U> as EvenPlusEven<Even<T>, U>>::Output as Next>::Output;

    fn EvenPlusEven(&self, rhs: Even<T>) -> Self::Output {
        self.Prev().EvenPlusEven(rhs).Next()
    }
}

trait OddPlusOdd<M, N> {
    type Output;

    fn OddPlusOdd(&self, rhs: M) -> Self::Output;
}

impl<T: Copy> OddPlusOdd<Odd<T>, T> for OneOdd
where
    Odd<T>: OddPlusOne<T>,
{
    type Output = <Odd<T> as OddPlusOne<T>>::Output;

    fn OddPlusOdd(&self, rhs: Odd<T>) -> Self::Output {
        rhs.OddPlusOne()
    }
}

impl<T: Copy, U: Copy, X> OddPlusOdd<Odd<T>, X> for Odd<S<S<U>>>
where
    Odd<U>: OddPlusOdd<Odd<T>, X>,
    <Odd<U> as OddPlusOdd<Odd<T>, U>>::Output: Next,
{
    type Output = <<Odd<U> as OddPlusOdd<Odd<T>, U>>::Output as Next>::Output;

    fn OddPlusOdd(&self, rhs: Odd<T>) -> Self::Output {
        self.Prev().OddPlusOdd(rhs).Next()
    }
}
