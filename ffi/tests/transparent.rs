#![allow(unsafe_code, clippy::restriction, clippy::pedantic)]

use std::{alloc, marker::PhantomData, mem::MaybeUninit};

use iroha_ffi::{
    ffi_export,
    slice::{OutBoxedSlice, SliceRef},
    FfiConvert, FfiOutPtrRead, FfiReturn, FfiType,
};

iroha_ffi::def_ffi_fn! { dealloc }

#[derive(Clone, Copy, PartialEq, Eq, Debug, FfiType)]
#[ffi_type(unsafe {robust})]
#[repr(transparent)]
pub struct GenericTransparentStruct<P>(u64, PhantomData<P>);

impl<P> GenericTransparentStruct<P> {
    fn new(value: u64) -> Self {
        Self(value, PhantomData)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, FfiType)]
#[ffi_type(unsafe {robust})]
#[repr(transparent)]
pub struct TransparentStruct {
    payload: GenericTransparentStruct<()>,
    _zst1: [u8; 0],
    _zst2: (),
    _zst3: PhantomData<String>,
}

#[ffi_export]
impl TransparentStruct {
    pub fn new(payload: GenericTransparentStruct<()>) -> Self {
        Self {
            payload,
            _zst1: [],
            _zst2: (),
            _zst3: PhantomData,
        }
    }

    pub fn with_payload(mut self, payload: GenericTransparentStruct<()>) -> Self {
        self.payload = payload;
        self
    }

    pub fn payload(&self) -> &GenericTransparentStruct<()> {
        &self.payload
    }

    pub fn payload_mut(&mut self) -> &mut GenericTransparentStruct<()> {
        &mut self.payload
    }
}

#[ffi_export]
pub fn self_to_self(value: TransparentStruct) -> TransparentStruct {
    value
}

#[ffi_export]
pub fn vec_to_vec(value: Vec<TransparentStruct>) -> Vec<TransparentStruct> {
    value
}

#[ffi_export]
pub fn slice_to_slice(value: &[TransparentStruct]) -> &[TransparentStruct] {
    value
}

#[test]
#[webassembly_test::webassembly_test]
fn transparent_self_to_self() {
    let transparent_struct = TransparentStruct::new(GenericTransparentStruct::new(42));
    // NOTE: recursively traversing transparent structs
    let mut output: MaybeUninit<u64> = MaybeUninit::new(0);

    unsafe {
        assert_eq!(
            FfiReturn::Ok,
            __self_to_self(transparent_struct.into_ffi(&mut ()), output.as_mut_ptr())
        );
        assert_eq!(
            Ok(transparent_struct),
            TransparentStruct::try_from_ffi(output.assume_init(), &mut ())
        );
    }
}

#[test]
#[webassembly_test::webassembly_test]
fn transparent_vec_to_vec() {
    let transparent_struct_vec = vec![
        TransparentStruct::new(GenericTransparentStruct::new(1)),
        TransparentStruct::new(GenericTransparentStruct::new(2)),
        TransparentStruct::new(GenericTransparentStruct::new(3)),
    ];

    let mut store = Default::default();
    let mut output = MaybeUninit::new(OutBoxedSlice::from_raw_parts(core::ptr::null_mut(), 0));

    unsafe {
        assert_eq!(
            FfiReturn::Ok,
            __vec_to_vec(
                transparent_struct_vec.clone().into_ffi(&mut store),
                output.as_mut_ptr()
            )
        );

        let output = output.assume_init();
        assert_eq!(output.len(), 3);
        let vec = Vec::<TransparentStruct>::try_read_out(output).expect("Valid");
        assert_eq!(transparent_struct_vec, vec);
    }
}

#[test]
#[webassembly_test::webassembly_test]
// False positive
#[allow(clippy::let_unit_value)]
fn transparent_slice_to_slice() {
    let transparent_struct_slice = [
        TransparentStruct::new(GenericTransparentStruct::new(1)),
        TransparentStruct::new(GenericTransparentStruct::new(2)),
        TransparentStruct::new(GenericTransparentStruct::new(3)),
    ];
    let mut output = MaybeUninit::new(SliceRef::from_raw_parts(core::ptr::null(), 0));

    unsafe {
        assert_eq!(
            FfiReturn::Ok,
            __slice_to_slice(
                transparent_struct_slice.as_slice().into_ffi(&mut ()),
                output.as_mut_ptr()
            )
        );

        let mut store = ();
        let output: &[TransparentStruct] =
            FfiConvert::try_from_ffi(output.assume_init(), &mut store).expect("Invalid output");
        assert_eq!(output, transparent_struct_slice);
    }
}

#[test]
#[webassembly_test::webassembly_test]
fn transparent_method_consume() {
    let mut transparent_struct = TransparentStruct::new(GenericTransparentStruct::new(42));
    let payload = GenericTransparentStruct::new(24);

    let mut output: MaybeUninit<u64> = MaybeUninit::new(0);

    unsafe {
        assert_eq!(
            FfiReturn::Ok,
            TransparentStruct__with_payload(
                transparent_struct.into_ffi(&mut ()),
                payload.into_ffi(&mut ()),
                output.as_mut_ptr()
            )
        );
        transparent_struct =
            TransparentStruct::try_from_ffi(output.assume_init(), &mut ()).expect("valid");

        assert_eq!(transparent_struct.payload, payload);
    }
}

#[test]
#[webassembly_test::webassembly_test]
fn transparent_method_borrow() {
    let transparent_struct = TransparentStruct::new(GenericTransparentStruct::new(42));
    let mut output = MaybeUninit::new(core::ptr::null());

    unsafe {
        assert_eq!(
            FfiReturn::Ok,
            TransparentStruct__payload(
                (&transparent_struct).into_ffi(&mut ()),
                output.as_mut_ptr()
            )
        );
        assert_eq!(
            Ok(&transparent_struct.payload),
            <&GenericTransparentStruct<_>>::try_from_ffi(output.assume_init(), &mut ())
        );
    }
}

#[test]
fn transparent_method_borrow_mut() {
    let mut transparent_struct = TransparentStruct::new(GenericTransparentStruct::new(42));
    let mut output = MaybeUninit::new(core::ptr::null_mut());

    unsafe {
        assert_eq!(
            FfiReturn::Ok,
            TransparentStruct__payload_mut(
                (&mut transparent_struct).into_ffi(&mut ()),
                output.as_mut_ptr()
            )
        );
        assert_eq!(
            Ok(&mut transparent_struct.payload),
            <&mut GenericTransparentStruct<_>>::try_from_ffi(output.assume_init(), &mut ())
        );
    }
}