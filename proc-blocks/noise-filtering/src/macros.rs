macro_rules! builder_methods {
    ($( $property:ident : $type:ty ),* $(,)?) => {
        $(
            paste::paste! {
                pub fn [< set_ $property >](&mut self, $property: $type) -> &mut Self {
                    self.$property = $property;
                    self
                }
            }
        )*

        $(
            paste::paste! {
                pub const fn $property(&self) -> $type {
                    self.$property
                }
            }
        )*
    };
}

macro_rules! defered_builder_methods {
    ($( $component:ident . $property:ident : $type:ty; )*) => {
        $(
            paste::paste! {
                pub fn [< set_ $property >](&mut self, $property: $type) -> &mut Self {
                    self.$component.[< set_ $property >]($property);
                    self
                }
            }
        )*

        $(
            paste::paste! {
                pub fn $property(&self) -> $type {
                    self.$component.$property()
                }
            }
        )*
    };
}
