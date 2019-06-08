use std::cell::RefCell;
use std::ffi::c_void;
use std::mem;
use std::rc::Rc;

use crate::convert::Error;
use crate::def::ClassLike;
use crate::interpreter::Mrb;
use crate::sys;
use crate::value::types::{Ruby, Rust};
use crate::value::Value;

/// Provides converters to and from [`Value`] with ruby type of [`Ruby::Data`].
///
/// This trait provides default implementations of `try_into_ruby` and
/// `try_from_ruby`.
///
/// The 'static type bound comes from the class spec registry on
/// [`State`](crate::state::State).
///
/// **Warning**: `Self` must be allocated on the heap. If `Self` is not heap
/// allocated, malloc will fail when attempting to free the generated Ruby
/// objects.
pub trait RustBackedValue
where
    Self: 'static + Sized,
{
    /// Try to convert a Rust object into a [`Value`].
    ///
    /// Inject the data pointer into `slf` if it is provided, otherwise call
    /// [`sys::mrb_obj_new`] to get a new instance of the class associated with
    /// `Self`.
    ///
    /// To store `self` in a [`sys::mrb_value`], this function wraps `self` in
    /// an `Rc<RefCell<_>>`.
    unsafe fn try_into_ruby(
        self,
        interp: &Mrb,
        slf: Option<sys::mrb_value>,
    ) -> Result<Value, Error<Rust, Ruby>> {
        let spec = interp.borrow().class_spec::<Self>().ok_or(Error {
            from: Rust::Object,
            to: Ruby::Object,
        })?;
        let rclass = spec.borrow().rclass(interp).ok_or(Error {
            from: Rust::Object,
            to: Ruby::Object,
        })?;
        let mut slf = slf.unwrap_or_else(|| {
            sys::mrb_obj_new(interp.borrow().mrb, rclass, 0, &[] as *const sys::mrb_value)
        });
        let data = Rc::new(RefCell::new(self));

        let ptr = mem::transmute::<Rc<RefCell<Self>>, *mut c_void>(data);
        sys::mrb_sys_data_init(&mut slf, ptr, spec.borrow().data_type());
        Ok(Value::new(interp, slf))
    }

    /// Extract the Rust object from the [`Value`] if the [`Value`] is backed by
    /// `Self`.
    ///
    /// Extract the data pointer from `slf` and return an `Rc<RefCell<_>>`
    /// containing the Rust object.
    ///
    /// This function sanity checks to make sure that [`Value`] is a
    /// [`Ruby::Data`] and that the `RClass *` of the spec matches the
    /// [`Value`].
    unsafe fn try_from_ruby(
        interp: &Mrb,
        slf: &Value,
    ) -> Result<Rc<RefCell<Self>>, Error<Ruby, Rust>> {
        // Make sure we have a Data otherwise extraction will fail.
        if slf.ruby_type() != Ruby::Data {
            return Err(Error {
                from: slf.ruby_type(),
                to: Rust::Object,
            });
        }
        let spec = interp.borrow().class_spec::<Self>().ok_or(Error {
            from: Ruby::Object,
            to: Rust::Object,
        })?;
        // Sanity check that the RClass matches.
        if let Some(rclass) = spec.borrow().rclass(interp) {
            if !std::ptr::eq(
                sys::mrb_sys_class_of_value(interp.borrow().mrb, slf.inner()),
                rclass,
            ) {
                return Err(Error {
                    from: slf.ruby_type(),
                    to: Rust::Object,
                });
            }
        } else {
            return Err(Error {
                from: slf.ruby_type(),
                to: Rust::Object,
            });
        }
        let ptr = {
            let borrow = spec.borrow();
            sys::mrb_data_get_ptr(interp.borrow().mrb, slf.inner(), borrow.data_type())
        };
        let data = mem::transmute::<*mut c_void, Rc<RefCell<Self>>>(ptr);
        let value = Rc::clone(&data);
        mem::forget(data);
        Ok(value)
    }
}

