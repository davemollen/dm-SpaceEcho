use crate::space_echo_parameters::{BoolParam, Params};
use std::any::Any;
use vizia::{
  prelude::{ActionModifiers, Context, EmitContext, LensExt},
  state::{Binding, Data, Lens},
  views::{Checkbox, Label},
};

pub struct ParamCheckbox;

impl ParamCheckbox {
  pub fn new<L, F, M, C>(
    cx: &mut Context,
    lens: L,
    param: &BoolParam,
    params_to_param: F,
    on_change: C,
  ) where
    L: 'static + Lens + Copy + Send + Sync,
    <L as Lens>::Source: 'static,
    <L as Lens>::Target: Data,
    F: 'static + Fn(&<L as Lens>::Target) -> &BoolParam + Copy + Send + Sync,
    M: Any + Send,
    C: 'static + Fn(bool) -> M + Copy + Send + Sync,
  {
    let name = param.name;

    Binding::new(cx, lens, move |cx, params| {
      Label::new(cx, name);
      Checkbox::new(
        cx,
        params.map(move |params| params_to_param(params).get_value()),
      )
      .on_press(move |cx| {
        let is_checked = params
          .map(move |params| params_to_param(params).get_value())
          .get(cx);
        cx.emit(on_change(!is_checked));
      });
    });
  }
}
