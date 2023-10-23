use pyo3::prelude::*;
use pyxel::{
    Effect, Note, SharedSound as PyxelSharedSound, Sound as PyxelSound, Speed, Tone, Volume,
};

macro_rules! define_private_methods_for_list {
    ($type: ty, $elems: ident) => {
        fn new(pyxel_sound: PyxelSharedSound) -> Self {
            Self { pyxel_sound }
        }

        fn list(&self) -> &[$type] {
            unsafe { &*(&self.pyxel_sound.lock().$elems as *const Vec<$type>) }
        }

        fn list_mut(&mut self) -> &mut Vec<$type> {
            unsafe { &mut *(&mut self.pyxel_sound.lock().$elems as *mut Vec<$type>) }
        }
    };
}

#[pyclass]
#[derive(Clone)]
pub struct Notes {
    pyxel_sound: PyxelSharedSound,
}

impl Notes {
    define_private_methods_for_list!(Note, notes);
}

#[pymethods]
impl Notes {
    fn __len__(&self) -> PyResult<usize> {
        impl_len_method_for_list!(self)
    }

    fn __getitem__(&self, index: isize) -> PyResult<Note> {
        impl_getitem_method_for_list!(self, index)
    }

    fn __setitem__(&mut self, index: isize, value: Note) -> PyResult<()> {
        impl_setitem_method_for_list!(self, index, value)
    }

    pub fn from_list(&mut self, lst: Vec<Note>) -> PyResult<()> {
        impl_from_list_method_for_list!(self, lst)
    }

    pub fn to_list(&self) -> PyResult<Vec<Note>> {
        impl_to_list_method_for_list!(self)
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Tones {
    pyxel_sound: PyxelSharedSound,
}

impl Tones {
    define_private_methods_for_list!(Tone, tones);
}

#[pymethods]
impl Tones {
    fn __len__(&self) -> PyResult<usize> {
        impl_len_method_for_list!(self)
    }

    fn __getitem__(&self, index: isize) -> PyResult<u8> {
        //impl_getitem_method_for_list!(self, index)
        if index < self.list().len() as isize {
            Ok(self.list()[index as usize].clone() as u8)
        } else {
            Err(pyo3::exceptions::PyIndexError::new_err(
                "list index out of range",
            ))
        }
    }

    fn __setitem__(&mut self, index: isize, value: u8) -> PyResult<()> {
        if index < self.list_mut().len() as isize {
            self.list_mut()[index as usize] = match value {
                0 => Tone::Triangle,
                1 => Tone::Square,
                2 => Tone::Pulse,
                3 => Tone::Noise,
                4 => Tone::Sine,
                5 => Tone::Saw,
                _ => panic!(),
            };
            Ok(())
        } else {
            Err(pyo3::exceptions::PyIndexError::new_err(
                "list assignment index out of range",
            ))
        }
    }

    pub fn from_list(&mut self, lst: Vec<u8>) -> PyResult<()> {
        *self.list_mut() = lst
            .iter()
            .map(|&x| match x {
                0 => Tone::Triangle,
                1 => Tone::Square,
                2 => Tone::Pulse,
                3 => Tone::Noise,
                4 => Tone::Sine,
                5 => Tone::Saw,
                _ => panic!(),
            })
            .collect();
        Ok(())
    }

    pub fn to_list(&self) -> PyResult<Vec<u8>> {
        Ok(self.list().iter().map(|&x| x as u8).collect())
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Volumes {
    pyxel_sound: PyxelSharedSound,
}

impl Volumes {
    define_private_methods_for_list!(Volume, volumes);
}

#[pymethods]
impl Volumes {
    fn __len__(&self) -> PyResult<usize> {
        impl_len_method_for_list!(self)
    }

    fn __getitem__(&self, index: isize) -> PyResult<Volume> {
        impl_getitem_method_for_list!(self, index)
    }

    fn __setitem__(&mut self, index: isize, value: Volume) -> PyResult<()> {
        impl_setitem_method_for_list!(self, index, value)
    }

    pub fn from_list(&mut self, lst: Vec<Volume>) -> PyResult<()> {
        impl_from_list_method_for_list!(self, lst)
    }

    pub fn to_list(&self) -> PyResult<Vec<Volume>> {
        impl_to_list_method_for_list!(self)
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Effects {
    pyxel_sound: PyxelSharedSound,
}

impl Effects {
    define_private_methods_for_list!(Effect, effects);
}

#[pymethods]
impl Effects {
    fn __len__(&self) -> PyResult<usize> {
        impl_len_method_for_list!(self)
    }

    fn __getitem__(&self, index: isize) -> PyResult<Effect> {
        impl_getitem_method_for_list!(self, index)
    }

    fn __setitem__(&mut self, index: isize, value: Effect) -> PyResult<()> {
        impl_setitem_method_for_list!(self, index, value)
    }

    pub fn from_list(&mut self, lst: Vec<Effect>) -> PyResult<()> {
        impl_from_list_method_for_list!(self, lst)
    }

    pub fn to_list(&self) -> PyResult<Vec<Effect>> {
        impl_to_list_method_for_list!(self)
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Sound {
    pub pyxel_sound: PyxelSharedSound,
}

pub const fn wrap_pyxel_sound(pyxel_sound: PyxelSharedSound) -> Sound {
    Sound { pyxel_sound }
}

#[pymethods]
impl Sound {
    #[new]
    pub fn new() -> Self {
        wrap_pyxel_sound(PyxelSound::new())
    }

    #[getter]
    pub fn notes(&self) -> Notes {
        Notes::new(self.pyxel_sound.clone())
    }

    #[getter]
    pub fn tones(&self) -> Tones {
        Tones::new(self.pyxel_sound.clone())
    }

    #[getter]
    pub fn volumes(&self) -> Volumes {
        Volumes::new(self.pyxel_sound.clone())
    }

    #[getter]
    pub fn effects(&self) -> Effects {
        Effects::new(self.pyxel_sound.clone())
    }

    #[getter]
    pub fn get_speed(&self) -> Speed {
        self.pyxel_sound.lock().speed
    }

    #[setter]
    pub fn set_speed(&self, speed: Speed) {
        self.pyxel_sound.lock().speed = speed;
    }

    pub fn set(&self, notes: &str, tones: &str, volumes: &str, effects: &str, speed: Speed) {
        self.pyxel_sound
            .lock()
            .set(notes, tones, volumes, effects, speed);
    }

    pub fn set_notes(&self, notes: &str) {
        self.pyxel_sound.lock().set_notes(notes);
    }

    pub fn set_tones(&self, tones: &str) {
        self.pyxel_sound.lock().set_tones(tones);
    }

    pub fn set_volumes(&self, volumes: &str) {
        self.pyxel_sound.lock().set_volumes(volumes);
    }

    pub fn set_effects(&self, effects: &str) {
        self.pyxel_sound.lock().set_effects(effects);
    }
}

pub fn add_sound_class(m: &PyModule) -> PyResult<()> {
    m.add_class::<Notes>()?;
    m.add_class::<Tones>()?;
    m.add_class::<Volumes>()?;
    m.add_class::<Effects>()?;
    m.add_class::<Sound>()?;
    Ok(())
}
