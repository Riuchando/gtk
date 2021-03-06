// This file was generated by gir (3c73dd9) from gir-files (71d73f0)
// DO NOT EDIT

use Container;
use Widget;
use ffi;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct Fixed(Object<ffi::GtkFixed>): Container, Widget;

    match fn {
        get_type => || ffi::gtk_fixed_get_type(),
    }
}

impl Fixed {
    pub fn new() -> Fixed {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_fixed_new()).downcast_unchecked()
        }
    }
}

pub trait FixedExt {
    fn move_<P: IsA<Widget>>(&self, widget: &P, x: i32, y: i32);

    fn put<P: IsA<Widget>>(&self, widget: &P, x: i32, y: i32);

    fn get_child_x<T: IsA<Widget>>(&self, item: &T) -> i32;

    fn set_child_x<T: IsA<Widget>>(&self, item: &T, x: i32);

    fn get_child_y<T: IsA<Widget>>(&self, item: &T) -> i32;

    fn set_child_y<T: IsA<Widget>>(&self, item: &T, y: i32);
}

impl<O: IsA<Fixed> + IsA<Container>> FixedExt for O {
    fn move_<P: IsA<Widget>>(&self, widget: &P, x: i32, y: i32) {
        unsafe {
            ffi::gtk_fixed_move(self.to_glib_none().0, widget.to_glib_none().0, x, y);
        }
    }

    fn put<P: IsA<Widget>>(&self, widget: &P, x: i32, y: i32) {
        unsafe {
            ffi::gtk_fixed_put(self.to_glib_none().0, widget.to_glib_none().0, x, y);
        }
    }

    fn get_child_x<T: IsA<Widget>>(&self, item: &T) -> i32 {
        let mut value = Value::from(&0);
        unsafe {
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "x".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_child_x<T: IsA<Widget>>(&self, item: &T, x: i32) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "x".to_glib_none().0, Value::from(&x).to_glib_none().0);
        }
    }

    fn get_child_y<T: IsA<Widget>>(&self, item: &T) -> i32 {
        let mut value = Value::from(&0);
        unsafe {
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "y".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_child_y<T: IsA<Widget>>(&self, item: &T, y: i32) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "y".to_glib_none().0, Value::from(&y).to_glib_none().0);
        }
    }
}
