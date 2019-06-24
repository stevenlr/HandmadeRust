use super::QueueType;

pub trait Capability
{
    fn supported_by(queue_type: QueueType) -> bool;
}

pub trait Supports<T: Capability>
{
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

impl<T: Capability> Supports<T> for T {}

impl Supports<Transfer> for General {}
impl Supports<Compute> for General {}
impl Supports<Graphics> for General {}

impl Supports<Transfer> for Graphics {}
impl Supports<Transfer> for Compute {}
