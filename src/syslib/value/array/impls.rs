use super::*;

/*
    Getter/Setter
*/

macro_rules! impl_array {
    (
        @all
        $type_name: ty, $type: ty, $type_check: expr
    ) => {
        impl_array!(
            @getter
            $type_name, $type, $type_check
        );
        impl_array!(
            @setter
            $type_name, $type, $type_check
        );
    };
    (
        @getter
        $type_name: ty, $type: ty, $type_check: expr
    ) => {
        ::paste::paste! {
            #[doc = "获取`" $type_name "`类型元素值"]
            ///
            /// # Panics
            ///
            /// 索引越界或类型不匹配时会触发Panic
            pub fn [<get_item_ $type_name>](&self, dim: impl AsArrayIndex) -> Option<$type> {
                self.[<try_get_item_ $type_name>](dim).unwrap()
            }

            #[doc = "获取`" $type_name "`类型元素值"]
            pub fn [<try_get_item_ $type_name>](&self, dim: impl AsArrayIndex) -> Result<Option<$type>> {
                let dim = dim.as_array_index();
                self.check_get(dim, $type_check)?;
                unsafe {
                    Ok(self.[<get_item_ $type_name _unchecked>](dim))
                }
            }

            #[doc = "获取`" $type_name "`类型元素值,不检查类型"]
            ///
            /// # Safety
            ///
            /// 索引越界或类型不兼容时可能会出现未定义行为
            pub unsafe fn [<get_item_ $type_name _unchecked>](&self, dim: impl AsArrayIndex) -> Option<$type> {
                let item = self.get_item_value_unchecked(dim);
                item.[<get_ $type_name _unchecked>]()
            }
        }
    };
    (
        @setter
        $type_name: ty, $type: ty, $type_check: expr
    ) => {
        ::paste::paste! {
            #[doc = "设置`" $type_name "`类型元素值"]
            ///
            /// # Panics
            ///
            /// 索引越界或类型不匹配时会触发Panic
            pub fn [<set_item_ $type_name>](&mut self, dim: impl AsArrayIndex, value: $type) {
                self.[<try_set_item_ $type_name>](dim, value).unwrap()
            }

            #[doc = "设置`" $type_name "`类型元素值"]
            ///
            /// # Panics
            ///
            /// 索引越界或类型不匹配时会触发Panic
            pub fn [<try_set_item_ $type_name>](&mut self, dim: impl AsArrayIndex, value: $type) -> Result<()> {
                let dim = dim.as_array_index();
                self.check_set(dim, $type_check)?;
                unsafe {
                    self.[<set_item_ $type_name _unchecked>](dim, value);
                }
                Ok(())
            }

            #[doc = "设置`" $type_name "`类型元素值,不检查类型"]
            ///
            /// # Safety
            ///
            /// 索引越界或类型不兼容时可能会出现未定义行为
            pub unsafe fn [<set_item_ $type_name _unchecked>](&mut self, dim: impl AsArrayIndex, value: $type) {
                let mut item = self.get_item_value_unchecked(dim);
                item.[<set_ $type_name _unchecked>](value);
            }
        }
    };
}

impl<'arr> Array<'arr> {
    impl_array!(
        @all
        int, pbint, ValueType::Int
    );
    impl_array!(
        @all
        uint, pbuint, ValueType::Uint
    );
    impl_array!(
        @all
        long, pblong, ValueType::Long
    );
    impl_array!(
        @all
        ulong, pbulong, ValueType::Ulong
    );
    impl_array!(
        @all
        longlong, pblonglong, ValueType::LongLong
    );
    impl_array!(
        @all
        real, pbreal, ValueType::Real
    );
    impl_array!(
        @all
        double, pbdouble, ValueType::Double
    );
    impl_array!(
        @all
        byte, pbbyte, ValueType::Byte
    );
    impl_array!(
        @all
        bool, bool, ValueType::Boolean
    );
    impl_array!(
        @all
        char, PBChar, ValueType::Char
    );

    impl_array!(
        @all
        dec, Decimal, ValueType::Decimal
    );
    impl_array!(
        @all
        date, NaiveDate, ValueType::Date
    );
    impl_array!(
        @all
        time, NaiveTime, ValueType::Time
    );
    impl_array!(
        @all
        datetime, NaiveDateTime, ValueType::DateTime
    );
    impl_array!(
        @getter
        blob, &'arr [u8], ValueType::Blob
    );
    impl_array!(
        @setter
        blob, &[u8], ValueType::Blob
    );
    impl_array!(
        @getter
        str, &'arr PBStr, ValueType::String
    );
    impl_array!(
        @getter
        string, PBString, ValueType::String
    );
    impl_array!(
        @setter
        str, impl AsPBStr, ValueType::String
    );
}
