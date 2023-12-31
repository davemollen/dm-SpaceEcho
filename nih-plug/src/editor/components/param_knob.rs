use nih_plug::prelude::{Param, ParamPtr};
use std::any::Any;
use nih_plug_vizia::vizia::{
  prelude::{ActionModifiers, LayoutModifiers, Context, EmitContext, LensExt, StyleModifiers, Units::{Stretch, Pixels}, Weight},
  state::{Data, Lens},
  views::{Knob, Label, TextEvent, Textbox, VStack}, handle::Handle, modifiers::TextModifiers,
};

pub struct ParamKnob {}

impl ParamKnob {
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
    P: Param,
    F: 'static + Fn(&<L as Lens>::Target) -> &P + Copy + Send + Sync,
    M: Any + Send,
    C: 'static + Fn(ParamPtr, f32) -> M + Copy + Send + Sync,
  {
    VStack::new(cx, |cx| {
      Label::new(cx, name)
        .font_size(13.0)
        .font_weight(Weight::SEMIBOLD)
        .text_wrap(false)
        .child_space(Stretch(1.0));

      Knob::new(
        cx,
        lens.map(move |p| params_to_param(p).default_normalized_value()),
        lens.map(move |p| {
          params_to_param(p).preview_normalized(params_to_param(p).modulated_plain_value())
        }),
        false,
      )
      .on_changing(move |cx, val| {
        cx.emit(on_change(param_ptr, val));
      });

      Textbox::new(
        cx,
        lens.map(move |p| {
          params_to_param(p)
            .normalized_value_to_string(params_to_param(p).modulated_normalized_value(), true)
        }),
      )
      .on_mouse_down(|cx, _| {
        cx.emit(TextEvent::StartEdit);
        cx.emit(TextEvent::ResetText("".to_string()));
      })
      .on_submit(move |cx, text, success| {
        cx.emit(TextEvent::EndEdit);

        if success {
          let val =
            lens.map(move |p| {
              params_to_param(p).string_to_normalized_value(&text)
          }).get(cx).unwrap();
          
          cx.emit(on_change(param_ptr, val));
        }
      })
      .class("align_center");
    })
    .child_space(Stretch(1.0))
    .row_between(Pixels(4.0))
  }
}