impl<T> RustBackedValue for Box<T> where T: RustBackedValue {}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::ffi::c_void;
    use std::mem;
    use std::rc::Rc;

    use crate::convert::object::RustBackedValue;
    use crate::convert::FromMrb;
    use crate::def::{rust_data_free, ClassLike, Define};
    use crate::interpreter::Interpreter;
    use crate::sys;
    use crate::value::{Value, ValueLike};

    struct Container {
        inner: String,
    }

    impl Container {
        unsafe extern "C" fn value(
            mrb: *mut sys::mrb_state,
            slf: sys::mrb_value,
        ) -> sys::mrb_value {
            let interp = interpreter_or_raise!(mrb);
            let spec = class_spec_or_raise!(interp, Self);

            let borrow = spec.borrow();
            let ptr = sys::mrb_data_get_ptr(mrb, slf, borrow.data_type());
            let data = mem::transmute::<*mut c_void, Rc<RefCell<Self>>>(ptr);
            let container = Rc::clone(&data);
            mem::forget(data);
            let borrow = container.borrow();
            Value::from_mrb(&interp, borrow.inner.as_str()).inner()
        }
    }

    impl RustBackedValue for Container {}

    #[derive(Default)]
    // this struct is stack allocated
    struct Other {
        _inner: bool,
    }

    impl RustBackedValue for Other {}

    #[test]
    fn convert_obj_roundtrip() {
        let interp = Interpreter::create().expect("mrb init");
        let spec = interp.borrow_mut().def_class::<Container>(
            "Container",
            None,
            Some(rust_data_free::<Container>),
        );
        spec.borrow_mut().mrb_value_is_rust_backed(true);
        spec.borrow_mut()
            .add_method("value", Container::value, sys::mrb_args_none());
        spec.borrow().define(&interp).expect("class install");
        let obj = Container {
            inner: "contained string contents".to_owned(),
        };

        let value = unsafe { obj.try_into_ruby(&interp, None) }.expect("convert");
        let class = value.funcall::<Value, _, _>("class", &[]).expect("funcall");
        assert_eq!(class.to_s(), "Container");
        let data = unsafe { Container::try_from_ruby(&interp, &value) }.expect("convert");
        assert_eq!(Rc::strong_count(&data), 2);
        assert_eq!(&data.borrow().inner, "contained string contents");
        drop(data);
        let inner = value
            .funcall::<String, _, _>("value", &[])
            .expect("funcall");
        assert_eq!(&inner, "contained string contents");
    }

    #[test]
    fn convert_obj_not_data() {
        let interp = Interpreter::create().expect("mrb init");
        let spec = interp.borrow_mut().def_class::<Container>(
            "Container",
            None,
            Some(rust_data_free::<Container>),
        );
        spec.borrow_mut().mrb_value_is_rust_backed(true);
        spec.borrow_mut()
            .add_method("value", Container::value, sys::mrb_args_none());
        spec.borrow().define(&interp).expect("class install");
        let spec = interp.borrow_mut().def_class::<Box<Other>>(
            "Other",
            None,
            Some(rust_data_free::<Container>),
        );
        spec.borrow_mut().mrb_value_is_rust_backed(true);
        spec.borrow().define(&interp).expect("class install");

        let value = Value::from_mrb(&interp, "string");
        let class = value.funcall::<Value, _, _>("class", &[]).expect("funcall");
        assert_eq!(class.to_s(), "String");
        let data = unsafe { Container::try_from_ruby(&interp, &value) };
        assert!(data.is_err());
        let value =
            unsafe { Box::new(Other::default()).try_into_ruby(&interp, None) }.expect("convert");
        let class = value.funcall::<Value, _, _>("class", &[]).expect("funcall");
        assert_eq!(class.to_s(), "Other");
        let data = unsafe { Container::try_from_ruby(&interp, &value) };
        assert!(data.is_err());
    }
}