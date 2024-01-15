use crate::space_echo_parameters::{IntParam, Params};
use std::any::Any;
use vizia::{
  prelude::{Context, EmitContext, LayoutModifiers, LensExt, StyleModifiers, Units::{Pixels, Stretch}},
  views::{HStack, Label, RadioButton, VStack}, modifiers::TextModifiers,
  view::Handle, 
  binding::Lens, 
  style::FontWeightKeyword, layout::Units::Auto
};

pub struct ParamRadioButton;

impl ParamRadioButton {
  pub fn new<'a, L, F, M, C>(
    cx: &'a mut Context,
    name: &'a str,
    lens: L,
    params_to_param: F,
    on_change: C,
  ) -> Handle<'a, VStack> 
  where
    L: 'static + Lens + Copy + Send + Sync,
    <L as Lens>::Source: 'static,
    F: 'static + Fn(&<L as Lens>::Target) -> &IntParam + Copy + Send + Sync,
    M: Any + Send,
    C: 'static + Fn(i32) -> M + Copy + Send + Sync,
  {
    VStack::new(cx, |cx| {
      Label::new(cx, name)
      .font_size(13.0)
      .font_weight(FontWeightKeyword::SemiBold)
      .text_wrap(false)
      .child_space(Stretch(1.0));
    
    
      VStack::new(cx, move |cx| {
        let names = lens
          .map(move |params| params_to_param(params).get_options())
          .get(cx);

        for i in 0..names.len() as i32 {
          HStack::new(cx, |cx| {
            RadioButton::new(
              cx,
              lens.map(move |p| params_to_param(p).get_value() == i),
            )
            .on_select(move |cx| {
              cx.emit(on_change(i))
            })
            .id(format!("{name}_{i}"));

            Label::new(cx, &names[i as usize])
              .font_size(12.0)
              .describing(format!("{name}_{i}"));
          })
          .size(Auto)
          .col_between(Pixels(8.0))
          .child_space(Pixels(4.0));
        }
      }).size(Auto);
    })
    .size(Auto)
    .child_space(Stretch(1.0))
    .row_between(Pixels(6.0))
  }
}
