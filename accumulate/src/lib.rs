/// What should the type of _function be?
pub fn map<F, T, U>(input: Vec<F>, mut _function: T) -> Vec<U>
where
    T: FnMut(F) -> U,
{
    // unimplemented!("Transform input vector {:?} using passed function", input);
    let mut ans = Vec::<U>::new();
    input.into_iter().for_each(|x| ans.push(_function(x)));
    ans
    // input.into_iter().map(_function).collect()
}
