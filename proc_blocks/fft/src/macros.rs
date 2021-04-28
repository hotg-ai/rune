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
            paste::paste! {
                pub fn [< set_ $property >](&mut self, $property: $type) {
                    self.$property = $property;
                }
            }
        )*

        $(
            paste::paste! {
                pub fn $property(&self) -> $type {
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
                pub fn [< with_ $property >](mut self, $property: $type) -> Self {
                    self.[< set_ $property >]($property);
                    self
                }
            }
        )*

        $(
            paste::paste! {
                pub fn [< set_ $property >](&mut self, $property: $type) {
                    self.$component.[< set_ $property >]($property);
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
