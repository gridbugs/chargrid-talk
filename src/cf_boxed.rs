use crate::cf::{self, CF};
pub use crate::cf::{Escape, LoopControl, OrEscape};
use chargrid_core::prelude::*;
use chargrid_core::BoxedComponent;

pub struct BoxedCF<O, S>(CF<BoxedComponent<O, S>>);

impl<O, S> Component for BoxedCF<O, S> {
    type Output = O;
    type State = S;
    fn render(&self, state: &Self::State, ctx: Ctx, fb: &mut FrameBuffer) {
        self.0.render(state, ctx, fb);
    }
    fn update(&mut self, state: &mut Self::State, ctx: Ctx, event: Event) -> Self::Output {
        self.0.update(state, ctx, event)
    }
    fn size(&self, state: &Self::State, ctx: Ctx) -> Size {
        self.0.size(state, ctx)
    }
}

pub fn boxed_cf<C: 'static + Component>(component: C) -> BoxedCF<C::Output, C::State>
where
    C::State: Sized,
{
    BoxedCF(cf::cf(BoxedComponent(Box::new(component))))
}

pub fn val<S: 'static, T: 'static + Clone>(t: T) -> BoxedCF<Option<T>, S> {
    boxed_cf(cf::val(t))
}

pub fn loop_unit<Br, C: 'static, F: 'static>(f: F) -> BoxedCF<Option<Br>, C::State>
where
    C::State: Sized,
    C: Component<Output = Option<LoopControl<(), Br>>>,
    F: FnMut() -> C,
{
    boxed_cf(cf::loop_unit(f))
}

pub fn on_state<S: 'static, T: 'static, F: 'static>(f: F) -> BoxedCF<Option<T>, S>
where
    F: FnOnce(&mut S) -> T,
{
    boxed_cf(cf::on_state(f))
}

impl<T: 'static, S: 'static> BoxedCF<Option<T>, S> {
    pub fn catch_escape(self) -> BoxedCF<Option<OrEscape<T>>, S> {
        boxed_cf(self.0.catch_escape())
    }

    pub fn map<U, F: 'static>(self, f: F) -> BoxedCF<Option<U>, S>
    where
        F: FnOnce(T) -> U,
    {
        boxed_cf(self.0.map(f))
    }

    pub fn and_then<U, D: 'static, F: 'static>(self, f: F) -> BoxedCF<Option<U>, S>
    where
        D: Component<Output = Option<U>, State = S>,
        F: FnOnce(T) -> D,
    {
        boxed_cf(self.0.and_then(f))
    }
}

impl<S: 'static> BoxedCF<app::Output, S> {
    pub fn exit_on_close(self) -> Self {
        boxed_cf(self.0.exit_on_close())
    }
}

impl<O: 'static> BoxedCF<O, ()> {
    pub fn ignore_state<S: 'static>(self) -> BoxedCF<O, S> {
        boxed_cf(self.0.ignore_state())
    }
}

impl<O: 'static, S: 'static> BoxedCF<O, S> {
    pub fn with_state(self, state: S) -> BoxedCF<O, ()> {
        boxed_cf(self.0.with_state(state))
    }

    pub fn clear_each_frame(self) -> Self {
        boxed_cf(self.0.clear_each_frame())
    }
}
