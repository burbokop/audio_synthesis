use std::ops::{Div, Add, Mul};

use num::{Float, One};







pub fn sin_generator(len: usize, freq: f32) -> impl Iterator<Item=f32> {
    (0..len).map(move |x| (x as f32 * freq).sin())
}

pub fn triangular_generator(len: usize, freq: f32) -> impl Iterator<Item=f32> {
    (0..len).map(move |x| (x as f32 * freq) % 2. - 1.)
}





pub struct Deteriorator<I>
where
    I: Iterator,
{
    divider: I::Item,
    i: I,
}

impl<I> Iterator for Deteriorator<I>
where
    I: Iterator,
    I::Item: Float
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.i.next().map(|x| (x * self.divider).floor() / self.divider)
    }
}

pub trait DeterioratorExt: Iterator {
    fn deteriorated(self, divider: Self::Item) -> Deteriorator<Self>
    where
        Self::Item: Float,
        Self: Sized,
    {
        Deteriorator {
            divider: divider,
            i: self,
        }
    }
}

impl<I: Iterator> DeterioratorExt for I {}










pub struct Integrator<I>
where
    I: Iterator,
{
    alpha: I::Item,
    prev: Option<I::Item>,
    i: I,
}



//return lastY = alpha * lastY + (1 - alpha) * value);

impl<I> Iterator for Integrator<I>
where
    I: Iterator,
    I::Item: Float + One
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.i.next().map(|x| {
            self.prev = match self.prev {
                Some(p) => Some(self.alpha * p + (I::Item::one() - self.alpha) * x),
                None => Some(x),
            };
            self.prev.unwrap()
        })
    }
}

pub trait IntegratorExt: Iterator {
    fn integrated(self, alpha: Self::Item) -> Integrator<Self>
    where
        Self::Item: Float,
        Self: Sized,
    {
        Integrator {
            alpha: alpha,
            prev: None,
            i: self,
        }
    }
}

impl<I: Iterator> IntegratorExt for I {}






pub struct Adder<I0, I1>
where
    I0: Iterator,
    I1: Iterator,
{
    i0: I0,
    i1: I1,
}



//return lastY = alpha * lastY + (1 - alpha) * value);

impl<I0, I1> Iterator for Adder<I0, I1>
where
    I0: Iterator,
    I1: Iterator,
    I0::Item: Add<I1::Item>
{
    type Item = <I0::Item as Add<I1::Item>>::Output;

    fn next(&mut self) -> Option<Self::Item> {
        let n0 = self.i0.next();
        let n1 = self.i1.next();

        match n0.zip(n1) {
            Some(r) => Some(r.0 + r.1),
            None => None,
        } 

         
        /* experimental 
        if let Some(r0) = n0 && let Some(r1) = n1 { 
            Some(r0 + r1)
        } else if let Some(r0) = n0 {
            Some(r0)
        } else if let Some(r1) = n1 {
            Some(r1)
        } else {
            None
        }
        */
    }
}

pub trait AdderExt<O>: Iterator {
    fn add(self, other: O) -> Adder<Self, O>
    where
        O: Iterator,
        Self: Sized,
    {
        Adder {
            i0: self,
            i1: other
        }
    }
}

impl<I0: Iterator, I1: Iterator> AdderExt<I1> for I0 {}






pub struct Amplifier<I>
where
    I: Iterator,
{
    multiplier: I::Item,
    i: I,
}

impl<I> Iterator for Amplifier<I>
where
    I: Iterator,
    I::Item: Mul<I::Item, Output = I::Item> + Clone
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.i.next().map(|x| x * self.multiplier.clone())
    }
}

pub trait AmplifierExt: Iterator {
    fn amplified(self, multiplier: Self::Item) -> Amplifier<Self>
    where
        Self::Item: Float,
        Self: Sized,
    {
        Amplifier {
            multiplier: multiplier,
            i: self,
        }
    }
}

impl<I: Iterator> AmplifierExt for I {}
