use crate::space_echo_parameters::{IntParam, Params};
use std::any::Any;
use vizia::{
  prelude::{Context, EmitContext, LayoutModifiers, LensExt, StyleModifiers, Units::{Pixels, Stretch}, Weight},
  state::{Binding, Data, Lens},
  views::{HStack, Label, RadioButton, VStack}, handle::Handle, modifiers::TextModifiers,
};

pub struct ParamRadioButton;

impl ParamRadioButton {
  pub fn new<'a, L, F, M, C>(
    cx: &'a mut Context,
    lens: L,
    param: &IntParam,
    params_to_param: F,
    on_change: C,
  ) -> Handle<'a, VStack> 
  where
    L: 'static + Lens + Copy + Send + Sync,
    <L as Lens>::Source: 'static,
    <L as Lens>::Target: Data,
    F: 'static + Fn(&<L as Lens>::Target) -> &IntParam + Copy + Send + Sync,
    M: Any + Send,
    C: 'static + Fn(i32) -> M + Copy + Send + Sync,
  {
    let param_name = param.name;

    VStack::new(cx, |cx| {
      Label::new(cx, param.name)
        .font_size(13.0)
        .font_weight(Weight::SEMIBOLD)
        .text_wrap(true)
        .child_space(Stretch(1.0));
  
      Binding::new(cx, lens, move |cx, params| {
        let names = params
          .map(move |params| params_to_param(params).get_options())
          .get(cx);
  
        VStack::new(cx, move |cx| {
          for i in 0..names.len() as i32 {
            HStack::new(cx, |cx| {
              RadioButton::new(
                cx,
                params.map(move |params| params_to_param(params).get_value() == i),
              )
              .on_select(move |cx| cx.emit(on_change(i)))
              .id(format!("{param_name}_{i}"));
              Label::new(cx, &names[i as usize])
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
    .row_between(Pixels(4.0))
  }
}
