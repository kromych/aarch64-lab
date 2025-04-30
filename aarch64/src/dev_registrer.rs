//! Simple abstractions for memory-mapped device register(s) access.
//!
//! These lack RMW implementation and atomic access.

use core::marker::PhantomData;
use core::ops::Range;

/// Trait to describe the register access
pub trait DeviceRegisterSpec {
    /// The raw type used for memory representation
    type Raw: Copy + From<Self::Value>;
    /// The value type used in the API
    type Value: Copy + From<Self::Raw>;
    /// The register offset from the base address
    const OFFSET: usize;
}

/// A memory-mapped device register
///
/// The user must ensure that the base address and the offset are valid,
/// and that the memory is mapped as required for the device access.
pub struct DeviceRegister<S: DeviceRegisterSpec> {
    address: *mut S::Raw,
    _spec: PhantomData<S>,
}

impl<S: DeviceRegisterSpec> DeviceRegister<S> {
    /// Create a new MMIO register from a base address
    ///
    /// Caller must ensure:
    /// - The base address is valid and properly aligned,
    /// - The resulting address (base + OFFSET) points to valid memory,
    /// - The memory has the required access permissions and caching set.
    pub const fn new(base_address: usize) -> Self {
        Self {
            address: (base_address + S::OFFSET) as *mut S::Raw,
            _spec: PhantomData,
        }
    }

    /// Read the register value
    pub fn read(&self) -> S::Value {
        // SAFETY:
        // - The address was validated during creation,
        // - Volatile access ensures proper hardware interaction: no accesses
        //   will be elided or reordered by the compiler,
        // - The conversion from Raw to Value is guaranteed by the trait bounds
        unsafe { core::ptr::read_volatile(self.address).into() }
    }

    /// Write a value to the register
    pub fn write(&mut self, value: S::Value) {
        // SAFETY:
        // - The address was validated during creation
        // - Volatile access ensures proper hardware interaction: no accesses
        //   will be elided or reordered by the compiler,
        // - The conversion from Value to Raw is guaranteed by the trait bounds
        unsafe { core::ptr::write_volatile(self.address, value.into()) }
    }
}

/// Trait defining the specification for an array of device registers
pub trait DeviceRegisterArraySpec: DeviceRegisterSpec {
    /// The stride between consecutive registers in bytes
    const STRIDE: usize;
    /// The number of registers in the array
    const COUNT: usize;
}

/// An array of memory-mapped device registers
///
/// The user must ensure that the base address and the offset are valid,
/// and that the memory is mapped as required for the device access.
pub struct DeviceRegisterArray<S: DeviceRegisterArraySpec> {
    base_address: usize,
    _spec: PhantomData<S>,
}

impl<S: DeviceRegisterArraySpec> DeviceRegisterArray<S> {
    /// Create a new array of MMIO registers from a base address.
    ///
    /// The expectations are the same as for the `DeviceRegioster::new()`.
    pub const fn new(base_address: usize) -> Self {
        Self {
            base_address,
            _spec: PhantomData,
        }
    }

    /// Get a reference to a specific register in the array.
    pub fn index(&self, index: usize) -> DeviceRegister<S> {
        assert!(index < S::COUNT, "Register index out of bounds");

        DeviceRegister::<S>::new(self.base_address + S::OFFSET + index * S::STRIDE)
    }

    /// Iterate over all registers in the array.
    pub fn iter(&self) -> impl Iterator<Item = DeviceRegister<S>> + '_ {
        (0..S::COUNT).map(move |i| self.index(i))
    }

    /// Fill the range with some value.
    pub fn fill(&mut self, range: Range<usize>, value: S::Value) {
        self.iter()
            .skip(range.start)
            .take(range.len())
            .for_each(|mut r| r.write(value));
    }
}
