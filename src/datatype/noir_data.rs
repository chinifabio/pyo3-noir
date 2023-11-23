use noir::data_type::NoirData;
use pyo3::{
    pymethods, types::PyList, AsPyPointer, FromPyObject, IntoPy, PyAny, PyObject, PyResult, Python,
    ToPyObject,
};

use super::noir_type::PyNoirType;

#[repr(transparent)]
#[derive(Debug)]
pub struct PyNoirData(pub NoirData);

#[pymethods]
impl PyNoirData {
    #[new]
    pub fn new(columns: Vec<PyNoirType>) -> Self {
        if columns.is_empty() {
            Self(NoirData::new_empty())
        } else {
            Self(NoirData::new(columns.into_iter().map(|x| x.0).collect()))
        }
    }

    fn __repr__(&self) -> String {
        format!("NoirData: {}", self.0)
    }

    fn __str__(&self) -> String {
        format!("{}", self.0)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}


/**
 * -------------------------------------------------------------------
 * Implementations copied from pyo3 derive macros.
 */


impl pyo3::PyClass for PyNoirData {
    type Frozen = pyo3::pyclass::boolean_struct::False;
}

impl pyo3::impl_::pyclass::PyClassImpl for PyNoirData {
    const IS_BASETYPE: bool = false;
    const IS_SUBCLASS: bool = false;
    const IS_MAPPING: bool = false;
    const IS_SEQUENCE: bool = false;
    type Layout = pyo3::PyCell<Self>;
    type BaseType = pyo3::PyAny;
    type ThreadChecker = pyo3::impl_::pyclass::ThreadCheckerStub<PyNoirData>;
    type Inventory = Pyo3MethodsInventoryForPyNoirData;
    type PyClassMutability =  <<pyo3::PyAny as pyo3::impl_::pyclass::PyClassBaseType> ::PyClassMutability as pyo3::impl_::pycell::PyClassMutability> ::MutableChild;
    type Dict = pyo3::impl_::pyclass::PyClassDummySlot;
    type WeakRef = pyo3::impl_::pyclass::PyClassDummySlot;
    type BaseNativeType = pyo3::PyAny;
    fn items_iter() -> pyo3::impl_::pyclass::PyClassItemsIter {
        use pyo3::impl_::pyclass::*;
        let _collector = PyClassImplCollector::<Self>::new();
        static INTRINSIC_ITEMS: PyClassItems = PyClassItems {
            methods: &[],
            slots: &[],
        };
        PyClassItemsIter::new(
            &INTRINSIC_ITEMS,
            ::std::boxed::Box::new(::std::iter::Iterator::map(
                pyo3::inventory::iter::<<Self as pyo3::impl_::pyclass::PyClassImpl>::Inventory>(),
                pyo3::impl_::pyclass::PyClassInventory::items,
            )),
        )
    }
    fn doc(py: pyo3::Python<'_>) -> pyo3::PyResult<&'static ::std::ffi::CStr> {
        use pyo3::impl_::pyclass::*;
        static DOC: pyo3::once_cell::GILOnceCell<::std::borrow::Cow<'static, ::std::ffi::CStr>> =
            pyo3::once_cell::GILOnceCell::new();
        DOC.get_or_try_init(py, || {
            let collector = PyClassImplCollector::<Self>::new();
            build_pyclass_doc(
                <PyNoirData as pyo3::PyTypeInfo>::NAME,
                "\0",
                ::std::option::Option::None.or_else(|| collector.new_text_signature()),
            )
        })
        .map(::std::ops::Deref::deref)
    }
    fn lazy_type_object() -> &'static pyo3::impl_::pyclass::LazyTypeObject<Self> {
        use pyo3::impl_::pyclass::LazyTypeObject;
        static TYPE_OBJECT: LazyTypeObject<PyNoirData> = LazyTypeObject::new();
        &TYPE_OBJECT
    }

    fn dict_offset() -> Option<pyo3::ffi::Py_ssize_t> {
        None
    }

    fn weaklist_offset() -> Option<pyo3::ffi::Py_ssize_t> {
        None
    }
}

#[doc(hidden)]
pub struct Pyo3MethodsInventoryForPyNoirData {
    items: pyo3::impl_::pyclass::PyClassItems,
}
impl Pyo3MethodsInventoryForPyNoirData {
    pub const fn new(items: pyo3::impl_::pyclass::PyClassItems) -> Self {
        Self { items }
    }
}
impl pyo3::impl_::pyclass::PyClassInventory for Pyo3MethodsInventoryForPyNoirData {
    fn items(&self) -> &pyo3::impl_::pyclass::PyClassItems {
        &self.items
    }
}
pyo3::inventory::collect!(Pyo3MethodsInventoryForPyNoirData);

unsafe impl pyo3::type_object::PyTypeInfo for PyNoirData {
    type AsRefTarget = pyo3::PyCell<Self>;
    const NAME: &'static str = "PyNoirData";
    const MODULE: ::std::option::Option<&'static str> = ::core::option::Option::None;
    #[inline]
    fn type_object_raw(py: pyo3::Python<'_>) -> *mut pyo3::ffi::PyTypeObject {
        <PyNoirData as pyo3::impl_::pyclass::PyClassImpl>::lazy_type_object()
            .get_or_init(py)
            .as_type_ptr()
    }

    fn type_object(py: Python<'_>) -> &pyo3::types::PyType {
        unsafe { py.from_borrowed_ptr(Self::type_object_raw(py) as _) }
    }

    fn is_type_of(object: &PyAny) -> bool {
        unsafe {
            pyo3::ffi::PyObject_TypeCheck(object.as_ptr(), Self::type_object_raw(object.py())) != 0
        }
    }

    fn is_exact_type_of(object: &PyAny) -> bool {
        unsafe { pyo3::ffi::Py_TYPE(object.as_ptr()) == Self::type_object_raw(object.py()) }
    }
}


/**
 * ---------------------------------------------------------------------
 * Custom implementations for sending and receiving data from Python.
 */

impl IntoPy<PyObject> for PyNoirData {
    fn into_py(self, py: Python) -> PyObject {
        match self.0 {
            NoirData::NoirType(a) => PyNoirType(a).into_py(py),
            NoirData::Row(a) => PyList::new(py, a.into_iter().map(PyNoirType)).into(),
        }
    }
}

impl ToPyObject for PyNoirData {
    fn to_object(&self, py: Python) -> PyObject {
        match &self.0 {
            NoirData::NoirType(a) => PyNoirType(*a).into_py(py),
            NoirData::Row(a) => PyList::new(py, a.iter().map(|x| PyNoirType(*x))).into(),
        }
    }
}

impl FromPyObject<'_> for PyNoirData {
    fn extract(ob: &'_ PyAny) -> PyResult<Self> {
        let data = ob.extract::<Vec<PyNoirType>>();
        if data.is_err() {
            let data = ob.extract::<PyNoirType>()?;
            Ok(PyNoirData(noir::data_type::NoirData::NoirType(data.0)))
        } else {
            let data = data?;
            let row = Vec::from_iter(data.into_iter().map(|x| x.0));
            Ok(PyNoirData(noir::data_type::NoirData::Row(row)))
        }
    }
}
