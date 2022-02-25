use crate::Vector2;

// A Cast trait
pub trait Cast<Output>: Copy {
    fn cast(self) -> Output;
}

impl<Input: Copy + Into<Output>, Output> Cast<Output> for Input {
    fn cast(self) -> Output {
        self.into()
    }
}

// Cast between similar types
pub(crate) fn cast<Input: Copy + Cast<Output>, Output>(me: Vector2<Input>) -> Vector2<Output> {
    Vector2::<Output> {
        inner: [me.inner[0].cast(), me.inner[1].cast()],
    }
}