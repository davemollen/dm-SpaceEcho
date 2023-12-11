use std::any::Any;
use nih_plug::params::{internals::ParamPtr, Param};
use vizia::{
  prelude::{Context, TextModifiers, LayoutModifiers, LensExt, StyleModifiers, Units::{Pixels, Stretch}, Weight},
  state::{Binding, Data, Lens},
  views::{HStack, Label, RadioButton, VStack}, handle::Handle, context::EmitContext,
};

pub struct ParamRadioButton;

impl ParamRadioButton {
  pub fn new<L, P, F, M, C>(
    cx: &mut Context,
    lens: L,
    param_ptr: ParamPtr,
    params_to_param: F,
    on_change: C,
  ) -> Handle<'_, VStack> 
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
        let param_name = unsafe { param_ptr.name() };
        let step_count = unsafe { param_ptr.step_count() }.unwrap();

        VStack::new(cx, |cx| {
          for i in 0..step_count+1 {
            let normalized_option = i as f32 / step_count as f32;
            let option = unsafe { param_ptr.normalized_value_to_string(normalized_option, false) };

            HStack::new(cx, |cx| {
              RadioButton::new(
                cx,
                params.map(move |params| params_to_param(params).modulated_normalized_value() == normalized_option)
              )
              .on_select(move |cx| {
                cx.emit(on_change(param_ptr, normalized_option))
              })
              .id(format!("{param_name}_{i}"));

              Label::new(cx, &option)
                .font_size(12.0)
                .describing(format!("{param_name}_{i}"));
            })
            .col_between(Pixels(8.0))
            .child_space(Pixels(2.0));
          }
        });
      })
    })
    .child_space(Stretch(1.0))
    .child_top(Pixels(4.0))
    .row_between(Pixels(4.0))
  }
}
