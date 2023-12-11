use nih_plug::prelude::{Param, ParamPtr};
use std::any::Any;
use vizia::{
  prelude::{ActionModifiers, LayoutModifiers, Context, EmitContext, LensExt, Units::{Stretch, Pixels}, Weight},
  state::{Binding, Data, Lens},
  views::{Checkbox, Label, VStack}, handle::Handle, modifiers::TextModifiers,
};

pub struct ParamCheckbox {}

impl ParamCheckbox {
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
    P: Param<Plain = bool>,
    F: 'static + Fn(&<L as Lens>::Target) -> &P + Copy + Send + Sync,
    M: Any + Send,
    C: 'static + Fn(ParamPtr, f32) -> M + Copy + Send + Sync,
  {
    VStack::new(cx, |cx| {
      Binding::new(cx, lens, move |cx, params| {
        Label::new(cx, unsafe { param_ptr.name() })
          .font_size(13.0)
          .font_weight(Weight::SEMIBOLD)
          .text_wrap(true)
          .child_space(Stretch(1.0));
        
        Checkbox::new(cx, {
          params.map(move |params| params_to_param(params).modulated_plain_value())
        })
        .on_press(move |cx| {
          let current_normalized_value = params
            .map(move |params| params_to_param(params).modulated_normalized_value())
            .get(cx);

          cx.emit(on_change(param_ptr, 1. - current_normalized_value));
        });
      });
    })
    .child_top(Pixels(4.0))
    .child_left(Stretch(1.0))
    .child_right(Stretch(1.0))
    .row_between(Pixels(8.0))
  }
}
