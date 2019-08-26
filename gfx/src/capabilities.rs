use super::QueueType as QueueTypeEnum;

pub trait Capability
{
}

pub trait QueueType
{
    fn supported_by(queue_type: QueueTypeEnum) -> bool;
}

pub struct Compute;
pub struct Graphics;
pub struct Transfer;
pub struct General;
pub struct GraphicsOrCompute;

impl Capability for Compute {}
impl Capability for Graphics {}
impl Capability for Transfer {}
impl Capability for General {}
impl Capability for GraphicsOrCompute {}

impl QueueType for Compute
{
    #[inline]
    fn supported_by(queue_type: QueueTypeEnum) -> bool
    {
        match queue_type
        {
            QueueTypeEnum::Compute => true,
            QueueTypeEnum::General => true,
            _ => false,
        }
    }
}

impl QueueType for Graphics
{
    #[inline]
    fn supported_by(queue_type: QueueTypeEnum) -> bool
    {
        match queue_type
        {
            QueueTypeEnum::General => true,
            QueueTypeEnum::Graphics => true,
            _ => false,
        }
    }
}

impl QueueType for Transfer
{
    #[inline]
    fn supported_by(queue_type: QueueTypeEnum) -> bool
    {
        match queue_type
        {
            QueueTypeEnum::Compute => true,
            QueueTypeEnum::General => true,
            QueueTypeEnum::Graphics => true,
            QueueTypeEnum::Transfer => true,
        }
    }
}

impl QueueType for General
{
    #[inline]
    fn supported_by(queue_type: QueueTypeEnum) -> bool
    {
        match queue_type
        {
            QueueTypeEnum::General => true,
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
impl Supports<GraphicsOrCompute> for Compute {}
impl Supports<GraphicsOrCompute> for Graphics {}
impl Supports<GraphicsOrCompute> for General {}

pub struct Primary;
pub struct Secondary;

pub trait Level
{
}

impl Level for Primary {}
impl Level for Secondary {}
