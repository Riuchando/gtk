// This file was generated by gir (3c73dd9) from gir-files (71d73f0)
// DO NOT EDIT

use Orientable;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;
use pango;

glib_wrapper! {
    pub struct ProgressBar(Object<ffi::GtkProgressBar>): Widget, Orientable;

    match fn {
        get_type => || ffi::gtk_progress_bar_get_type(),
    }
}

impl ProgressBar {
    pub fn new() -> ProgressBar {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_progress_bar_new()).downcast_unchecked()
        }
    }
}

pub trait ProgressBarExt {
    fn get_ellipsize(&self) -> pango::EllipsizeMode;

    fn get_fraction(&self) -> f64;

    fn get_inverted(&self) -> bool;

    fn get_pulse_step(&self) -> f64;

    fn get_show_text(&self) -> bool;

    fn get_text(&self) -> Option<String>;

    fn pulse(&self);

    fn set_ellipsize(&self, mode: pango::EllipsizeMode);

    fn set_fraction(&self, fraction: f64);

    fn set_inverted(&self, inverted: bool);

    fn set_pulse_step(&self, fraction: f64);

    fn set_show_text(&self, show_text: bool);

    fn set_text<'a, P: Into<Option<&'a str>>>(&self, text: P);
}

impl<O: IsA<ProgressBar>> ProgressBarExt for O {
    fn get_ellipsize(&self) -> pango::EllipsizeMode {
        unsafe {
            from_glib(ffi::gtk_progress_bar_get_ellipsize(self.to_glib_none().0))
        }
    }

    fn get_fraction(&self) -> f64 {
        unsafe {
            ffi::gtk_progress_bar_get_fraction(self.to_glib_none().0)
        }
    }

    fn get_inverted(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_progress_bar_get_inverted(self.to_glib_none().0))
        }
    }

    fn get_pulse_step(&self) -> f64 {
        unsafe {
            ffi::gtk_progress_bar_get_pulse_step(self.to_glib_none().0)
        }
    }

    fn get_show_text(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_progress_bar_get_show_text(self.to_glib_none().0))
        }
    }

    fn get_text(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_progress_bar_get_text(self.to_glib_none().0))
        }
    }

    fn pulse(&self) {
        unsafe {
            ffi::gtk_progress_bar_pulse(self.to_glib_none().0);
        }
    }

    fn set_ellipsize(&self, mode: pango::EllipsizeMode) {
        unsafe {
            ffi::gtk_progress_bar_set_ellipsize(self.to_glib_none().0, mode.to_glib());
        }
    }

    fn set_fraction(&self, fraction: f64) {
        unsafe {
            ffi::gtk_progress_bar_set_fraction(self.to_glib_none().0, fraction);
        }
    }

    fn set_inverted(&self, inverted: bool) {
        unsafe {
            ffi::gtk_progress_bar_set_inverted(self.to_glib_none().0, inverted.to_glib());
        }
    }

    fn set_pulse_step(&self, fraction: f64) {
        unsafe {
            ffi::gtk_progress_bar_set_pulse_step(self.to_glib_none().0, fraction);
        }
    }

    fn set_show_text(&self, show_text: bool) {
        unsafe {
            ffi::gtk_progress_bar_set_show_text(self.to_glib_none().0, show_text.to_glib());
        }
    }

    fn set_text<'a, P: Into<Option<&'a str>>>(&self, text: P) {
        let text = text.into();
        let text = text.to_glib_none();
        unsafe {
            ffi::gtk_progress_bar_set_text(self.to_glib_none().0, text.0);
        }
    }
}
