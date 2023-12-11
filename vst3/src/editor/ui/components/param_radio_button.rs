use std::any::Any;
use nih_plug::params::{internals::ParamPtr, Param};
use vizia::{
  prelude::{Context, TextModifiers, LayoutModifiers, LensExt, StyleModifiers, Units::{Pixels, Stretch}, Weight},
  state::{Binding, Data, Lens},
  views::{HStack, Label, RadioButton, VStack}, handle::Handle, context::EmitContext,
};

pub struct ParamRadioButton;

impl ParamRadioButton {
  pub fn new<'a, L, P, F, M, C>(
    cx: &'a mut Context,
    lens: L,
    param_ptr: ParamPtr,
    params_to_param: F,
    on_change: C,
    variants: &'static [&'static str],
  ) -> Handle<'a, VStack> 
  where
    L: 'static + Lens + Copy + Send + Sync,
    <L as Lens>::Source: 'static,
    <L as Lens>::Target: Data,
    P: Param,
    F: 'static + Fn(&<L as Lens>::Target) -> &P + Copy + Send + Sync,
    M: Any + Send,
    C: 'static + Fn(ParamPtr, f32) -> M + Copy + Send + Sync,
  {
    VStack::new(cx, |cx| {
      Label::new(cx, unsafe { param_ptr.name() })
        .font_size(13.0)
        .font_weight(Weight::SEMIBOLD)
        .text_wrap(true)
        .child_space(Stretch(1.0));
  
      Binding::new(cx, lens, move |cx, params| {
        VStack::new(cx, |cx| {
          let param_name = unsafe { param_ptr.name() };

          variants.iter().for_each(|variant| {
            let variant = *variant;

            HStack::new(cx, |cx| {
              let normalized_option = unsafe { param_ptr.string_to_normalized_value(variant)}.unwrap();

              RadioButton::new(
                cx,
                params.map(move |params| params_to_param(params).modulated_normalized_value() == normalized_option)
              )
              .on_select(move |cx| {
                cx.emit(on_change(param_ptr, normalized_option))
              })
              .id(format!("{param_name}_{variant}"));

              Label::new(cx, variant)
                .font_size(12.0)
                .describing(format!("{param_name}_{variant}"));
              })
              .col_between(Pixels(8.0))
              .child_space(Pixels(2.0));
          });
        });
      })
    })
    .child_space(Stretch(1.0))
    .child_top(Pixels(4.0))
    .row_between(Pixels(4.0))
  }
}
