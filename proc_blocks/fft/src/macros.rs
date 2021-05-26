macro_rules! builder_methods {
    ($( $property:ident : $type:ty ),* $(,)?) => {
        $(
            #[allow(dead_code)]
            paste::paste! {
                pub fn [< set_ $property >](&mut self, $property: $type) -> &mut Self {
                    self.$property = $property;
                    self
                }
            }
        )*

        $(
            #[allow(dead_code)]
            paste::paste! {
                pub fn $property(&self) -> $type {
                    self.$property
                }
            }
        )*
    };
}
