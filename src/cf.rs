use chargrid_core::prelude::*;
use std::marker::PhantomData;

pub struct CF<C: Component>(C);

pub fn cf<C: Component>(component: C) -> CF<C> {
    CF(component)
}

impl<C: Component> Component for CF<C> {
    type Output = C::Output;
    type State = C::State;
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

impl<T, C: Component<Output = Option<T>>> CF<C> {
    pub fn catch_escape(self) -> CF<CatchEscape<C>> {
        cf(CatchEscape(self.0))
    }

    pub fn map<U, F>(self, f: F) -> CF<Map<C, F>>
    where
        F: FnOnce(T) -> U,
    {
        cf(Map {
            component: self.0,
            f: Some(f),
        })
    }

    pub fn and_then<U, D, F>(self, f: F) -> CF<AndThen<C, D, F>>
    where
        D: Component<Output = Option<U>, State = C::State>,
        F: FnOnce(T) -> D,
    {
        cf(AndThen::First {
            component: self.0,
            f: Some(f),
        })
    }
}

impl<C: Component<Output = app::Output>> CF<C> {
    pub fn exit_on_close(self) -> CF<ExitOnClose<C>> {
        cf(ExitOnClose(self.0))
    }
}

impl<C: Component> CF<C>
where
    C::State: Sized,
{
    pub fn with_state(self, state: C::State) -> CF<WithState<C>> {
        cf(WithState {
            component: self.0,
            state,
        })
    }
}

impl<C: Component> CF<C> {
    pub fn clear_each_frame(self) -> CF<ClearEachFrame<C>> {
        cf(ClearEachFrame(self.0))
    }
}

impl<C: Component<State = ()>> CF<C> {
    pub fn ignore_state<S>(self) -> CF<IgnoreState<S, C>> {
        cf(IgnoreState {
            state: PhantomData,
            component: self.0,
        })
    }
}

pub struct Escape;
pub type OrEscape<T> = Result<T, Escape>;

pub struct CatchEscape<C: Component>(pub C);

impl<T, C> Component for CatchEscape<C>
where
    C: Component<Output = Option<T>>,
{
    type Output = Option<OrEscape<T>>;
    type State = C::State;
    fn render(&self, state: &Self::State, ctx: Ctx, fb: &mut FrameBuffer) {
        self.0.render(state, ctx, fb);
    }
    fn update(&mut self, state: &mut Self::State, ctx: Ctx, event: Event) -> Self::Output {
        if let Event::Input(input::Input::Keyboard(input::keys::ESCAPE)) = event {
            return Some(Err(Escape));
        }
        self.0.update(state, ctx, event).map(Ok)
    }
    fn size(&self, state: &Self::State, ctx: Ctx) -> Size {
        self.0.size(state, ctx)
    }
}

#[derive(Clone, Copy)]
pub enum LoopControl<Co, Br> {
    Continue(Co),
    Break(Br),
}

pub struct Loop<C, F> {
    component: C,
    f: F,
}
pub fn loop_<Co, Br, C, F>(init: Co, mut f: F) -> CF<Loop<C, F>>
where
    C: Component<Output = Option<LoopControl<Co, Br>>>,
    F: FnMut(Co) -> C,
{
    cf(Loop {
        component: f(init),
        f,
    })
}

impl<Co, Br, C, F> Component for Loop<C, F>
where
    C: Component<Output = Option<LoopControl<Co, Br>>>,
    F: FnMut(Co) -> C,
{
    type Output = Option<Br>;
    type State = C::State;
    fn render(&self, state: &Self::State, ctx: Ctx, fb: &mut FrameBuffer) {
        self.component.render(state, ctx, fb);
    }
    fn update(&mut self, state: &mut Self::State, ctx: Ctx, event: Event) -> Self::Output {
        if let Some(control) = self.component.update(state, ctx, event) {
            match control {
                LoopControl::Continue(co) => {
                    self.component = (self.f)(co);
                    while let Some(control) = self.component.update(state, ctx, Event::Peek) {
                        match control {
                            LoopControl::Continue(co) => {
                                self.component = (self.f)(co);
                            }
                            LoopControl::Break(br) => {
                                return Some(br);
                            }
                        }
                    }
                    None
                }
                LoopControl::Break(br) => Some(br),
            }
        } else {
            None
        }
    }
    fn size(&self, state: &Self::State, ctx: Ctx) -> Size {
        self.component.size(state, ctx)
    }
}

pub struct WithState<C: Component> {
    component: C,
    state: C::State,
}
impl<C> Component for WithState<C>
where
    C: Component,
{
    type Output = C::Output;
    type State = ();
    fn render(&self, _state: &Self::State, ctx: Ctx, fb: &mut FrameBuffer) {
        self.component.render(&self.state, ctx, fb);
    }
    fn update(&mut self, _state: &mut Self::State, ctx: Ctx, event: Event) -> Self::Output {
        self.component.update(&mut self.state, ctx, event)
    }
    fn size(&self, _state: &Self::State, ctx: Ctx) -> Size {
        self.component.size(&self.state, ctx)
    }
}

pub struct ClearEachFrame<C: Component>(pub C);

impl<C> Component for ClearEachFrame<C>
where
    C: Component,
{
    type Output = C::Output;
    type State = C::State;
    fn render(&self, state: &Self::State, ctx: Ctx, fb: &mut FrameBuffer) {
        fb.clear();
        self.0.render(state, ctx, fb);
    }
    fn update(&mut self, state: &mut Self::State, ctx: Ctx, event: Event) -> Self::Output {
        self.0.update(state, ctx, event)
    }
    fn size(&self, state: &Self::State, ctx: Ctx) -> Size {
        self.0.size(state, ctx)
    }
}

pub struct LoopUnit<C, F> {
    component: C,
    f: F,
}
pub fn loop_unit<Br, C, F>(mut f: F) -> CF<LoopUnit<C, F>>
where
    C: Component<Output = Option<LoopControl<(), Br>>>,
    F: FnMut() -> C,
{
    cf(LoopUnit { component: f(), f })
}
impl<Br, C, F> Component for LoopUnit<C, F>
where
    C: Component<Output = Option<LoopControl<(), Br>>>,
    F: FnMut() -> C,
{
    type Output = Option<Br>;
    type State = C::State;
    fn render(&self, state: &Self::State, ctx: Ctx, fb: &mut FrameBuffer) {
        self.component.render(state, ctx, fb);
    }
    fn update(&mut self, state: &mut Self::State, ctx: Ctx, event: Event) -> Self::Output {
        if let Some(control) = self.component.update(state, ctx, event) {
            match control {
                LoopControl::Continue(()) => {
                    self.component = (self.f)();
                    while let Some(control) = self.component.update(state, ctx, Event::Peek) {
                        match control {
                            LoopControl::Continue(()) => {
                                self.component = (self.f)();
                            }
                            LoopControl::Break(br) => {
                                return Some(br);
                            }
                        }
                    }
                    None
                }
                LoopControl::Break(br) => Some(br),
            }
        } else {
            None
        }
    }
    fn size(&self, state: &Self::State, ctx: Ctx) -> Size {
        self.component.size(state, ctx)
    }
}

pub struct Val<T: Clone>(pub T);
pub fn val<S, T: Clone>(t: T) -> CF<IgnoreState<S, Val<T>>> {
    cf(Val(t)).ignore_state()
}
impl<T: Clone> Component for Val<T> {
    type Output = Option<T>;
    type State = ();
    fn render(&self, _state: &Self::State, _ctx: Ctx, _fb: &mut FrameBuffer) {}
    fn update(&mut self, _state: &mut Self::State, _ctx: Ctx, _event: Event) -> Self::Output {
        Some(self.0.clone())
    }
    fn size(&self, _state: &Self::State, _ctx: Ctx) -> Size {
        Size::new_u16(0, 0)
    }
}

pub struct IgnoreState<S, C: Component<State = ()>> {
    state: PhantomData<S>,
    component: C,
}

impl<S, C> Component for IgnoreState<S, C>
where
    C: Component<State = ()>,
{
    type Output = C::Output;
    type State = S;

    fn render(&self, _state: &Self::State, ctx: Ctx, fb: &mut FrameBuffer) {
        self.component.render(&(), ctx, fb);
    }
    fn update(&mut self, _state: &mut Self::State, ctx: Ctx, event: Event) -> Self::Output {
        self.component.update(&mut (), ctx, event)
    }
    fn size(&self, _state: &Self::State, ctx: Ctx) -> Size {
        self.component.size(&(), ctx)
    }
}

pub struct ExitOnClose<C: Component<Output = app::Output>>(pub C);

impl<C> Component for ExitOnClose<C>
where
    C: Component<Output = app::Output>,
{
    type Output = app::Output;
    type State = C::State;
    fn render(&self, state: &Self::State, ctx: Ctx, fb: &mut FrameBuffer) {
        self.0.render(state, ctx, fb);
    }
    fn update(&mut self, state: &mut Self::State, ctx: Ctx, event: Event) -> Self::Output {
        if let Event::Input(input::Input::Keyboard(input::keys::ETX)) = event {
            return Some(app::Exit);
        }
        self.0.update(state, ctx, event)
    }
    fn size(&self, state: &Self::State, ctx: Ctx) -> Size {
        self.0.size(state, ctx)
    }
}

pub enum Either2<A, B> {
    A(A),
    B(B),
}

impl<A, B> Component for Either2<A, B>
where
    A: Component,
    B: Component<State = A::State, Output = A::Output>,
{
    type Output = A::Output;
    type State = A::State;
    fn render(&self, state: &Self::State, ctx: Ctx, fb: &mut FrameBuffer) {
        match self {
            Self::A(a) => a.render(state, ctx, fb),
            Self::B(b) => b.render(state, ctx, fb),
        }
    }
    fn update(&mut self, state: &mut Self::State, ctx: Ctx, event: Event) -> Self::Output {
        match self {
            Self::A(a) => a.update(state, ctx, event),
            Self::B(b) => b.update(state, ctx, event),
        }
    }
    fn size(&self, state: &Self::State, ctx: Ctx) -> Size {
        match self {
            Self::A(a) => a.size(state, ctx),
            Self::B(b) => b.size(state, ctx),
        }
    }
}

pub enum Either3<A, B, C> {
    A(A),
    B(B),
    C(C),
}

impl<A, B, C> Component for Either3<A, B, C>
where
    A: Component,
    B: Component<State = A::State, Output = A::Output>,
    C: Component<State = A::State, Output = A::Output>,
{
    type Output = A::Output;
    type State = A::State;
    fn render(&self, state: &Self::State, ctx: Ctx, fb: &mut FrameBuffer) {
        match self {
            Self::A(a) => a.render(state, ctx, fb),
            Self::B(b) => b.render(state, ctx, fb),
            Self::C(c) => c.render(state, ctx, fb),
        }
    }
    fn update(&mut self, state: &mut Self::State, ctx: Ctx, event: Event) -> Self::Output {
        match self {
            Self::A(a) => a.update(state, ctx, event),
            Self::B(b) => b.update(state, ctx, event),
            Self::C(c) => c.update(state, ctx, event),
        }
    }
    fn size(&self, state: &Self::State, ctx: Ctx) -> Size {
        match self {
            Self::A(a) => a.size(state, ctx),
            Self::B(b) => b.size(state, ctx),
            Self::C(c) => c.size(state, ctx),
        }
    }
}

pub enum Either4<A, B, C, D> {
    A(A),
    B(B),
    C(C),
    D(D),
}

impl<A, B, C, D> Component for Either4<A, B, C, D>
where
    A: Component,
    B: Component<State = A::State, Output = A::Output>,
    C: Component<State = A::State, Output = A::Output>,
    D: Component<State = A::State, Output = A::Output>,
{
    type Output = A::Output;
    type State = A::State;
    fn render(&self, state: &Self::State, ctx: Ctx, fb: &mut FrameBuffer) {
        match self {
            Self::A(a) => a.render(state, ctx, fb),
            Self::B(b) => b.render(state, ctx, fb),
            Self::C(c) => c.render(state, ctx, fb),
            Self::D(d) => d.render(state, ctx, fb),
        }
    }
    fn update(&mut self, state: &mut Self::State, ctx: Ctx, event: Event) -> Self::Output {
        match self {
            Self::A(a) => a.update(state, ctx, event),
            Self::B(b) => b.update(state, ctx, event),
            Self::C(c) => c.update(state, ctx, event),
            Self::D(d) => d.update(state, ctx, event),
        }
    }
    fn size(&self, state: &Self::State, ctx: Ctx) -> Size {
        match self {
            Self::A(a) => a.size(state, ctx),
            Self::B(b) => b.size(state, ctx),
            Self::C(c) => c.size(state, ctx),
            Self::D(d) => d.size(state, ctx),
        }
    }
}

pub enum AndThen<C, D, F> {
    // f is an option because when it is called, the compiler doesn't know that we're about to
    // destroy it
    First { component: C, f: Option<F> },
    Second(D),
}

impl<T, U, C, D, F> Component for AndThen<C, D, F>
where
    C: Component<Output = Option<T>>,
    D: Component<Output = Option<U>, State = C::State>,
    F: FnOnce(T) -> D,
{
    type Output = Option<U>;
    type State = C::State;
    fn render(&self, state: &Self::State, ctx: Ctx, fb: &mut FrameBuffer) {
        match self {
            Self::First { component, .. } => component.render(state, ctx, fb),
            Self::Second(component) => component.render(state, ctx, fb),
        }
    }
    fn update(&mut self, state: &mut Self::State, ctx: Ctx, event: Event) -> Self::Output {
        match self {
            Self::First { component, f } => match component.update(state, ctx, event) {
                None => None,
                Some(t) => {
                    let mut d = (f.take().unwrap())(t);
                    let peek_result = d.update(state, ctx, Event::Peek);
                    *self = Self::Second(d);
                    peek_result
                }
            },
            Self::Second(component) => component.update(state, ctx, event),
        }
    }
    fn size(&self, state: &Self::State, ctx: Ctx) -> Size {
        match self {
            Self::First { component, .. } => component.size(state, ctx),
            Self::Second(component) => component.size(state, ctx),
        }
    }
}

pub struct OnState<S, T, F> {
    state: PhantomData<S>,
    output: PhantomData<T>,
    f: Option<F>,
}
pub fn on_state<S, T, F>(f: F) -> CF<OnState<S, T, F>>
where
    F: FnOnce(&mut S) -> T,
{
    cf(OnState {
        state: PhantomData,
        output: PhantomData,
        f: Some(f),
    })
}
impl<S, T, F> Component for OnState<S, T, F>
where
    F: FnOnce(&mut S) -> T,
{
    type Output = Option<T>;
    type State = S;

    fn render(&self, _state: &Self::State, _ctx: Ctx, _fb: &mut FrameBuffer) {
        panic!("this component should not live long enough to be rendered");
    }
    fn update(&mut self, state: &mut Self::State, _ctx: Ctx, _event: Event) -> Self::Output {
        Some((self
            .f
            .take()
            .expect("this component should only be updated once"))(
            state
        ))
    }
    fn size(&self, _state: &Self::State, _ctx: Ctx) -> Size {
        panic!("nothing should be checking the size of this component")
    }
}

pub struct Map<C, F> {
    component: C,
    f: Option<F>,
}
impl<T, U, C, F> Component for Map<C, F>
where
    C: Component<Output = Option<T>>,
    F: FnOnce(T) -> U,
{
    type Output = Option<U>;
    type State = C::State;
    fn render(&self, state: &Self::State, ctx: Ctx, fb: &mut FrameBuffer) {
        self.component.render(state, ctx, fb);
    }
    fn update(&mut self, state: &mut Self::State, ctx: Ctx, event: Event) -> Self::Output {
        match self.component.update(state, ctx, event) {
            None => None,
            Some(t) => Some((self.f.take().expect("component yielded multiple times"))(
                t,
            )),
        }
    }
    fn size(&self, state: &Self::State, ctx: Ctx) -> Size {
        self.component.size(state, ctx)
    }
}
