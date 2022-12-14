use super::functor::*;
use super::monad::*;
use super::HGT;
pub struct MState<'a, A, S> {
    pub run: Box<dyn 'a + Fn(S) -> (A, S)>,
}
impl<'a, A, S> std::fmt::Debug for MState<'a, A, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(
            f,
            "State {{run : {}}}",
            std::any::type_name::<Box<dyn 'a + Fn(S) -> (A, S)>>()
        )
    }
}

impl<'a, A, S> MState<'a, A, S> {
    fn new<F: 'a + Fn(S) -> (A, S)>(f: F) -> Self {
        MState { run: Box::new(f) }
    }
    pub fn run(&self, s: S) -> (A, S) {
        (self.run)(s)
    }
    pub fn eval(&self, s: S) -> A {
        self.run(s).0
    }
}

impl<'a, A, S> HGT for MState<'a, A, S> {
    type F<B> = MState<'a, B, S>;
}

impl<'a, A: 'a, S: 'a> Functor<'a, A> for MState<'a, A, S> {
    fn fmap<B, F>(self, f: F) -> Self::F<B>
    where
        F: Copy + Fn(A) -> B + 'a,
    {
        MState::new(move |s| {
            let (a, s) = self.run(s);
            (f(a), s)
        })
    }
}

impl<'a, A: 'a + Copy, S: 'a> Monad<'a, A> for MState<'a, A, S> {
    fn pure(a: A) -> Self {
        MState::new(move |s| (a, s))
    }
    fn bind<B, F>(self, f: F) -> Self::F<B>
    where
        F: Copy + Fn(A) -> Self::F<B> + 'a,
    {
        MState::new(move |s| {
            let (a, s) = self.run(s);
            f(a).run(s)
        })
    }
}

pub fn get<'a, S>() -> MState<'a, S, S>
where
    S: Clone,
{
    MState::new(|s: S| (s.clone(), s))
}
pub fn put<'a, S>(s: S) -> MState<'a, (), S>
where
    S: Copy + 'a,
{
    MState::new(move |_| ((), s))
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fib<'a>(n: u64) -> MState<'a, u64, (u64, u64)> {
        match n {
            0 => get().bind(move |x: (u64, u64)| {
                println!("fib({}): {:?} -> {:?}", n, x, x.0);
                MState::pure(x.0)
            }),
            _ => get()
                .bind(move |x: (u64, u64)| {
                    println!("fib({}): {:?} -> {:?}", n, x, (x.1, x.0 + x.1));
                    put((x.1, x.0 + x.1))
                })
                .bind(move |_| fib(n - 1)),
        }
    }
    #[test]
    fn state_monad_test() {
        let d = fib(8).eval((0, 1));
        println!("result = {}", d);
    }
}
