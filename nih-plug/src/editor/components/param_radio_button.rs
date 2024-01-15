use std::any::Any;
use nih_plug::params::{internals::ParamPtr, Param};
use nih_plug_vizia::vizia::{
  prelude::{Context, TextModifiers, LayoutModifiers, LensExt, StyleModifiers, Units::{Pixels, Stretch}},
  views::{HStack, Label, RadioButton, VStack}, context::EmitContext, style::FontWeightKeyword, 
  view::Handle, 
  binding::Lens, layout::Units::Auto
};

pub struct ParamRadioButton;

impl ParamRadioButton {
  pub fn new<'a, L, P, F, M, C>(
    cx: &'a mut Context,
    name: &'a str,
    lens: L,
    param_ptr: ParamPtr,
    params_to_param: F,
    on_change: C,
    variants: &'static [&'static str],
  ) -> Handle<'a, VStack> 
  where
    L: 'static + Lens + Copy + Send + Sync,
    <L as Lens>::Source: 'static,
    P: Param,
    F: 'static + Fn(&<L as Lens>::Target) -> &P + Copy + Send + Sync,
    M: Any + Send,
    C: 'static + Fn(ParamPtr, f32) -> M + Copy + Send + Sync,
  {
    VStack::new(cx, |cx| {
      Label::new(cx, name)
        .font_size(13.0)
        .font_weight(FontWeightKeyword::SemiBold)
        .text_wrap(false)
        .child_space(Stretch(1.0));
      
      VStack::new(cx, |cx| {
        variants.iter().for_each(|variant| {
          let variant = *variant;

          HStack::new(cx, |cx| {
            let normalized_option = lens.map(move |p| {
              params_to_param(p).string_to_normalized_value(variant)
            }).get(cx).unwrap();

            RadioButton::new(
              cx,
              lens.map(move |p| params_to_param(p).modulated_normalized_value() == normalized_option)
            )
            .on_select(move |cx| {
              cx.emit(on_change(param_ptr, normalized_option))
            })
            .id(format!("{name}_{variant}"));

            Label::new(cx, variant)
              .font_size(12.0)
              .describing(format!("{name}_{variant}"));
            })
            .size(Auto)
            .col_between(Pixels(8.0))
            .child_space(Pixels(4.0));
        });
      }).size(Auto);
    })
    .size(Auto)
    .child_space(Stretch(1.0))
    .row_between(Pixels(6.0))
  }
}
