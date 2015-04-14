use std::marker::{PhantomData, Send};
use std::ffi::CString;
use std::fmt;
use std::ptr;
use std::mem;
use context::Context;
use gccjit_sys;
use object::{ToObject, Object};
use object;
use function::Function;
use function;
use location::Location;
use location;
use rvalue::RValue;
use rvalue;
use lvalue::LValue;
use lvalue;

#[repr(C)]
pub enum BinaryOp {
    Plus,
    Minus,
    Mult,
    Divide,
    Modulo,
    BitwiseAnd,
    BitwiseXor,
    BitwiseOr,
    LogicalAnd,
    LogicalOr,
    LShift,
    RShift
}

#[repr(C)]
pub enum UnaryOp {
    Minus,
    BitwiseNegate,
    LogicalNegate,
    Abs
}

#[derive(Copy, Clone)]
pub struct Block<'ctx> {
    marker: PhantomData<&'ctx Context<'ctx>>,
    ptr: *mut gccjit_sys::gcc_jit_block
}

impl<'ctx> ToObject<'ctx> for Block<'ctx> {
    fn to_object(&self) -> Object<'ctx> {
        unsafe {
            let ptr = gccjit_sys::gcc_jit_block_as_object(self.ptr);
            object::from_ptr(ptr)
        }
    }
}

impl<'ctx> fmt::Debug for Block<'ctx> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let obj = self.to_object();
        obj.fmt(fmt)
    }
}

impl<'ctx> Block<'ctx> {
    pub fn get_function(&self) -> Function<'ctx> {
        unsafe {
            let ptr = gccjit_sys::gcc_jit_block_get_function(self.ptr);
            function::from_ptr(ptr)
        }
    }

    pub fn add_eval(&self, loc: Option<Location<'ctx>>, rvalue: RValue<'ctx>) {
        unsafe {
            let loc_ptr = match loc {
                Some(loc) => location::get_ptr(&loc),
                None => ptr::null_mut()
            };
            gccjit_sys::gcc_jit_block_add_eval(self.ptr,
                                               loc_ptr,
                                               rvalue::get_ptr(&rvalue));
        }
    }

    pub fn add_assignment(&self,
                          loc: Option<Location<'ctx>>,
                          lvalue: LValue<'ctx>,
                          rvalue: RValue<'ctx>) {
        unsafe {
            let loc_ptr = match loc {
                Some(loc) => location::get_ptr(&loc),
                None => ptr::null_mut()
            };
            gccjit_sys::gcc_jit_block_add_assignment(self.ptr,
                                                     loc_ptr,
                                                     lvalue::get_ptr(&lvalue),
                                                     rvalue::get_ptr(&rvalue));
        }
    }

    pub fn add_assignment_op(&self,
                             loc: Option<Location<'ctx>>,
                             lvalue: LValue<'ctx>,
                             op: BinaryOp,
                             rvalue: RValue<'ctx>) {
        unsafe {
            let loc_ptr = match loc {
                Some(loc) => location::get_ptr(&loc),
                None => ptr::null_mut()
            };
            gccjit_sys::gcc_jit_block_add_assignment_op(self.ptr,
                                                        loc_ptr,
                                                        lvalue::get_ptr(&lvalue),
                                                        mem::transmute(op),
                                                        rvalue::get_ptr(&rvalue));
        }
    }

    pub fn add_comment(&self,
                       loc: Option<Location<'ctx>>,
                       message: &str) {
        unsafe {
            let loc_ptr = match loc {
                Some(loc) => location::get_ptr(&loc),
                None => ptr::null_mut()
            };
            let cstr = CString::new(message).unwrap();
            gccjit_sys::gcc_jit_block_add_comment(self.ptr,
                                                  loc_ptr,
                                                  cstr.as_ptr());
        }
    }

    pub fn end_with_conditional(&self,
                                loc: Option<Location<'ctx>>,
                                cond: RValue<'ctx>,
                                on_true: Block<'ctx>,
                                on_false: Block<'ctx>) {
        unimplemented!()
    }

    pub fn end_with_jump(&self,
                         loc: Option<Location<'ctx>>,
                         target: Block<'ctx>) {
        unimplemented!()
    }

    pub fn end_with_return(&self,
                           loc: Option<Location<'ctx>>,
                           ret: RValue<'ctx>) {
        let loc_ptr = match loc {
            Some(loc) => unsafe { location::get_ptr(&loc) },
            None => ptr::null_mut()
        };
        unsafe {
            gccjit_sys::gcc_jit_block_end_with_return(self.ptr,
                                                      loc_ptr,
                                                      rvalue::get_ptr(&ret));
        }
    }

    pub fn end_with_void_return(&self, loc: Option<Location<'ctx>>) {
        unimplemented!()
    }
}

impl<'ctx> !Send for Block<'ctx> {}

pub unsafe fn from_ptr<'ctx>(ptr: *mut gccjit_sys::gcc_jit_block) -> Block<'ctx> {
    Block {
        marker: PhantomData,
        ptr: ptr
    }
}

pub unsafe fn get_ptr<'ctx>(loc: &Block<'ctx>) -> *mut gccjit_sys::gcc_jit_block {
    loc.ptr
}
