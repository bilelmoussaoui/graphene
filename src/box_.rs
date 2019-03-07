use Box;
use Point3D;
use Vec3;
use ffi;
use glib::translate::*;

impl Box {
    pub fn get_vertices(&self) -> [Vec3; 8] {
        unsafe {
            let mut out: [ffi::graphene_vec3_t; 8] = std::mem::uninitialized();
            ffi::graphene_box_get_vertices(self.to_glib_none().0, &mut out as *mut _);

            let mut res: [Vec3; 8] = std::mem::uninitialized();
            for i in 0..8 {
                let t = from_glib_none(&out[i] as *const _);
                std::ptr::copy_nonoverlapping(&t as *const _, &mut res[i] as *mut _, 1);
                std::mem::forget(t);
            }
            res
        }
    }

    pub fn init_from_points(&mut self, points: &[&Point3D]) {
        let vec: Vec<_> = points
            .iter()
            .map(|e| unsafe { *e.to_glib_none().0 })
            .collect();
        let n = vec.len() as u32;

        unsafe {
            ffi::graphene_box_init_from_points(self.to_glib_none_mut().0, n, vec.as_ptr());
        }
    }

    pub fn init_from_vectors(&mut self, vectors: &[&Vec3]) {
        let vec: Vec<_> = vectors
            .iter()
            .map(|e| unsafe { *e.to_glib_none().0 })
            .collect();
        let n = vec.len() as u32;

        unsafe {
            ffi::graphene_box_init_from_vectors(self.to_glib_none_mut().0, n, vec.as_ptr());
        }
    }

    pub fn new(min: Option<&Point3D>, max: Option<&Point3D>) -> Box {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_box_alloc();
            ffi::graphene_box_init(alloc, min.to_glib_none().0, max.to_glib_none().0);
            from_glib_full(alloc)
        }
    }

    pub fn new_from_box(src: &Box) -> Box {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_box_alloc();
            ffi::graphene_box_init_from_box(alloc, src.to_glib_none().0);
            from_glib_full(alloc)
        }
    }

    pub fn new_from_points(&mut self, points: &[&Point3D]) -> Box {
        assert_initialized_main_thread!();

        let vec: Vec<_> = points
            .iter()
            .map(|e| unsafe { *e.to_glib_none().0 })
            .collect();
        let n = vec.len() as u32;

        unsafe {
            let alloc = ffi::graphene_box_alloc();
            ffi::graphene_box_init_from_points(alloc, n, vec.as_ptr());
            from_glib_full(alloc)
        }
    }

    pub fn new_from_vec3(min: Option<&Vec3>, max: Option<&Vec3>) -> Box {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_box_alloc();
            ffi::graphene_box_init_from_vec3(alloc, min.to_glib_none().0, max.to_glib_none().0);
            from_glib_full(alloc)
        }
    }

    pub fn new_from_vectors(vectors: &[&Vec3]) -> Box {
        assert_initialized_main_thread!();

        let vec: Vec<_> = vectors
            .iter()
            .map(|e| unsafe { *e.to_glib_none().0 })
            .collect();
        let n = vec.len() as u32;

        unsafe {
            let alloc = ffi::graphene_box_alloc();
            ffi::graphene_box_init_from_vectors(alloc, n, vec.as_ptr());
            from_glib_full(alloc)
        }
    }
}