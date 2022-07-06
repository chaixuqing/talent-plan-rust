use failure::Fail;

#[derive(Fail,Debug)]
#[fail(display="Error for KvStore")]
pub enum KvError{
    #[fail(display="this method is unimplemented")]
    Unimplemented,
}

pub type Result<T> = std::result::Result<T,KvError>;