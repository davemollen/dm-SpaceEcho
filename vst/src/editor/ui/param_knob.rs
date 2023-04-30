use crate::space_echo_parameters::{FloatParam, Params};
use std::any::Any;
use vizia::{
  prelude::{ActionModifiers, Context, EmitContext, LensExt, StyleModifiers},
  state::{Binding, Data, Lens},
  views::{Knob, Label, TextEvent, Textbox},
};

pub struct ParamKnob;

impl ParamKnob {
  pub fn new<L, F, M, C>(
    cx: &mut Context,
    lens: L,
    param: &FloatParam,
    params_to_param: F,
    on_change: C,
  ) where
    L: 'static + Lens + Copy + Send + Sync,
    <L as Lens>::Source: 'static,
    <L as Lens>::Target: Data,
    F: 'static + Fn(&<L as Lens>::Target) -> &FloatParam + Copy + Send + Sync,
    M: Any + Send,
    C: 'static + Fn(f32) -> M + Copy + Send + Sync,
  {
    Label::new(cx, param.name);

    Binding::new(cx, lens, move |cx, params| {
      Knob::new(
        cx,
        params.map(move |params| params_to_param(params).get_default_normalized_value()),
        params.map(move |params| params_to_param(params).get_normalized_value()),
        false,
      )
      .on_changing(move |cx, val| {
        cx.emit(on_change(val));
      });

      Textbox::new(
        cx,
        params.map(move |params| params_to_param(params).get_display_value(true)),
      )
      .on_mouse_down(|cx, _| {
        cx.emit(TextEvent::StartEdit);
        cx.emit(TextEvent::ResetText("".to_string()));
      })
      .on_submit(move |cx, text, success| {
        if success {
          let normalized_value =
            params.map(move |params| params_to_param(params).string_to_normalized_value(&text));
          match normalized_value.get(cx) {
            Some(val) => cx.emit(on_change(val)),
            _ => (),
          };
        }
      })
      .class("align_center");
    })
  }
}
