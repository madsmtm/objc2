macro_rules! associated_object {
    (
        $(#[$m:meta])*
        impl $name:ident {
            $v_getter:vis fn $getter_name:ident(&self) -> $getter_ty:ty;
            $v_setter:vis fn $setter_name:ident(&self, $setter_param:ident: $setter_ty:ty);
        }
    ) => {
        const _: () = {
            static mut __KEY: u8 = 0;

            $(#[$m])*
            impl $name {
                $v_getter fn $getter_name(&self) -> $getter_ty {

                }

                $v_setter fn $setter_name(&self, $setter_param: $setter_ty) {

                }
            }
        }
    };
}
