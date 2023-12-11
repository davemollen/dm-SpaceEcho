use nih_plug::prelude::{Param, ParamPtr};
use std::any::Any;
use vizia::{
  prelude::{ActionModifiers, LayoutModifiers, Context, EmitContext, LensExt, StyleModifiers, Units::{Stretch, Pixels}, Weight},
  state::{Binding, Data, Lens},
  views::{Knob, Label, TextEvent, Textbox, VStack}, handle::Handle, modifiers::TextModifiers,
};

pub struct ParamKnob {}

impl ParamKnob {
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
        .text_wrap(false)
        .child_space(Stretch(1.0));

      Binding::new(cx, lens, move |cx, params| {
        Knob::new(
          cx,
          params.map(move |params| params_to_param(params).default_normalized_value()),
          params.map(move |params| {
            params_to_param(params)
              .preview_normalized(params_to_param(params).modulated_plain_value())
          }),
          false,
        )
        .on_changing(move |cx, val| {
          cx.emit(on_change(param_ptr, val));
        });

        Textbox::new(
          cx,
          params.map(move |params| {
            params_to_param(params)
              .normalized_value_to_string(params_to_param(params).modulated_normalized_value(), true)
          }),
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
              Some(val) => cx.emit(on_change(param_ptr, val)),
              _ => (),
            };
          }
        })
        .class("align_center");
      })
    })
    .child_space(Stretch(1.0))
    .row_between(Pixels(4.0))
  }
}
