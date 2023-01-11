use super::HGT;
// category-theory-for-programmers challenge 1.4
trait Morphism<'a, A, B>: HGT {
    type Output<G: 'a + Copy + Morphism<'a, B, C, F<C> = Self::F<C>>, C>;
    fn compose<G, C>(self, g: G) -> Self::Output<G, C>
    where
        G: 'a + Morphism<'a, B, C, F<C> = Self::F<C>> + Copy;
    fn eval(self, a: A) -> Self::F<B>;
}
trait Category<'a> {
    type M<T: Copy + 'a>: Morphism<'a, T, T> + Copy;
    fn id<T: Copy + 'a>() -> Self::M<T>;
}
#[derive(Copy, Clone)]
struct Function<A, B, F: Fn(A) -> B + Copy> {
    f: F,
    _marker: std::marker::PhantomData<(A, B)>,
}

impl<A, B, F: Fn(A) -> B + Copy> Function<A, B, F> {
    fn new(f: F) -> Self {
        Function {
            f,
            _marker: std::marker::PhantomData,
        }
    }
}
impl<A, B, F: Fn(A) -> B + Copy> HGT for Function<A, B, F> {
    type F<T> = T;
}

impl<'a, A: Copy + 'a, B: Copy + 'a, F: Fn(A) -> B + Copy + 'a> Morphism<'a, A, B>
    for Function<A, B, F>
{
    type Output<G: 'a + Copy + Morphism<'a, B, C, F<C> = Self::F<C>>, C> =
        Function<A, C, impl Fn(A) -> C + Copy + 'a>;
    fn compose<G, C>(self, g: G) -> Self::Output<G, C>
    where
        G: 'a + Morphism<'a, B, C, F<C> = Self::F<C>> + Copy,
    {
        Function::new(move |a| g.eval(self.eval(a)))
    }
    fn eval(self, a: A) -> Self::F<B> {
        (self.f)(a)
    }
}

struct Set;
impl<'a> Category<'a> for Set {
    type M<T: Copy + 'a> = Function<T, T, impl Fn(T) -> T + Copy + 'a>;
    fn id<T: Copy + 'a>() -> Self::M<T> {
        Function::new(move |a: T| a)
    }
}

use super::monad::*;
#[derive(Copy, Clone)]
struct OptionKleisiliArrow<A, B, F: Fn(A) -> Option<B> + Copy> {
    f: F,
    _marker: std::marker::PhantomData<(A, B)>,
}

impl<A, B, F: Fn(A) -> Option<B> + Copy> OptionKleisiliArrow<A, B, F> {
    fn new(f: F) -> Self {
        OptionKleisiliArrow {
            f,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<A, B, F: Fn(A) -> Option<B> + Copy> HGT for OptionKleisiliArrow<A, B, F> {
    type F<T> = Option<T>;
}

impl<'a, A: Copy + 'a, B: Copy + 'a, F: Fn(A) -> Option<B> + Copy + 'a> Morphism<'a, A, B>
    for OptionKleisiliArrow<A, B, F>
{
    type Output<G: 'a + Copy + Morphism<'a, B, C, F<C> = Self::F<C>>, C> =
        OptionKleisiliArrow<A, C, impl Fn(A) -> Option<C> + Copy + 'a>;
    fn compose<G, C>(self, g: G) -> Self::Output<G, C>
    where
        G: 'a + Morphism<'a, B, C, F<C> = Self::F<C>> + Copy,
    {
        OptionKleisiliArrow::new(move |a| self.eval(a).bind(|b| g.eval(b)))
    }
    fn eval(self, a: A) -> Self::F<B> {
        (self.f)(a)
    }
}

struct OptionKleisili;
impl<'a> Category<'a> for OptionKleisili {
    type M<T: Copy + 'a> = OptionKleisiliArrow<T, T, impl Fn(T) -> Option<T> + Copy + 'a>;
    fn id<T: Copy + 'a>() -> Self::M<T> {
        OptionKleisiliArrow::new(move |a: T| Option::pure(a))
    }
}
use super::state::*;
#[derive(Copy, Clone)]
struct StateKleisiliArrow<'a, A, B, S, F: Fn(A) -> MState<'a, B, S> + Copy> {
    f: F,
    _marker: std::marker::PhantomData<(A, B, S)>,
}

impl<'a, A, B, S, F: Fn(A) -> MState<'a, B, S> + Copy> StateKleisiliArrow<'a, A, B, S, F> {
    fn new(f: F) -> Self {
        StateKleisiliArrow {
            f,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<'a, A, B, S, F: Fn(A) -> MState<'a, B, S> + Copy> HGT for StateKleisiliArrow<'a, A, B, S, F> {
    type F<T> = MState<'a, T, S>;
}

impl<'a, A: Copy + 'a, B: Copy + 'a, S: Copy + 'a, F: Fn(A) -> MState<'a, B, S> + Copy + 'a>
    Morphism<'a, A, B> for StateKleisiliArrow<'a, A, B, S, F>
{
    type Output<G: 'a + Copy + Morphism<'a, B, C, F<C> = Self::F<C>>, C> =
        StateKleisiliArrow<'a, A, C, S, impl Fn(A) -> MState<'a, C, S> + Copy>;
    fn compose<G, C>(self, g: G) -> Self::Output<G, C>
    where
        G: 'a + Morphism<'a, B, C, F<C> = Self::F<C>> + Copy,
    {
        StateKleisiliArrow::new(move |a| self.eval(a).bind(move |b| g.eval(b)))
    }
    fn eval(self, a: A) -> Self::F<B> {
        (self.f)(a)
    }
}

struct StateKleisili<S>(std::marker::PhantomData<S>);
impl<'a, S: Copy + 'a> Category<'a> for StateKleisili<S> {
    type M<T: Copy + 'a> =
        StateKleisiliArrow<'a, T, T, S, impl Fn(T) -> MState<'a, T, S> + Copy + 'a>;
    fn id<T: Copy + 'a>() -> Self::M<T> {
        StateKleisiliArrow::new(move |a: T| MState::pure(a))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn id_test() {
        let a = 1;
        assert_eq!(a, Set::id().eval(a));
    }

    #[test]
    fn associative_test() {
        let f = Function::new(|a: u32| -> f32 { (a + 3) as f32 });
        let g = Function::new(|a: f32| -> f64 { (a / 8.0) as f64 });
        let h = Function::new(|a: f64| -> u64 { (a * a) as u64 });
        let l_ass = f.compose(g).compose(h);
        let r_ass = f.compose(g.compose(h));
        let check = |a: u32| {
            println!(
                "l_ass.eval({}) = {}, r_ass.eval({}) = {}",
                a,
                l_ass.eval(a),
                a,
                r_ass.eval(a)
            );
            assert_eq!(l_ass.eval(a), r_ass.eval(a));
        };
        check(1);
        check(16);
        check(29);
    }

    #[test]
    fn uint_test() {
        let f = Function::new(|a: u32| -> f32 { (a + 3) as f32 });
        let l_unit = Set::id().compose(f);
        let r_unit = f.compose(Set::id());
        let check = |a: u32| {
            println!(
                "l_unit.eval({}) = {}, r_unit.eval({}) = {}, f({}).eval = {}",
                a,
                l_unit.eval(a),
                a,
                r_unit.eval(a),
                a,
                f.eval(a)
            );
            assert_eq!(l_unit.eval(a), r_unit.eval(a));
            assert_eq!(l_unit.eval(a), f.eval(a));
            assert_eq!(r_unit.eval(a), f.eval(a));
        };
        check(1);
        check(16);
        check(29);
    }
}
