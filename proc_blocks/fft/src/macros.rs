macro_rules! builder_methods {
    ($( $property:ident : $type:ty ),* $(,)?) => {
        $(
            paste::paste! {
                #[allow(dead_code)]
                pub fn [< with_ $property >](mut self, $property: $type) -> Self {
                    self.[< set_ $property >]($property);
                    self
                }
            }
        )*

        $(
            #[allow(dead_code)]
            paste::paste! {
                pub fn [< set_ $property >](&mut self, $property: $type) {
                    self.$property = $property;
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
