use std::marker::PhantomData;

use indexmap::IndexMap;
use noir::{data_type::NoirType, prelude::IteratorSource, Stream};
use pyo3::{pyclass, pymethods, exceptions::PyAttributeError, PyAny};

use crate::{datatype::noir_type::PyNoirIter, PyNoirHandle, REGISTRY};

#[pyclass]
pub struct PyStream(PyNoirHandle<Stream<NoirType, IteratorSource<NoirType, PyNoirIter>>>);

impl PyStream {
    pub fn new(stream: Stream<NoirType, IteratorSource<NoirType, PyNoirIter>>) -> Self {
        let mut binding = REGISTRY.lock().unwrap();
        if !binding
            .contains_key::<PyNoirHandle<Stream<NoirType, IteratorSource<NoirType, PyNoirIter>>>>()
        {
            binding.insert::<PyNoirHandle<Stream<NoirType, IteratorSource<NoirType, PyNoirIter>>>>(
                IndexMap::with_capacity(1),
            );
        }
        let map: &mut IndexMap<usize, Stream<NoirType, IteratorSource<NoirType, PyNoirIter>>> =
            binding
                .get_mut::<PyNoirHandle<Stream<NoirType, IteratorSource<NoirType, PyNoirIter>>>>()
                .unwrap();
        map.insert(0, stream);
        Self(PyNoirHandle {
            idx: 0,
            _marker: PhantomData,
        })
    }
}

#[pymethods]
impl PyStream {
    pub fn description(&self) -> String {
        "Stream of Iterator Source".to_string()
    }

    pub fn max(&mut self) {
        let id = self.0.idx;
        let mut binding = REGISTRY.lock().unwrap();
        let map_str: &mut IndexMap<usize, Stream<NoirType, IteratorSource<NoirType, PyNoirIter>>> =
            binding
                .get_mut::<PyNoirHandle<Stream<NoirType, IteratorSource<NoirType, PyNoirIter>>>>()
                .unwrap();
        let stream = map_str.get(&id).unwrap();
        //stream.max_noir(true);
    }


}
