use crate::utils::get_fl_name;
use proc_macro::TokenStream;
use quote::*;
use syn::*;

pub fn impl_group_trait(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let name_str = get_fl_name(name.to_string());

    let begin = Ident::new(format!("{}_{}", name_str, "begin").as_str(), name.span());
    let end = Ident::new(format!("{}_{}", name_str, "end").as_str(), name.span());
    let clear = Ident::new(format!("{}_{}", name_str, "clear").as_str(), name.span());
    let children = Ident::new(format!("{}_{}", name_str, "children").as_str(), name.span());
    let child = Ident::new(format!("{}_{}", name_str, "child").as_str(), name.span());
    let find = Ident::new(format!("{}_{}", name_str, "find").as_str(), name.span());
    let add = Ident::new(format!("{}_{}", name_str, "add").as_str(), name.span());
    let insert = Ident::new(format!("{}_{}", name_str, "insert").as_str(), name.span());
    let remove = Ident::new(format!("{}_{}", name_str, "remove").as_str(), name.span());
    let remove_by_index = Ident::new(
        format!("{}_{}", name_str, "remove_by_index").as_str(),
        name.span(),
    );
    let resizable = Ident::new(
        format!("{}_{}", name_str, "resizable").as_str(),
        name.span(),
    );
    let clip_children = Ident::new(
        format!("{}_{}", name_str, "clip_children").as_str(),
        name.span(),
    );
    let set_clip_children = Ident::new(
        format!("{}_{}", name_str, "set_clip_children").as_str(),
        name.span(),
    );
    let init_sizes = Ident::new(
        format!("{}_{}", name_str, "init_sizes").as_str(),
        name.span(),
    );
    let update_child = Ident::new(
        format!("{}_{}", name_str, "update_child").as_str(),
        name.span(),
    );
    let draw_child = Ident::new(
        format!("{}_{}", name_str, "draw_child").as_str(),
        name.span(),
    );
    let draw_children = Ident::new(
        format!("{}_{}", name_str, "draw_children").as_str(),
        name.span(),
    );
    let draw_outside_label = Ident::new(
        format!("{}_{}", name_str, "draw_outside_label").as_str(),
        name.span(),
    );

    let gen = quote! {
        impl IntoIterator for #name {
            type Item = Widget;
            type IntoIter = std::vec::IntoIter<Self::Item>;

            fn into_iter(self) -> Self::IntoIter {
                let mut v: Vec<Widget> = vec![];
                for i in 0..self.children() {
                    v.push(self.child(i).unwrap());
                }
                v.into_iter()
            }
        }

        unsafe impl GroupExt for #name {
            fn begin(&self) {
                assert!(!self.was_deleted());
                unsafe { #begin(self.inner) }
            }

            fn end(&self) {
                assert!(!self.was_deleted());
                unsafe { #end(self.inner) }
            }

            fn clear(&mut self) {
                assert!(!self.was_deleted());
                unsafe { #clear(self.inner); }
            }

            unsafe fn unsafe_clear(&mut self) {
                assert!(!self.was_deleted());
                unsafe { #clear(self.inner); }
            }

            fn children(&self) -> i32 {
                unsafe {
                    assert!(!self.was_deleted());
                    #children(self.inner) as i32
                }
            }

            fn child(&self, idx: i32) -> Option<Widget> {
                unsafe {
                    assert!(!self.was_deleted());
                    if idx >= self.children() || idx < 0 {
                        return None;
                    }
                    let child_widget = #child(self.inner, idx as i32);
                    if child_widget.is_null() {
                        None
                    } else {
                        Some(Widget::from_widget_ptr(child_widget as *mut fltk_sys::widget::Fl_Widget))
                    }
                }
            }

            fn find<W: WidgetExt>(&self, widget: &W) -> i32 {
                unsafe {
                    assert!(!self.was_deleted());
                    assert!(!widget.was_deleted());
                    #find(self.inner, widget.as_widget_ptr() as *mut _) as i32
                }
            }

            fn add<W: WidgetExt>(&mut self, widget: &W) {
                unsafe {
                    assert!(!self.was_deleted());
                    assert!(!widget.was_deleted());
                    #add(self.inner, widget.as_widget_ptr() as *mut _)
                }
            }

            fn insert<W: WidgetExt>(&mut self, widget: &W, index: i32) {
                unsafe {
                    assert!(!self.was_deleted());
                    assert!(!widget.was_deleted());
                    #insert(self.inner, widget.as_widget_ptr() as *mut _, index as i32)
                }
            }

            fn remove<W: WidgetExt>(&mut self, widget: &W) {
                unsafe {
                    assert!(!self.was_deleted());
                    assert!(!widget.was_deleted());
                    #remove(self.inner, widget.as_widget_ptr() as *mut _)
                }
            }

            fn remove_by_index(&mut self, idx: i32) {
                unsafe {
                    assert!(!self.was_deleted());
                    assert!(idx < self.children());
                    #remove_by_index(self.inner, idx as i32);
                }
            }

            fn resizable<W: WidgetExt>(&self, widget: &W) {
                unsafe {
                    assert!(!self.was_deleted());
                    assert!(!widget.was_deleted());
                    #resizable(self.inner, widget.as_widget_ptr() as *mut _)
                }
            }

            fn make_resizable(&mut self, val: bool) {
                assert!(!self.was_deleted());
                let ptr = if val { self.inner } else { std::ptr::null_mut() };
                unsafe {
                    #resizable(self.inner, ptr as *mut _)
                }
            }

            fn add_resizable<W: WidgetExt>(&mut self, widget: &W) {
                self.resizable(widget);
                self.add(widget);
            }

            fn set_clip_children(&mut self, flag: bool) {
                assert!(!self.was_deleted());
                unsafe {
                    #set_clip_children(self.inner, flag as i32)
                }
            }

            fn clip_children(&mut self) -> bool {
                assert!(!self.was_deleted());
                unsafe {
                    #clip_children(self.inner) != 0
                }
            }

            fn draw_child<W: WidgetExt>(&self, w: &mut W) {
                assert!(!self.was_deleted());
                assert!(!w.was_deleted());
                unsafe {
                    crate::app::open_display();
                    #draw_child(self.inner as _, w.as_widget_ptr() as _)
                }
            }

            fn update_child<W: WidgetExt>(&self, w: &mut W) {
                assert!(!self.was_deleted());
                assert!(!w.was_deleted());
                unsafe {
                        crate::app::open_display();
                    #update_child(self.inner as _, w.as_widget_ptr() as _)
                }
            }

            fn draw_outside_label<W: WidgetExt>(&self, w: &mut W) {
                assert!(!self.was_deleted());
                assert!(!w.was_deleted());
                unsafe {
                    crate::app::open_display();
                    #draw_outside_label(self.inner as _, w.as_widget_ptr() as _)
                }
            }

            fn draw_children(&mut self) {
                assert!(!self.was_deleted());
                unsafe {
                    crate::app::open_display();
                    #draw_children(self.inner as _)
                }
            }

            fn init_sizes(&mut self) {
                unsafe {
                    assert!(!self.was_deleted());
                    #init_sizes(self.inner)
                }
            }

            fn bounds(&self) -> Vec<(i32, i32, i32, i32)> {
                let children = self.children();
                let mut vec = vec![];
                for i in 0..children {
                    let child = self.child(i).unwrap();
                    let x = child.x();
                    let y = child.y();
                    let r = child.w() + x;
                    let b = child.h() + y;
                    vec.push((x, y, r, b));
                }
                vec
            }

            unsafe fn into_group(&self) -> crate::group::Group {
                crate::group::Group::from_widget_ptr(self.inner as _)
            }
        }
    };
    gen.into()
}
