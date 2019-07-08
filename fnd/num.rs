pub trait Real
{
    fn zero() -> Self;
}

impl Real for f32
{
    #[inline]
    fn zero() -> f32
    {
        0.0f32
    }
}

impl Real for f64
{
    #[inline]
    fn zero() -> f64
    {
        0.0f64
    }
}
