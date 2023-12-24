use crate::space_echo_parameters::{BoolParam, Params};
use std::any::Any;
use vizia::{
  prelude::{ActionModifiers, Context, EmitContext, LensExt, LayoutModifiers, Units::{Stretch, Pixels}, Weight},
  state::{Data, Lens},
  views::{Checkbox, Label, VStack}, handle::Handle, modifiers::TextModifiers,
};

pub struct ParamCheckbox;

impl ParamCheckbox {
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
    <L as Lens>::Target: Data,
    F: 'static + Fn(&<L as Lens>::Target) -> &BoolParam + Copy + Send + Sync,
    M: Any + Send,
    C: 'static + Fn(bool) -> M + Copy + Send + Sync,
  {
    VStack::new(cx, |cx| {
      Label::new(cx, name)
        .font_size(13.0)
        .font_weight(Weight::SEMIBOLD)
        .text_wrap(true)
        .child_space(Stretch(1.0));

      Checkbox::new(
        cx,
        lens.map(move |p| params_to_param(p).get_value()),
      )
      .on_press(move |cx| {
        let is_checked = lens
          .map(move |params| params_to_param(params).get_value())
          .get(cx);
        cx.emit(on_change(!is_checked));
      });
    })
    .child_top(Pixels(4.0))
    .child_left(Stretch(1.0))
    .child_right(Stretch(1.0))
    .row_between(Pixels(8.0))
  }
}
