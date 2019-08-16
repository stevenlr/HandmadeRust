use super::QueueType;

pub trait Capability
{
    fn supported_by(queue_type: QueueType) -> bool;
}

pub struct Compute;
pub struct Graphics;
pub struct Transfer;
pub struct General;

impl Capability for Compute
{
    #[inline]
    fn supported_by(queue_type: QueueType) -> bool
    {
        match queue_type
        {
            QueueType::Compute => true,
            QueueType::General => true,
            _ => false,
        }
    }
}

impl Capability for Graphics
{
    #[inline]
    fn supported_by(queue_type: QueueType) -> bool
    {
        match queue_type
        {
            QueueType::General => true,
            QueueType::Graphics => true,
            _ => false,
        }
    }
}

impl Capability for Transfer
{
    #[inline]
    fn supported_by(queue_type: QueueType) -> bool
    {
        match queue_type
        {
            QueueType::Compute => true,
            QueueType::General => true,
            QueueType::Graphics => true,
            QueueType::Transfer => true,
        }
    }
}

impl Capability for General
{
    #[inline]
    fn supported_by(queue_type: QueueType) -> bool
    {
        match queue_type
        {
            QueueType::General => true,
            _ => false,
        }
    }
}

pub trait Supports<T>
{
}

impl<C: Capability> Supports<C> for C {}
impl Supports<Transfer> for Compute {}
impl Supports<Transfer> for Graphics {}
impl Supports<Transfer> for General {}
impl Supports<Graphics> for General {}
impl Supports<Compute> for General {}

pub struct Primary;
pub struct Secondary;

pub trait Level
{
}

impl Level for Primary {}
impl Level for Secondary {}
