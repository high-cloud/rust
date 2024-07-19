struct F<A, O>(fn(A) -> O);

trait Fun {
    type O;
    type A;
}

impl<A, O> Fun for F<A, O> {
    type O = O;
    type A = A;
}

trait Gen {
    type B;
}

fn a() {
    b(F(c::<()>));
}

fn b<G, B>(_: G)
where
    G: Fun<A = B>,
    <G as Fun>::O: Gen<B=B>,
{
}

fn c<T>(_: T) -> impl Gen {}

impl Gen for () {
    type B = ();
}