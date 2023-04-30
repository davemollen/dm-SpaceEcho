use crate::space_echo_parameters::{IntParam, Params};
use std::any::Any;
use vizia::{
  prelude::{Context, EmitContext, LayoutModifiers, LensExt, StyleModifiers, Units::Pixels},
  state::{Binding, Data, Lens},
  views::{HStack, Label, RadioButton, VStack},
};

pub struct ParamRadioButton;

impl ParamRadioButton {
  pub fn new<L, F, M, C>(
    cx: &mut Context,
    lens: L,
    param: &IntParam,
    params_to_param: F,
    on_change: C,
  ) where
    L: 'static + Lens + Copy + Send + Sync,
    <L as Lens>::Source: 'static,
    <L as Lens>::Target: Data,
    F: 'static + Fn(&<L as Lens>::Target) -> &IntParam + Copy + Send + Sync,
    M: Any + Send,
    C: 'static + Fn(i32) -> M + Copy + Send + Sync,
  {
    let param_name = param.name;

    Label::new(cx, param_name);

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
            Label::new(cx, &names[i as usize]).describing(format!("{param_name}_{i}"));
          })
          .col_between(Pixels(8.0));
        }
      });
    })
  }
}
