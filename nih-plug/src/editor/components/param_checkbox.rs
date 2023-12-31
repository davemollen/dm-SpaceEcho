use nih_plug::prelude::{Param, ParamPtr};
use std::any::Any;
use nih_plug_vizia::vizia::{
  prelude::{ActionModifiers, LayoutModifiers, Context, EmitContext, LensExt, Units::{Stretch, Pixels}, Weight},
  state::{Data, Lens},
  views::{Checkbox, Label, VStack}, handle::Handle, modifiers::TextModifiers,
};

pub struct ParamCheckbox {}

impl ParamCheckbox {
  pub fn new<'a, L, P, F, M, C>(
    cx: &'a mut Context,
    name: &'a str,
    lens: L,
    param_ptr: ParamPtr,
    params_to_param: F,
    on_change: C,
  ) -> Handle<'a, VStack> 
  where
    L: 'static + Lens + Copy + Send + Sync,
    <L as Lens>::Source: 'static,
    <L as Lens>::Target: Data,
    P: Param<Plain = bool>,
    F: 'static + Fn(&<L as Lens>::Target) -> &P + Copy + Send + Sync,
    M: Any + Send,
    C: 'static + Fn(ParamPtr, f32) -> M + Copy + Send + Sync,
  {
    VStack::new(cx, |cx| {
      Label::new(cx, name)
        .font_size(13.0)
        .font_weight(Weight::SEMIBOLD)
        .text_wrap(true)
        .child_space(Stretch(1.0));
      
      Checkbox::new(
        cx, lens.map(move |p: &<L as Lens>::Target| params_to_param(p).modulated_plain_value())
      )
      .on_press(move |cx| {
        let current_normalized_value = lens
          .map(move |params| params_to_param(params).modulated_normalized_value())
          .get(cx);

        cx.emit(on_change(param_ptr, 1. - current_normalized_value));
      });
    })
    .child_top(Pixels(4.0))
    .child_left(Stretch(1.0))
    .child_right(Stretch(1.0))
    .row_between(Pixels(8.0))
  }
}
