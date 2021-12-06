// Copyright 2021 Hwakyeom Kim(=just-do-halee)

#[macro_export]
macro_rules! impl_ {
    (
        Chopable<$lt:lifetime> for $name:ty
    ) => {
        impl<$lt> Chopable<$lt> for $name {
            type Out = SourceChunk<$lt>;
            /// `out of bounds == None`
            #[inline]
            fn chop<A: Into<Span>>(&$lt self, span: A) -> Option<Self::Out> {
                SourceChunk::new(self.as_ref(), span)
            }
        }
    };
}
