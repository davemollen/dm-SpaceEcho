use nih_plug::prelude::{Param, ParamPtr};
use nih_plug_vizia::vizia::{
  binding::Lens,
  layout::Units::Auto,
  modifiers::{LayoutModifiers, StyleModifiers, TextModifiers},
  prelude::{Context, EmitContext, LensExt, Units::Stretch},
  style::FontWeightKeyword,
  view::Handle,
  views::{Button, Label},
};
use std::any::Any;

pub struct ParamToggleButton;

impl ParamToggleButton {
  pub fn new<'a, L, P, F, M, C>(
    cx: &'a mut Context,
    name: &'a str,
    lens: L,
    param_ptr: ParamPtr,
    params_to_param: F,
    on_change: C,
  ) -> Handle<'a, Button>
  where
    L: 'static + Lens + Copy + Send + Sync,
    <L as Lens>::Source: 'static,
    P: Param<Plain = bool>,
    F: 'static + Fn(&<L as Lens>::Target) -> &P + Copy + Send + Sync,
    M: Any + Send,
    C: 'static + Fn(ParamPtr, f32) -> M + Copy + Send + Sync,
  {
    Button::new(
      cx,
      move |cx| {
        let current_normalized_value = lens
          .map(move |params| params_to_param(params).modulated_normalized_value())
          .get(cx);

        cx.emit(on_change(param_ptr, 1. - current_normalized_value));
      },
      |cx| {
        Label::new(cx, name)
          .font_size(13.0)
          .font_weight(FontWeightKeyword::SemiBold)
          .text_wrap(true)
      },
    )
    .toggle_class(
      "active",
      lens.map(move |params| params_to_param(params).modulated_plain_value()),
    )
    .size(Auto)
    .space(Stretch(1.0))
  }
}
