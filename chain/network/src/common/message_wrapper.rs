use actix::dev::MessageResponse;
use actix::Message;
use near_rate_limiter::{ThrottleController, ThrottleToken};
use std::marker::PhantomData;

// Wrapper around Actix messages, used to track size of all messages sent to PeerManager.
// TODO(#5155) Finish implementation of this.

#[allow(unused)]
/// TODO - Once we start using this `ActixMessageWrapper` we will need to make following changes
/// to get this struct to work
/// - Add needed decorators. Probably `Debug`, `Message` from Actix, etc.
/// - Add two rate limiters (local per peer, global one)
/// - Any other metadata we need debugging, etc.
pub struct ActixMessageWrapper<T, Q> {
    msg: T,
    throttle_token: ThrottleToken,
    ph: PhantomData<Q>,
}

impl<T, Q> ActixMessageWrapper<T, Q> {
    pub fn new_without_size(msg: T, throttle_controller: ThrottleController) -> Self {
        Self { msg, throttle_token: ThrottleToken::new(throttle_controller, 0), ph: PhantomData }
    }

    #[allow(unused)]
    pub fn into_inner(mut self) -> (T) {
        return self.msg;
    }

    #[allow(unused)]
    pub fn take(mut self) -> (T, ThrottleToken) {
        return (self.msg, self.throttle_token);
    }
}

impl<T, Q> Message for ActixMessageWrapper<T, Q>
where
    Q: 'static,
{
    type Result = ActixMessageResponse<Q>;
}

#[derive(MessageResponse)]
pub struct ActixMessageResponse<T> {
    msg: T,
    /// Ignore the warning, this code is used. We decrease counters `throttle_controller` when
    /// this attribute gets dropped.
    #[allow(unused)]
    throttle_token: ThrottleToken,
}

impl<T> ActixMessageResponse<T> {
    #[allow(unused)]
    pub fn new(msg: T, throttle_token: ThrottleToken) -> Self {
        Self { msg, throttle_token }
    }

    #[allow(unused)]
    pub fn into_inner(mut self) -> (T) {
        return self.msg;
    }

    #[allow(unused)]
    pub fn take(mut self) -> (T, ThrottleToken) {
        return (self.msg, self.throttle_token);
    }
}
