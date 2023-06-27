use pyo3::{pyclass, pymethods};

use crate::datatype::noir_type::PyNoirIter;

#[pyclass]
#[derive(Clone)]
pub struct PySource{
    pub iter: PyNoirIter,
}

#[pymethods]
impl PySource{

    #[new]
    pub fn new(iter: PyNoirIter) -> Self {
        Self{iter}
    }

    /*
    #[new]
    pub fn new(iter: PyNoirIter) -> Self {
        let mut binding = REGISTRY.lock().unwrap();
        if !binding.contains_key::<PyNoirHandle<IteratorSource<NoirType, PyNoirIter>>>() {
            binding.insert::<PyNoirHandle<IteratorSource<NoirType, PyNoirIter>>>(IndexMap::with_capacity(1));
        }
        let map: &mut IndexMap<usize, IteratorSource<NoirType, PyNoirIter>> = binding.get_mut::<PyNoirHandle<IteratorSource<NoirType, PyNoirIter>>>().unwrap();
        let mut idx = IDX.lock().unwrap();
        *idx += 1;
        map.insert(*idx, IteratorSource::new(iter));
        Self(PyNoirHandle{idx: *idx, _marker: PhantomData})
    }

    pub fn description(&mut self) -> String {
        let id = self.0.idx;
        let mut binding = REGISTRY.lock().unwrap();
        let map: &mut IndexMap<usize, IteratorSource<NoirType, PyNoirIter>> = binding.get_mut::<PyNoirHandle<IteratorSource<NoirType, PyNoirIter>>>().unwrap();
        let source = map.get(&id).unwrap();
        return "Iterator Source starting with max parallelism of: ".to_string() + source.max_parallelism().unwrap().to_string().as_str();
    }
    */
}